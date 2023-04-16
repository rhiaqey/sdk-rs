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

    pub fn to_vec(&self) -> Option<Vec<u8>> {
        match self {
            MessageValue::Text(text) => Some(text.as_bytes().to_vec()),
            MessageValue::Json(value) => match serde_json::to_string(value) {
                Ok(data) => Some(data.as_bytes().to_vec()),
                Err(_) => None,
            }
            MessageValue::Binary(data) => Some(data.to_vec()),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use crate::message::MessageValue;

    #[derive(Serialize, Deserialize)]
    struct TestStruct {
        pub name: String
    }

    #[test]
    fn can_decode() {
        let data = TestStruct{ name: "test".to_string() };
        let msg = MessageValue::Json(json!(data));
        // println!("{:?}", msg.to_vec());
        let decoded = msg.decode::<TestStruct>();
        assert_eq!(decoded.is_ok(), true);
        assert_eq!(decoded.unwrap().name, "test".to_string());
    }

    #[test]
    fn can_convert_to_vec() {
        let result = Some(vec![123, 34, 110, 97, 109, 101, 34, 58, 34, 116, 101, 115, 116, 34, 125]);
        let data = TestStruct{ name: "test".to_string() };
        let msg = MessageValue::Json(json!(data));
        assert_eq!(msg.to_vec(), result);
    }

}
