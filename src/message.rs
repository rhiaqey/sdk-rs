use std::str;
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

    pub fn as_str(&self) -> &str {
        match self {
            MessageValue::Text(text) => text.as_str(),
            MessageValue::Json(value) => value.as_str().unwrap_or(""),
            MessageValue::Binary(data) => str::from_utf8(data.as_slice()).unwrap_or("")
        }
    }
}
