//! config

use serde::Deserialize;

use crate::error::AppError;

fn default_host() -> String {
  "0.0.0.0".to_string()
}

fn default_port() -> u16 {
  5001
}

fn default_workers() -> usize {
  1
}

#[derive(Deserialize, Debug)]
pub struct Config {
  #[serde(default = "default_workers")]
  pub workers: usize,
  #[serde(default = "default_host")]
  pub host: String,
  #[serde(default = "default_port")]
  pub port: u16,
  pub agent_id: u32,
  pub agent_token: String,
  pub storage_path: String,
  pub master_url: String,
  pub upload_token_key: String,
  pub upload_token_key_expire: i64,
}

impl Config {
  pub fn from_env() -> Result<Config, AppError> {
    dotenvy::dotenv_override()?;
    Ok(envy::from_env()?)
  }
}
