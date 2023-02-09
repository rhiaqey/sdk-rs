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

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CreateChannelsRequest {
    #[serde(flatten)]
    pub channels: ChannelList
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AssignChannelsRequest {
    pub name: String,
    pub channels: Vec<String>
}

/*
pub struct ChannelListDeserializer;

impl ChannelList {
    pub fn is_empty(&self) -> bool { self.channels.len() == 0 }
    pub fn len(&self) -> usize {
        self.channels.len()
    }
}

impl<'de> Visitor<'de> for ChannelListDeserializer {
    type Value = ChannelList;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a valid json string representing an array of channels")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
    {
        let data: Result<Vec<Channel>, serde_json::Error> = serde_json::from_str(v);
        match data {
            Err(err) => Err(Error::invalid_type(
                Unexpected::Str(err.to_string().as_str()),
                &self,
            )),
            Ok(data) => Ok(ChannelList { channels: data }),
        }
    }
}

impl<'de> Deserialize<'de> for ChannelList {
    fn deserialize<D>(deserializer: D) -> Result<ChannelList, D::Error>
        where
            D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ChannelListDeserializer)
    }
}
*/
