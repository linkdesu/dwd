use super::dns_provider::dynv6_com::ConfigDynv6Com;
use super::dns_provider::name_com::ConfigNameCom;
use serde::{Deserialize, Serialize};
use std::{error::Error, fs, path::Path};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub dns_provider: Vec<String>,
    pub ip_provider: Vec<String>,
    pub interval: u32,

    pub name_com: Option<ConfigNameCom>,
    pub dynv6_com: Option<ConfigDynv6Com>,
}

pub fn load_config(config_path: &str) -> Result<Config, Box<dyn Error>> {
    let config_path = Path::new(config_path);
    let config_str = match fs::read_to_string(config_path) {
        Ok(data) => data,
        Err(e) => {
            return Err(format!(
                "[config::load_config] Read file failed. (path: {}, error: {})",
                config_path
                    .to_str()
                    .expect("The file path should be valid utf-8 string."),
                e.to_string()
            )
            .into())
        }
    };

    let config: Config = match config_path.extension().map(|val| val.to_str()) {
        Some(Some("json")) => match serde_json::from_str(&config_str) {
            Ok(config) => config,
            Err(err) => {
                return Err(format!("[config::load_config] Parse json failed. (error: {})", err.to_string()).into());
            }
        },
        Some(Some("toml")) => match toml::from_str(&config_str) {
            Ok(config) => config,
            Err(err) => {
                return Err(format!("[config::load_config] Parse toml failed. (error: {})", err.to_string()).into());
            }
        },
        Some(Some("yaml")) => match serde_yaml::from_str(&config_str) {
            Ok(config) => config,
            Err(err) => {
                return Err(format!("[config::load_config] Parse yaml failed. (error: {})", err.to_string()).into());
            }
        },
        _ => return Err("[config::load_config] Only .json, .toml and .yaml format is supported.".into()),
    };

    Ok(config)
}
