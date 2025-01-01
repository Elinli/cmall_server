use anyhow::{bail, Result};
use std::{fs::File, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub auth: AuthConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub db_url: String,
    pub base_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    // 加密
    pub secret_key: String,
    // 解密
    pub public_key: String,
    // 有效期
    pub expires_in: u64,
}

impl AppConfig {
    pub fn load_config() -> Result<Self> {
        let rlt = match (
            File::open("service.yml"),
            File::open("/etc/config/service.yml"),
        ) {
            (Ok(reader), _) => serde_yaml::from_reader(reader),
            (_, Ok(reader)) => serde_yaml::from_reader(reader),
            _ => bail!("Load config file error!"),
        };
        Ok(rlt?)
    }
}
