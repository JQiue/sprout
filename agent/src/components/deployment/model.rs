use actix_multipart::form::{MultipartForm, tempfile::TempFile, text::Text};

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
  #[multipart(limit = "50MB")]
  pub dist: Vec<TempFile>,
  pub upload_token: Text<String>,
}
