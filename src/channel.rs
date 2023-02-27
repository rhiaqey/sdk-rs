// use serde::de::{Error, Unexpected, Visitor};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Channel {
    pub name: String,
    pub size: usize,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ChannelList {
    pub channels: Vec<Channel>,
}
