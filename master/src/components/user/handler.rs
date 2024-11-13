use actix_web::{
  post,
  web::{Data, Json},
  HttpResponse,
};

use crate::{
  app::AppState,
  components::user::{model::*, service},
  response::Response,
};

#[post("/user")]
pub async fn user_register(state: Data<AppState>, body: Json<UserRegisterBody>) -> HttpResponse {
  let Json(UserRegisterBody {
    nickname,
    password,
    email,
  }) = body;

  match service::user_register(&state, nickname, email, password).await {
    Ok(data) => HttpResponse::Ok().json(Response::success(data)),
    Err(err) => HttpResponse::Ok().json(Response::<()>::error(err)),
  }
}

#[post("/token")]
pub async fn user_login(state: Data<AppState>, body: Json<UserLoginBody>) -> HttpResponse {
  let Json(UserLoginBody { email, password }) = body;
  match service::user_login(&state, email, password).await {
    Ok(data) => HttpResponse::Ok().json(Response::success(data)),
    Err(err) => HttpResponse::Ok().json(Response::<()>::error(err)),
  }
}
