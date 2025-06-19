use serde::{Deserialize, Serialize};
use serde_json;
use std::path::PathBuf;
use uuid::Uuid;

const ROUTE_INFO: &str = "/info";

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Server {
    pub url: String,
    pub name: String,
    pub icon: Icon,
    pub channel_list: Vec<Channel>,
}

impl Server {
    pub async fn from_url(url: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let base = format!("http://{}", url);
        let mut parsed_url = reqwest::Url::parse(&base)?;
        parsed_url.set_path(ROUTE_INFO);
        let response = reqwest::get(parsed_url).await?;
        let body = response.text().await?;
        let server: Server = serde_json::from_str(&body)?;
        Ok(server)
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub enum Icon {
    #[default]
    Default,
    Svg(PathBuf),
    Image(Vec<u8>),
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Channel {
    pub log: Vec<UserMessage>,
    pub id: Uuid,
    pub name: String,
}

impl Channel {
    pub fn from_name(name: String) -> Self {
        Self {
            log: Vec::new(),
            id: Uuid::new_v4(),
            name,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserMessage {
    pub user: User,
    pub channel_id: Uuid,
    pub content: String,
    pub id: Uuid,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub icon: Icon,
}

impl Default for User {
    fn default() -> Self {
        Self {
            username: String::default(),
            icon: Icon::Default,
        }
    }
}
