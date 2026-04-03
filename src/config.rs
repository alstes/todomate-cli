use crate::error::CliError;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const DEFAULT_API_BASE_URL: &str = "https://api.todo.ac";
const CONFIG_DIR: &str = "todomate";
const CONFIG_FILE: &str = "config.toml";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_api_base_url")]
    pub api_base_url: String,
}

fn default_api_base_url() -> String {
    DEFAULT_API_BASE_URL.to_string()
}

impl Default for Config {
    fn default() -> Self {
        Config {
            api_base_url: DEFAULT_API_BASE_URL.to_string(),
        }
    }
}

fn config_path() -> Result<PathBuf> {
    let dir = dirs::config_dir()
        .ok_or_else(|| CliError::Config("Cannot determine config directory".to_string()))?
        .join(CONFIG_DIR);
    Ok(dir.join(CONFIG_FILE))
}

pub fn load() -> Result<Config> {
    let path = config_path()?;
    if !path.exists() {
        return Ok(Config::default());
    }
    let contents = fs::read_to_string(&path)?;
    let config: Config = toml::from_str(&contents).map_err(|e| CliError::Config(e.to_string()))?;
    Ok(config)
}

pub fn save(config: &Config) -> Result<()> {
    let path = config_path()?;
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    let contents = toml::to_string_pretty(config).map_err(|e| CliError::Config(e.to_string()))?;
    fs::write(&path, contents)?;
    Ok(())
}

pub fn set_value(key: &str, value: &str) -> Result<()> {
    let mut config = load()?;
    match key {
        "api-url" => config.api_base_url = value.to_string(),
        other => return Err(CliError::Config(format!("Unknown config key: {other}")).into()),
    }
    save(&config)
}

pub fn get_value(key: &str) -> Result<String> {
    let config = load()?;
    let value = match key {
        "api-url" => config.api_base_url,
        other => return Err(CliError::Config(format!("Unknown config key: {other}")).into()),
    };
    Ok(value)
}

pub fn reset() -> Result<()> {
    save(&Config::default())
}
