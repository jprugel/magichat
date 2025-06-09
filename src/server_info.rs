use url::Url;
use std::path::PathBuf;

#[derive(Default)]
pub struct ServerInfo {
    url: String,
    pub name: String,
    pub icon: Icon,
    channel_list: Vec<Channel>
}

#[derive(Default)]
pub enum Icon {
    #[default]
    Default,
    Image(PathBuf),
}

pub struct Channel {
    log: Vec<UserMessage>
}

pub struct User {
    username: String,
    icon: Icon,
    server_list: Vec<ServerInfo>
}

impl Default for User {
    fn default() -> Self {
        Self {
            username: String::default(),
            icon: Icon::Default,
            server_list: Vec::default()
        }
    }
}

pub struct UserMessage {
    channel: String,
    user: String,
    message_content: String,
}
