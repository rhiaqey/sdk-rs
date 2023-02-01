use std::sync::mpsc::Receiver;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use crate::message::MessageValue;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProducerMessage {
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
}

pub type ProducerMessageReceiver = Result<Receiver<ProducerMessage>, Box<dyn std::error::Error>>;

pub trait Producer<S> {
    fn setup(&mut self, settings: Option<S>) -> ProducerMessageReceiver;
    fn start(&self);
    fn kind() -> String;
}

#[async_trait]
pub trait AsyncProducer<S> {
    fn setup(&mut self, settings: Option<S>) -> ProducerMessageReceiver;
    async fn start(&self);
    fn kind() -> String;
}
