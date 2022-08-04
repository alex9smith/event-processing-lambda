use aws_lambda_events::event::sqs::{SqsEvent, SqsMessage};
use aws_sdk_dynamodb;
use aws_sdk_dynamodb::model::AttributeValue::S;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};

mod models;
use models::{EventBody, ServiceRecord, ToAttributeValue, ToUserRecord, UserRecord};
/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/lambda-runtime/examples
/// - https://github.com/aws-samples/serverless-rust-demo/

async fn get_user_record(
    user_id: &String,
    client: &aws_sdk_dynamodb::Client,
) -> Result<Option<UserRecord>, Error> {
    let req = client
        .get_item()
        .table_name("user_services")
        .key("user_id", S(user_id.to_string()));
    let res = req.send().await?;
    let res = res.to_user_record();

    Ok(res)
}

async fn write_user_record(
    record: UserRecord,
    client: &aws_sdk_dynamodb::Client,
) -> Result<(), Error> {
    let req = client
        .put_item()
        .table_name("user_services")
        .item(record.user_id.to_owned(), record.to_attribute_value());
    req.send().await?;
    Ok(())
}

fn update_record(record: UserRecord, body: &EventBody) -> Result<UserRecord, Error> {
    let service = ServiceRecord {
        service_id: body.service_id.to_owned(),
        service_name: body.service_name.to_owned(),
        last_accessed: body.timestamp.to_owned(),
    };
    let mut services = record.services;
    services.retain(|s| s.service_id != service.service_id);
    services.push(service);

    Ok(UserRecord {
        user_id: record.user_id,
        services,
    })
}

async fn process_message(event: SqsMessage) -> Result<(), Error> {
    // deserialise message body
    let body: EventBody = {
        let body = event.body.as_ref().unwrap();
        serde_json::from_str(body.as_str())?
    };

    let shared_config = aws_config::load_from_env().await;
    let client = aws_sdk_dynamodb::Client::new(&shared_config);

    let record = match get_user_record(&body.user_id, &client).await.unwrap() {
        Some(record) => record,
        None => UserRecord::new(&body.user_id, vec![]),
    };
    let updated_record = update_record(record, &body).unwrap();
    write_user_record(updated_record, &client).await.unwrap();
    Ok(())
}

async fn function_handler(event: LambdaEvent<SqsEvent>) -> Result<(), Error> {
    // Lambdas listening to an SQS queue can be passed multiple messages in
    // the same event. Iterate over each of them
    for message in event.payload.records {
        match process_message(message).await {
            Ok(_) => {}
            Err(_) => {}
        };
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}

#[cfg(test)]
mod tests {

    use super::*;
    pub mod helpers;

    #[test]
    fn test_update_record_adds_new_service() {
        let record = UserRecord::new(
            "user_id",
            vec![ServiceRecord::new(
                "service_id",
                "service_name",
                "last_accessed",
            )],
        );

        let body = EventBody::new("user_id", "other_service_name", "other_service_id", "12345");
        let expected = update_record(record, &body).unwrap();

        assert_eq!(expected.services.len(), 2);
    }

    #[test]
    fn test_update_record_updates_timestamp() {
        let record = UserRecord::new(
            "user_id",
            vec![ServiceRecord::new(
                "service_id",
                "service_name",
                "last_accessed",
            )],
        );

        let body = EventBody::new("user_id", "service_name", "service_id", "12345");
        let expected = update_record(record, &body).unwrap();

        assert_eq!(expected.services.len(), 1);
        assert_eq!(expected.services[0].last_accessed, "12345".to_string());
    }
}
