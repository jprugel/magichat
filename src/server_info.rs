use serde::{Deserialize, Serialize};
use serde_json;
use std::path::PathBuf;
use url::Url;
use std::boxed::Box;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServerInfo {
    pub url: String,
    pub name: String,
    pub icon: Icon,
    pub channel_list: Vec<Channel>,
}

enum Message {
    ReceivedServerInfo(ServerInfo),
}

impl ServerInfo {
    pub async fn from_url(url: &str) -> Result<ServerInfo, Box<dyn std::error::Error + Send + Sync>> {
        let body = reqwest::get(url.clone())
            .await?
            .text()
            .await?;
        
        let server_info = serde_json::from_str(&body)?;
        Ok(server_info)
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub enum Icon {
    #[default]
    Default,
    Image(PathBuf),
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
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
