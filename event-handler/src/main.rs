use crate::models::ToUserRecord;
use aws_lambda_events::event::sqs::{SqsEvent, SqsMessage};
use aws_sdk_dynamodb;
use aws_sdk_dynamodb::model::AttributeValue::S;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
mod models;

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/lambda-runtime/examples
/// - https://github.com/aws-samples/serverless-rust-demo/

async fn get_user_record(user_id: &String) -> Result<models::UserRecord, Error> {
    let shared_config = aws_config::load_from_env().await;
    let client = aws_sdk_dynamodb::Client::new(&shared_config);
    let req = client
        .get_item()
        .set_table_name(models::to_option_string("user_services"))
        .key("user_id", S(user_id.to_string()));
    let res = req.send().await?;
    let res = res.to_user_record();

    Ok(res)
}

fn write_user_record(record: models::UserRecord) -> Result<(), Error> {
    todo!()
}

fn update_record(
    record: models::UserRecord,
    body: &models::EventBody,
) -> Result<models::UserRecord, Error> {
    todo!()
}

async fn process_message(event: SqsMessage) -> Result<String, Error> {
    // deserialise message body
    let body: models::EventBody = {
        let body = event.body.as_ref().unwrap();
        serde_json::from_str(body.as_str())?
    };

    let record = get_user_record(&body.user_id).await.unwrap();
    let updated_record = update_record(record, &body).unwrap();
    write_user_record(updated_record).unwrap();
    Ok(body.user_id)
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

    #[tokio::test]
    async fn test_process_message_returns_user_id() {
        let body = models::EventBody::new("user_id", "service_id", "1659082455");

        let message = helpers::build_sqs_message(
            "message_id",
            "receipt_handle",
            body,
            "md5_of_body",
            "md5_of_message_attributes",
            "event_source_arn",
            "aws:sqs",
            "eu-west-2",
        );

        let response = process_message(message).await.unwrap();

        assert_eq!(response, "user_id".to_string());
    }
}
