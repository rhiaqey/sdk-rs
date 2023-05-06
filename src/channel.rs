use std::borrow::Cow;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Channel {
    pub name: Cow<'static, str>,
    pub size: usize,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ChannelList {
    pub channels: Vec<Channel>,
}
