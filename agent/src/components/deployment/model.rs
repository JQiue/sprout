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
}
