use actix_web::{dev::ServiceRequest, Error, HttpMessage, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use helpers::jwt;
use serde::{Deserialize, Serialize};

use crate::response::{Response, StatusCode};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JwtPayload {
  pub user_id: String,
}

pub async fn validator(
  req: ServiceRequest,
  credentials: Option<BearerAuth>,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
  let url = req.uri();
  if url.to_string().eq("/api/token") || url.eq("/api/health") || url.eq("/api/user") {
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
