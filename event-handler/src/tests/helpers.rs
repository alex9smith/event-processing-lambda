use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::output::GetItemOutput;
use std::collections::HashMap;

pub fn build_find_user_output() -> GetItemOutput {
    GetItemOutput::builder()
        .item("user_id", AttributeValue::S("user_id".to_string()))
        .item(
            "services",
            AttributeValue::L(vec![AttributeValue::M(HashMap::from([
                (
                    "service_id".to_string(),
                    AttributeValue::S("service_id".to_string()),
                ),
                (
                    "service_name".to_string(),
                    AttributeValue::S("service_name".to_string()),
                ),
                (
                    "last_accessed".to_string(),
                    AttributeValue::S("last_accessed".to_string()),
                ),
            ]))]),
        )
        .build()
}
