use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterServerBody {
  pub hostname: String,
  pub ip_address: String,
  pub storage_path: String,
  pub available_space: u32,
  pub status: String,
}
