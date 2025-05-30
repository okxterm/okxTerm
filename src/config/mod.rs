use std::{fs::File, io::Read};

use anyhow::anyhow;
use once_cell::sync::OnceCell;
use serde::Deserialize;

use clap::Parser;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// config file path
    #[arg(short, long)]
    pub config: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub lang: String,
    pub okx: OkxConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OkxConfig {
    pub api_key: String,
    pub secret_key: String,
    pub passphrase: String,
}

static CONFIG: OnceCell<Config> = OnceCell::new();

impl Config {
    pub fn new(config_file: Option<String>) -> anyhow::Result<Config> {
        let config_file = config_file.unwrap_or("./config.toml".to_string());

        let mut file = File::open(config_file).map_err(|err| anyhow!(err))?;
        let mut buffer = String::new();
        let _ = file
            .read_to_string(&mut buffer)
            .map_err(|err| anyhow!(err))?;

        let config: Config = toml::from_str(buffer.as_str()).map_err(|err| anyhow!(err))?;

        CONFIG
            .set(config.clone())
            .map_err(|_| anyhow!("Failed to set CONFIG"))?;

        Ok(config)
    }
}

pub fn config() -> anyhow::Result<Config> {
    let config = CONFIG
        .get()
        .ok_or_else(|| anyhow!("Failed to get CONFIG"))?;
    Ok(config.clone())
}
