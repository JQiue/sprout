use actix_multipart::form::MultipartForm;
use actix_web::{
  post,
  web::{Data, Json},
  HttpResponse,
};

use crate::{
  app::AppState,
  components::deployment::{
    model::{InitUploadBody, UploadForm},
    service,
  },
  response::Response,
};

#[post("/upload/init")]
pub async fn init_upload(state: Data<AppState>, body: Json<InitUploadBody>) -> HttpResponse {
  let Json(InitUploadBody { site_id }) = body;
  match service::init_upload(&state, site_id).await {
    Ok(data) => HttpResponse::Ok().json(Response::success(data)),
    Err(err) => HttpResponse::Ok().json(Response::<()>::error(err)),
  }
}

#[post("/upload/file")]
pub async fn file_upload(
  state: Data<AppState>,
  MultipartForm(form): MultipartForm<UploadForm>,
) -> HttpResponse {
  match service::file_upload(&state, form).await {
    Ok(data) => HttpResponse::Ok().json(Response::success(data)),
    Err(err) => HttpResponse::Ok().json(Response::<()>::error(err)),
  }
}
