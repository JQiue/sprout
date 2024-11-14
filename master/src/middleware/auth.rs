use actix_web::{dev::ServiceRequest, error, App, Error};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::app::AppState;

pub async fn validator(
  req: ServiceRequest,
  credentials: Option<BearerAuth>,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
  let url = req.uri();
  if url.to_string().eq("/api/token") || url.eq("/api/health") || url.eq("/api/user") {
    return Ok(req);
  }
  let Some(credentials) = credentials else {
    return Err((error::ErrorBadRequest("no bearer header"), req));
  };
  println!("{credentials:?}");
  Ok(req)
}
