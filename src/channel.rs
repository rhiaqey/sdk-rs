use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SimpleChannel(String);

impl SimpleChannel {
    pub fn get_channel_with_category(&self) -> (String, Option<String>) {
        let parts: Vec<&str> = self.0.split('/').collect();
        match parts.len() {
            2 => (parts[0].to_string(), Some(parts[1].to_string())),
            _ => (parts[0].to_string(), None),
        }
    }
}

impl From<&str> for SimpleChannel {
    fn from(value: &str) -> Self {
        Self {
            0: value.to_string(),
        }
    }
}

impl Display for SimpleChannel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SimpleChannels(Vec<SimpleChannel>);

impl SimpleChannels {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get_channels_with_category(&self) -> Vec<(String, Option<String>)> {
        self.0
            .iter()
            .map(|x| x.get_channel_with_category())
            .collect()
    }
}

impl From<Vec<String>> for SimpleChannels {
    fn from(value: Vec<String>) -> Self {
        Self {
            0: value
                .iter()
                .map(|x| SimpleChannel { 0: x.clone() })
                .collect(),
        }
    }
}

impl From<Vec<&str>> for SimpleChannels {
    fn from(value: Vec<&str>) -> Self {
        Self {
            0: value.iter().map(|x| SimpleChannel::from(*x)).collect(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Channel {
    pub name: String,
    pub size: usize,
}

impl Channel {
    pub fn get_name(&self) -> &String {
        &self.name
    }
}

impl PartialEq<Channel> for Channel {
    fn eq(&self, other: &Channel) -> bool {
        self.name == other.name && self.size == other.size
    }
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ChannelList {
    pub channels: Vec<Channel>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn are_equal() {
        let ch1 = Channel {
            name: String::from("a"),
            size: 5,
        };
        let ch2 = Channel {
            name: String::from("a"),
            size: 5,
        };
        assert_eq!(ch1, ch2);
    }

    #[test]
    fn are_equal_more() {
        let ch1 = Channel {
            name: String::from("a"),
            size: 5,
        };
        let ch2 = Channel {
            name: String::from("a"),
            size: 5,
        };
        assert!(ch1.eq(&ch2));
    }

    #[test]
    fn are_equal_ref() {
        let ch1 = Channel {
            name: String::from("a"),
            size: 5,
        };
        let ch2 = Channel {
            name: String::from("a"),
            size: 5,
        };
        assert!(&ch1.eq(&ch2));
    }
}
