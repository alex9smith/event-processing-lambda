use crate::models::{to_option_string, EventBody};
use aws_lambda_events::event::sqs::SqsMessage;
use aws_sdk_dynamodb::output::GetItemOutput;
use std::collections::HashMap;

pub fn build_sqs_message(
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
        aws_region: to_option_string(aws_region),
    }
}

pub fn build_get_item_output() -> GetItemOutput {
    todo!()
}
