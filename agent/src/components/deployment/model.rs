use actix_multipart::form::{json::Json as MPJson, tempfile::TempFile, text::Text, MultipartForm};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InitUploadBody {
  pub site_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
  pub path: String,
}

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
  #[multipart(limit = "100MB")]
  pub files: Vec<TempFile>,
  pub json: Vec<MPJson<Metadata>>,
  pub upload_token: Text<String>,
}

pub fn is_safe_path(path: &str) -> bool {
  !path.contains("..") && !path.contains("//")
}
