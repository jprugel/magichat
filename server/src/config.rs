use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub info: Info,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: [u8; 4],
    pub port: u16,
}

impl ServerConfig {
    pub fn to_addr(&self) -> String {
        format!(
            "{}.{}.{}.{}:{}",
            self.host[0], self.host[1], self.host[2], self.host[3], self.port
        )
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Info {
    pub server_name: String,
    pub channels: Vec<String>,
    pub icon: PathBuf,
}

pub fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
