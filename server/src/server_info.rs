use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use serde_json;
use uuid::Uuid;

#[derive(Default, Clone, Debug, Serialize)]
pub struct ServerInfo {
    pub url: String,
    pub name: String,
    pub icon: Icon,
    pub channel_list: Vec<Channel>
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub enum Icon {
    #[default]
    Default,
    Image(PathBuf),
    Svg(PathBuf),
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct Channel {
    pub name: String,
    pub log: Vec<UserMessage>
}

impl Channel {
    pub fn from_name(name: String) -> Self {
        Self {
            name,
            log: Vec::default()
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    username: String,
    icon: Icon,
}

impl Default for User {
    fn default() -> Self {
        Self {
            username: String::default(),
            icon: Icon::Default,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserMessage {
    channel: String,
    user: User,
    content: String,
    id: Uuid,
}

