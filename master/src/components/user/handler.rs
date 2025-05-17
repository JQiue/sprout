use crate::{
  app::AppState,
  components::user::{model::*, service},
  error::AppError,
  middlewares::JwtPayload,
  traits::{IntoHttpResponse, MasterResponse},
};
use actix_web::{
  HttpResponse, get, post,
  web::{Data, Json, ReqData},
};
use common::{
  Response,
  master::{UserLoginRequest, UserRegisterRequest},
};
use validator::Validate;

#[get("/user/casual")]
pub async fn generate_casual_user(state: Data<AppState>) -> Result<HttpResponse, AppError> {
  service::generate_casual_user(&state)
    .await
    .into_http_response()
}

#[post("/user")]
pub async fn user_register(
  state: Data<AppState>,
  body: Json<UserRegisterRequest>,
) -> Result<HttpResponse, AppError> {
  body.0.validate()?;
  service::user_register(&state, body.0.nickname, body.0.email, body.0.password)
    .await
    .into_http_response()
}

#[post("/user/token")]
pub async fn user_login(
  state: Data<AppState>,
  body: Json<UserLoginRequest>,
) -> Result<HttpResponse, AppError> {
  service::user_login(&state, body.0.email, body.0.password)
    .await
    .into_http_response()
}

#[get("/user/info")]
pub async fn get_user_info(
  state: Data<AppState>,
  req_data: ReqData<JwtPayload>,
) -> Result<HttpResponse, AppError> {
  service::get_user_info(&state, req_data.user_id.clone())
    .await
    .into_http_response()
}

#[post("/user/password")]
pub async fn set_user_password(
  state: Data<AppState>,
  body: Json<SetUserPasswordBody>,
  req_data: ReqData<JwtPayload>,
) -> Result<HttpResponse, AppError> {
  let Json(SetUserPasswordBody { password }) = body;
  service::set_user_password(&state, req_data.user_id.clone(), password)
    .await
    .into_http_response()
}

#[post("/token/refresh")]
pub async fn refresh_user_token(
  state: Data<AppState>,
  req_data: ReqData<JwtPayload>,
) -> Result<HttpResponse, AppError> {
  service::refresh_user_token(&state, req_data.user_id.clone())
    .await
    .into_http_response()
}
