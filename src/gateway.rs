use crate::{message::MessageValue, settings::Settings};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedReceiver;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GatewayMessage {
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

    // target specific user ids
    #[serde(rename = "uid", skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,

    // target specific client ids
    #[serde(rename = "cid", skip_serializing_if = "Option::is_none")]
    pub client_ids: Option<Vec<String>>,
}

pub type GatewayMessageReceiver =
    Result<UnboundedReceiver<GatewayMessage>, Box<dyn std::error::Error>>;

#[derive(Clone, Debug)]
pub struct GatewayConfig {
    pub id: Option<String>,
    pub name: Option<String>,
    pub namespace: Option<String>,
    pub port: u16,
    pub host: Option<String>,
}

impl Default for GatewayConfig {
    fn default() -> Self {
        GatewayConfig {
            id: None,
            name: None,
            namespace: None,
            port: 8080,
            host: None,
        }
    }
}

pub trait Gateway<S: Settings>: Default {
    fn setup(
        &mut self,
        config: GatewayConfig,
        settings: Option<S>,
    ) -> impl std::future::Future<Output = GatewayMessageReceiver> + Send;
    fn set_settings(&mut self, settings: S) -> impl std::future::Future<Output = ()> + Send;
    fn start(&mut self) -> impl std::future::Future<Output = ()> + Send;
    fn schema() -> serde_json::Value;
    fn metrics(&self) -> impl std::future::Future<Output = serde_json::Value> + Send;
    fn kind() -> String;
}
