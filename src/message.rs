use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
#[serde(untagged)]
pub enum MessageValue {
    Text(String),
    Json(serde_json::Value),
    Binary(Vec<u8>),
}
