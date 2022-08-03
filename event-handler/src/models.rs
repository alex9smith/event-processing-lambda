use std::collections::hash_map::RandomState;
use std::collections::HashMap;

use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::{self, output::GetItemOutput};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct EventBody {
    pub user_id: String,
    pub service_id: String,
    pub timestamp: String,
}

impl EventBody {
    pub fn new(user_id: &str, service_id: &str, timestamp: &str) -> EventBody {
        EventBody {
            user_id: user_id.to_string(),
            service_id: service_id.to_string(),
            timestamp: timestamp.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct UserRecord {
    pub user_id: String,
    pub services: Vec<ServiceRecord>,
}

impl UserRecord {
    pub fn new(user_id: &str, services: Vec<ServiceRecord>) -> UserRecord {
        UserRecord {
            user_id: user_id.to_string(),
            services: services,
        }
    }
}

impl Default for UserRecord {
    fn default() -> Self {
        Self {
            user_id: "user_id".to_string(),
            services: vec![ServiceRecord::default()],
        }
    }
}

impl PartialEq for UserRecord {
    fn eq(&self, other: &Self) -> bool {
        let eq_ids = self.user_id == other.user_id;
        let eq_services = self
            .services
            .iter()
            .all(|item| other.services.contains(item));

        eq_ids && eq_services
    }
}

#[derive(PartialEq, Debug)]
pub struct ServiceRecord {
    pub service_id: String,
    pub service_name: String,
    pub last_accessed: String,
}

impl ServiceRecord {
    pub fn new(service_id: &str, service_name: &str, last_accessed: &str) -> ServiceRecord {
        ServiceRecord {
            service_id: service_id.to_string(),
            service_name: service_name.to_string(),
            last_accessed: last_accessed.to_string(),
        }
    }
}

impl Default for ServiceRecord {
    fn default() -> Self {
        Self {
            service_id: "service_id".to_string(),
            service_name: "service_name".to_string(),
            last_accessed: "last_accessed".to_string(),
        }
    }
}

pub fn to_option_string(s: &str) -> Option<String> {
    Some(s.to_string())
}

pub trait ToUserRecord {
    fn to_user_record(&self) -> UserRecord;
}

impl ToUserRecord for GetItemOutput {
    fn to_user_record(&self) -> UserRecord {
        fn get_string_attribute(
            map: &HashMap<String, AttributeValue, RandomState>,
            key: &str,
        ) -> String {
            map.get(&key.to_string())
                .unwrap()
                .to_owned()
                .as_s()
                .unwrap()
                .to_owned()
        }

        let item = self.item().unwrap().to_owned();
        let user_id = get_string_attribute(&item, "user_id");

        let services = item
            .get(&"services".to_string())
            .unwrap()
            .to_owned()
            .as_l()
            .unwrap()
            .to_owned();

        fn unwrap_service_attribute(attribute: &AttributeValue) -> ServiceRecord {
            let service = attribute.as_m().unwrap().to_owned();
            let service_id = get_string_attribute(&service, "service_id");
            let service_name = get_string_attribute(&service, "service_name");
            let last_accessed = get_string_attribute(&service, "last_accessed");

            ServiceRecord {
                service_id,
                service_name,
                last_accessed,
            }
        }

        let services = services
            .iter()
            .map(|s| unwrap_service_attribute(s))
            .collect::<Vec<_>>();

        UserRecord { user_id, services }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{ToUserRecord, UserRecord};
    use crate::tests::helpers;

    #[test]
    fn test_query_to_user_record() {
        let item = helpers::build_find_user_output();
        let expected_user_record = UserRecord::default();
        assert_eq!(item.to_user_record(), expected_user_record);
    }
}
