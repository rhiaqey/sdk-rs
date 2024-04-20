use serde::{Deserialize, Serialize};

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
