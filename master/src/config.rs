//! config

use serde::Deserialize;

use crate::error::AppError;

fn default_workers() -> usize {
  1
}

#[derive(Deserialize, Debug)]
pub struct Config {
  #[serde(default = "default_workers")]
  pub workers: usize,
  pub host: String,
  pub port: u16,
  pub database_url: String,
  pub login_token_key: String,
  pub register_agent_key: String,
  pub register_agent_key_expire: i64,
  pub cloudflare_api_key: String,
  pub cloudflare_email: String,
  pub cloudflare_zone_id: String,
}

impl Config {
  pub fn from_env() -> Result<Config, AppError> {
    dotenvy::dotenv_override().ok();
    Ok(envy::from_env::<Config>()?)
  }
}
