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
  pub server_id: u8,
  pub storage_path: String,
  pub master_url: String,
}

impl Config {
  pub fn from_env() -> anyhow::Result<Config> {
    // dotenvy::dotenv_override().ok();
    dotenvy::from_filename("agent.env");
    envy::from_env().map_err(anyhow::Error::from)
  }
}
