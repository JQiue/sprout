use actix_multipart::form::MultipartForm;
use actix_web::{
  HttpResponse, post,
  web::{Data, Json},
};
use common::agent::{InitUploadRequest, TaskPublishRequest, TaskRevokeRequest};

use crate::{
  app::AppState,
  components::deployment::{model::UploadForm, service},
  error::AppError,
  traits::IntoHttpResponse,
};

#[post("/upload/init")]
pub async fn init_upload(
  state: Data<AppState>,
  body: Json<InitUploadRequest>,
) -> Result<HttpResponse, AppError> {
  service::get_upload_token(&state, body.0.site_id)
    .await
    .into_http_response()
}

#[post("/upload/file")]
pub async fn file_upload(
  state: Data<AppState>,
  MultipartForm(form): MultipartForm<UploadForm>,
) -> Result<HttpResponse, AppError> {
  service::file_upload(&state, form)
    .await
    .into_http_response()
}

#[post("/task/publish")]
pub async fn publish_site(
  state: Data<AppState>,
  body: Json<TaskPublishRequest>,
) -> Result<HttpResponse, AppError> {
  service::publish_site(
    &state,
    body.0.site_id,
    body.0.bandwidth,
    body.0.bind_domain,
    body.0.preview_domain,
  )
  .await
  .into_http_response()
}

#[post("/task/revoke")]
pub async fn revoke_site(
  state: Data<AppState>,
  body: Json<TaskRevokeRequest>,
) -> Result<HttpResponse, AppError> {
  service::revoke_site(&state, body.0.site_id)
    .await
    .into_http_response()
}
