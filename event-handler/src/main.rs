use serde::{Deserialize, Serialize};
use aws_lambda_events::event::sqs::{ SqsEvent, SqsMessage};
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

fn process_message(event: SqsMessage) -> Result<String, Error> {

  // deserialise message body
  let body: EventBody = {
    let body = event.body.as_ref().unwrap();
    serde_json::from_str(body.as_str())?
  };
  
  Ok(body.user_id)
}

async fn function_handler(event: LambdaEvent<SqsEvent>) -> Result<(), Error> {
  // Lambdas listening to an SQS queue can be passed multiple messages in
  // the same event. Iterate over each of them 
  for message in event.payload.records {
    match process_message(message) {
      Ok(_) => {},
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
  use std::collections::HashMap;

  use super::*;

  fn to_option_string(s: &str) -> Option<String> {
    Some(s.to_string())
  }

  fn build_sqs_message(
    message_id: &str,
    receipt_handle: &str,
    body: EventBody,
    md5_of_body: &str,
    md5_of_message_attributes: &str,
    event_source_arn: &str,
    event_source: &str,
    aws_region: &str,
  ) -> SqsMessage {
    SqsMessage {
      message_id: to_option_string(message_id),
      receipt_handle: to_option_string(receipt_handle),
      body: Some(serde_json::to_string(&body).unwrap()),
      md5_of_body: to_option_string(md5_of_body),
      md5_of_message_attributes: to_option_string(md5_of_message_attributes),
      attributes: HashMap::new(),
      message_attributes: HashMap::new(),
      event_source_arn: to_option_string(event_source_arn),
      event_source: to_option_string(event_source),
      aws_region: to_option_string(aws_region) 
    }
  }

  #[tokio::test]
  async fn test_process_message_returns_user_id() {

    let body = EventBody {
      user_id: "user_id".to_string(),
      service_id: "service_id".to_string(),
      timestamp: "1659082455".to_string()
    };

    let message = build_sqs_message(
      "message_id",
      "receipt_handle",
      body,
      "md5_of_body",
      "md5_of_message_attributes",
      "event_source_arn",
      "aws:sqs",
      "eu-west-2" 
    );

    let response = process_message(message).unwrap();

    assert_eq!(response, "user_id".to_string());
  }
}
