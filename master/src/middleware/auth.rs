use actix_web::{dev::ServiceRequest, error::InternalError, web, Error, HttpMessage, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use helpers::jwt;
use serde::{Deserialize, Serialize};

use crate::{
  app::AppState,
  components::user::model::UserType,
  response::{Response, StatusCode},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JwtPayload {
  pub user_id: String,
  pub user_type: UserType,
}

pub async fn validator(
  req: ServiceRequest,
  credentials: Option<BearerAuth>,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
  let url = req.uri();
  let public_api = vec![
    "/api/health",
    "/api/user/token",
    "/api/user",
    "/api/deployment/status",
  ];
  if public_api.contains(&url.to_string().as_str()) {
    return Ok(req);
  }
  let Some(credentials) = credentials else {
    return Err((
      InternalError::from_response(
        "未携带 token",
        HttpResponse::Unauthorized()
          .content_type("application/json")
          .json(Response::<()>::error(StatusCode::Forbidden)),
      )
      .into(),
      req,
    ));
  };
  let state = req
    .app_data::<web::Data<AppState>>()
    .expect("State not found in app_data");
  println!("{}", state.login_token_key);
  match jwt::verify::<JwtPayload>(
    credentials.token().to_owned(),
    state.login_token_key.clone(),
  ) {
    Ok(data) => {
      req.extensions_mut().insert(data.claims.data);
      Ok(req)
    }
    Err(_) => Err((
      InternalError::from_response(
        "Invalid token",
        HttpResponse::Unauthorized()
          .content_type("application/json")
          .json(Response::<()>::error(StatusCode::AuthFailed)),
      )
      .into(),
      req,
    )),
  }
}
