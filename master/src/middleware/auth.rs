use actix_web::{dev::ServiceRequest, Error, HttpMessage, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use helpers::jwt;
use serde::{Deserialize, Serialize};

use crate::{
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
      actix_web::error::InternalError::from_response(
        "Invalid token",
        HttpResponse::Unauthorized()
          .content_type("application/json")
          .json(Response::<()>::error(StatusCode::Forbidden)),
      )
      .into(),
      req,
    ));
  };

  match jwt::verify::<JwtPayload>(credentials.token().to_owned(), "sprout".to_owned()) {
    Ok(data) => {
      req.extensions_mut().insert(data.claims.data);
      Ok(req)
    }
    Err(_) => Err((
      actix_web::error::InternalError::from_response(
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
