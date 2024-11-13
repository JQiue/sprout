use actix_web::{
  delete,
  http::header::HeaderValue,
  post,
  web::{Data, Json},
  HttpResponse,
};
use serde_json::json;

use crate::{
  app::AppState,
  components::deployment::{model::*, service},
};

#[post("/user")]
pub async fn user_register(state: Data<AppState>, body: Json<UserRegisterBody>) -> HttpResponse {
  let Json(UserRegisterBody {
    email,
    password,
    display_name,
  }) = body;

  match service::user_register(&state, display_name, email, password).await {
    Ok(data) => HttpResponse::Ok().json(json!({
     "data": data,
     "errmsg": "",
     "errno": 0,
    })),
    Err(err) => HttpResponse::Ok().json(json!({
     "errmsg": err,
     "errno": 1000,
    })),
  }
}

#[post("/token")]
pub async fn user_login(state: Data<AppState>, body: Json<UserLoginBody>) -> HttpResponse {
  let Json(UserLoginBody { email, password }) = body;
  match service::user_login(&state, email, password).await {
    Ok(data) => HttpResponse::Ok().json(json!({
     "data": data,
     "code": "",
     "msg": "",
    })),
    Err(msg) => HttpResponse::Ok().json(json!({
     "code": "",
     "msg": msg,
    })),
  }
}
