use serde::Deserialize;
use std::fs;

// 定义收件人结构
#[derive(Clone, Deserialize)]
pub struct Recipient {
    pub email: String,
    pub name: String,
    pub message: String,
    pub subject: String,
}

// 定义配置结构
#[derive(Clone, Deserialize)]
pub struct Config {
    pub smtp_server: String,
    pub smtp_port: u16,
    pub smtp_authorization_code: String,
    pub from_email: String,
    pub recipients: Vec<Recipient>,
}

impl Config {
    // 从配置文件加载配置
    pub fn new() -> Result<Self, config::ConfigError> {
        // 读取配置文件
        let config_str = fs::read_to_string("config.toml").expect("Failed to read config file");

        // 解析TOML格式的配置
        let config: Config = toml::from_str(&config_str).expect("Failed to parse config file");

        Ok(config)
    }
}
