use std::fs;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppConfig {
    pub server_conf: ServerConfig,
    pub mongodb_conf: MongodbConfig,
}

impl AppConfig {
    pub fn new() -> Self {
        let conf_file: &str = "env.toml";
        let content: String = match fs::read_to_string(conf_file) {
            Ok(c) => c,
            Err(_) => {
                eprintln!("Couldn't read {}", conf_file);
                std::process::exit(1);
            }
        };
        match toml::from_str::<AppConfig>(&content) {
            Ok(d) => d,
            Err(_) => {
                eprintln!("Couldn't parse toml file {}", conf_file);
                std::process::exit(1);
            }
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Deserialize)]
pub struct ServerConfig {
    pub http_listen_address: String,
    pub level: String,
}

#[derive(Deserialize)]
pub struct MongodbConfig {
    pub uri: String,
    pub db_name: String,
}

#[derive(PartialEq)]
pub enum Environment {
    Prod,
    Dev,
}

impl From<&str> for Environment {
    fn from(value: &str) -> Self {
        match value {
            "production" => Self::Prod,
            "development" => Self::Dev,
            _ => Self::Dev,
        }
    }
}
