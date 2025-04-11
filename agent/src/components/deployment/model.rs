use actix_multipart::form::{MultipartForm, tempfile::TempFile, text::Text};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InitUploadBody {
  pub site_id: String,
}

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
  #[multipart(limit = "50MB")]
  pub dist: Vec<TempFile>,
  pub upload_token: Text<String>,
  pub deployment_id: Text<u32>,
}

#[derive(Debug, Deserialize)]
pub struct SitePublishBody {
  pub site_id: String,
  pub deployment_id: u32,
  pub bandwidth: String,
  pub preview_url: String,
}

#[derive(Debug, Deserialize)]
pub struct SiteRevokeBody {
  pub site_id: String,
}
