//! config

use serde::Deserialize;

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
  pub register_agent_key: String,
  pub register_agent_key_expire: i64,
}

impl Config {
  pub fn from_env() -> anyhow::Result<Config> {
    dotenvy::dotenv_override().ok();
    envy::from_env().map_err(anyhow::Error::from)
  }
}
