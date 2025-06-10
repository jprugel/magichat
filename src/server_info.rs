use serde::{Deserialize, Serialize};
use serde_json;
use std::path::PathBuf;
use url::Url;

#[derive(Default, Clone, Debug)]
pub struct ServerInfo {
    pub url: String,
    pub name: String,
    pub icon: Icon,
    pub channel_list: Vec<Channel>,
}

#[derive(Default, Clone, Debug)]
pub enum Icon {
    #[default]
    Default,
    Image(PathBuf),
}

#[derive(Clone, Debug, Default)]
pub struct Channel {
    pub log: Vec<UserMessage>,
}

pub struct User {
    pub username: String,
}

impl Default for User {
    fn default() -> Self {
        Self {
            username: String::default(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct UserMessage {
    pub channel: String,
    pub user: String,
    pub content: String,
}
