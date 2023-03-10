use crate::message::MessageValue;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedReceiver;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GatewayMessage {
    #[serde(rename = "key")]
    pub key: String,

    #[serde(rename = "val")]
    pub value: MessageValue,

    // If timestamp is provided there will a check in timestamps. If latest entry in database is
    // older than the message then we do not store the new message
    #[serde(rename = "tms", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<u64>,

    // If tag is provided, there will be an extra comparison message comparison
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,

    // Extra grouping
    #[serde(rename = "cat", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,

    // Overrides the channel's size. Useful with the above extra grouping
    #[serde(rename = "siz", skip_serializing_if = "Option::is_none")]
    pub size: Option<usize>,

    // target specific client ids
    #[serde(rename = "cid", skip_serializing_if = "Option::is_none")]
    pub client_ids: Option<Vec<String>>,

    // target specific group ids
    #[serde(rename = "gid", skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,

    // target specific user ids
    #[serde(rename = "uid", skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

pub type GatewayMessageReceiver =
    Result<UnboundedReceiver<GatewayMessage>, Box<dyn std::error::Error>>;

#[async_trait]
pub trait Gateway<S> {
    fn setup(&mut self, settings: Option<S>) -> GatewayMessageReceiver;
    async fn set_settings(&mut self, settings: S);
    async fn start(&mut self);
    fn kind(&self) -> String;
}