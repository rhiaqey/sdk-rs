use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
#[serde(untagged)]
pub enum MessageValue {
    Text(String),
    Json(serde_json::Value),
    Binary(Vec<u8>),
}

impl MessageValue {
    pub fn decode<T: DeserializeOwned>(&self) -> serde_json::Result<T> {
        match self {
            MessageValue::Text(text) => serde_json::from_str::<T>(text.as_str()),
            MessageValue::Json(value) => serde_json::from_value::<T>(value.clone()),
            MessageValue::Binary(data) => serde_json::from_slice::<T>(data),
        }
    }
}
