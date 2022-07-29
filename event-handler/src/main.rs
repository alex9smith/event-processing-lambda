use serde::{Deserialize, Serialize};
use aws_lambda_events::event::sqs::SqsEvent;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};

#[derive(Deserialize, Serialize)]
struct EventBody {
  user_id: String,
  service_id: String,
  timestamp: String 
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/lambda-runtime/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(event: LambdaEvent<SqsEvent>) -> Result<String, Error> {
    // Extract some useful information from the request

    // deserialise message body
    let event_body = event.payload.records[0].body.as_ref().unwrap();
    let body: EventBody = serde_json::from_str(event_body.as_str())?;

    Ok(body.user_id)
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
  use std::collections::HashMap;

  use super::*;

  fn to_option_string(s: &str) -> Option<String> {
    Some(s.to_string())
  }

  #[tokio::test]
  async fn test_my_function_handler_returns_user_id() {

    let body = EventBody {
      user_id: "user_id".to_string(),
      service_id: "service_id".to_string(),
      timestamp: "1659082455".to_string()
    };

    let input = SqsEvent {
      records: vec![
        aws_lambda_events::event::sqs::SqsMessage {
          message_id: to_option_string("message_id"),
          receipt_handle: to_option_string("receipt_handle"),
          body: Some(serde_json::to_string(&body).unwrap()),
          md5_of_body: to_option_string("md5_of_body"),
          md5_of_message_attributes: to_option_string("md5_of_message_attributes"),
          attributes: HashMap::new(),
          message_attributes: HashMap::new(),
          event_source_arn: to_option_string("event_source_arn"),
          event_source: to_option_string("aws:sqs"),
          aws_region: to_option_string("eu-west-2") 
        }
      ]
    };
    let context = lambda_runtime::Context::default();

    let event = LambdaEvent::new(input, context);

    let response = function_handler(event).await.unwrap();

    assert_eq!(response, "user_id".to_string());
  }
}
