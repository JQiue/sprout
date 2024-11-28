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
  pub agent_id: u8,
  pub storage_path: String,
  pub master_url: String,
  pub upload_token_key: String,
  pub upload_token_key_expire: i64,
}

impl Config {
  pub fn from_env() -> Config {
    // dotenvy::dotenv_override().ok();
    dotenvy::from_filename("agent.env").unwrap();
    envy::from_env().unwrap()
  }
}
