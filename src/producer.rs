use crate::{message::MessageValue, settings::Settings};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedReceiver;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProducerMessage {
    #[serde(rename = "key")]
    pub key: String,

    #[serde(rename = "val")]
    pub value: MessageValue,

    // If timestamp is provided there will a check in timestamps.
    // If the latest entry in the database is older than the message,
    // then we do not store the new message
    #[serde(rename = "tms", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<u64>,

    // If the tag is provided, there will be an extra comparison message comparison
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,

    // Extra grouping
    #[serde(rename = "cat", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,

    // Overrides the channel's size. Useful with the above extra grouping
    #[serde(rename = "siz", skip_serializing_if = "Option::is_none")]
    pub size: Option<usize>,
}

pub type ProducerMessageReceiver =
    Result<UnboundedReceiver<ProducerMessage>, Box<dyn std::error::Error>>;

pub trait Producer<S: Settings> {
    fn setup(&mut self, settings: Option<S>) -> ProducerMessageReceiver;
    fn set_settings(&mut self, settings: S) -> impl std::future::Future<Output = ()> + Send;
    fn start(&mut self) -> impl std::future::Future<Output = ()> + Send;
    fn schema() -> impl std::future::Future<Output = serde_json::Value> + Send;
    fn metrics(&self) -> impl std::future::Future<Output = serde_json::Value> + Send;
    fn kind() -> String;
}
