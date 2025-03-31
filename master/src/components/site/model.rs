use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateSiteBody {
  pub site_name: String,
  // pub site_type: SiteType,
  // pub repo_url: Option<String>,
  // pub template_id: Option<String>,
}
