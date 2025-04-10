use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateSiteBody {
  pub site_name: String,
}
