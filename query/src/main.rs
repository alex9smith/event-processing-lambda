use aws_sdk_dynamodb::{self, model::AttributeValue};
use common::{ServiceRecord, UserRecord};
use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use std::collections::HashMap;

fn get_value(map: &HashMap<String, AttributeValue>, key: &str) -> String {
    map.get(key)
        .expect(&format!("Missing {}", key).to_string())
        .as_s()
        .expect(&format!("Couldn't parse {}", key).to_string())
        .to_owned()
}

fn as_service_record(service: &AttributeValue) -> ServiceRecord {
    let service = match service.as_m() {
        Ok(s) => s.to_owned(),
        Err(_) => panic!("Couldn't parse response from DynamoDB"),
    };

    let service_id = get_value(&service, "service_id");
    let service_name = get_value(&service, "service_name");
    let last_accessed = get_value(&service, "last_accessed");

    ServiceRecord {
        service_id,
        service_name,
        last_accessed,
    }
}

async fn get_user_services(user_id: String) -> Result<UserRecord, Error> {
    let shared_config = aws_config::load_from_env().await;
    let client = aws_sdk_dynamodb::Client::new(&shared_config);

    let req = client
        .query()
        .table_name("user_services")
        .index_name("user_id")
        .expression_attribute_values(":user_id", AttributeValue::S(user_id.to_owned()))
        .key_condition_expression("user_id = :user_id");

    let res = req.send().await.expect("Error querying DynamoDB");

    // Querying a primary index, so there will only be one item
    let item = res.items.unwrap()[0].to_owned();
    let services: Vec<ServiceRecord> = item
        .get("services")
        .unwrap()
        .as_l()
        .expect("Couldn't parse response from DynamoDB")
        .iter()
        .map(|s| as_service_record(s))
        .collect();

    Ok(UserRecord { user_id, services })
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request

    let user_id = event
        .query_string_parameters()
        .first("user_id")
        .expect("missing user_id")
        .to_string()
        .to_owned();

    let user_services = get_user_services(user_id)
        .await
        .expect("could not search for user's services");

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(
            serde_json::to_string(&user_services)
                .expect("could not serialise response")
                .into(),
        )
        .map_err(Box::new)?;
    Ok(resp)
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
    use common::ToAttributeValue;

    #[test]
    fn test_as_service_record() {
        let input = ServiceRecord::default().to_attribute_value();
        let expected = ServiceRecord::default();

        assert_eq!(as_service_record(&input), expected)
    }
}
