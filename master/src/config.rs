//! config

use helpers::uuid::{Alphabet, nanoid};
use serde::Deserialize;

use crate::error::AppError;

fn default_workers() -> usize {
  1
}

fn default_host() -> String {
  "127.0.0.1".to_string()
}

fn default_port() -> u16 {
  3000
}

fn default_key() -> String {
  nanoid(&Alphabet::DEFAULT, 8)
}

fn default_key_expire() -> i64 {
  100000
}

#[derive(Deserialize, Debug)]
pub struct Config {
  #[serde(default = "default_workers")]
  pub workers: usize,
  #[serde(default = "default_host")]
  pub host: String,
  #[serde(default = "default_port")]
  pub port: u16,
  pub database_url: String,
  #[serde(default = "default_key")]
  pub login_token_key: String,
  #[serde(default = "default_key")]
  pub register_agent_key: String,
  #[serde(default = "default_key_expire")]
  pub register_agent_key_expire: i64,
  pub cloudflare_api_key: String,
  pub cloudflare_email: String,
  pub cloudflare_zone_id: String,
}

impl Config {
  pub fn from_env() -> Result<Config, AppError> {
    dotenvy::dotenv_override().ok();
    Ok(envy::from_env()?)
  }
}
