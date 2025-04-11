use actix_web::{
  HttpRequest, HttpResponse, get, post,
  web::{Data, Json, Path, ReqData},
};
use common::master::AssignTaskRequest;
use helpers::jwt;

use crate::{
  app::AppState,
  components::agent::{model::*, service},
  error::AppError,
  helper::extract_token,
  middlewares::JwtPayload,
  response::Response,
};

#[post("/agent")]
pub async fn register_agent(
  req: HttpRequest,
  state: Data<AppState>,
  body: Json<RegisterAgentBody>,
) -> Result<HttpResponse, AppError> {
  let user_id = jwt::verify(&extract_token(&req)?, &state.login_token_key)?
    .claims
    .data;
  let Json(RegisterAgentBody {
    hostname,
    ip_address,
    storage_path,
    available_space,
  }) = body;
  match service::register_agent(
    &state,
    user_id,
    hostname,
    ip_address,
    storage_path,
    available_space,
  )
  .await
  {
    Ok(data) => Response::success(Some(data)),
    Err(err) => Response::<()>::error(err),
  }
}

#[get("/agent/{agent_id}")]
pub async fn get_agent_status(
  state: Data<AppState>,
  agent_id: Path<u32>,
) -> Result<HttpResponse, AppError> {
  match service::get_agent_status(&state, agent_id.into_inner()).await {
    Ok(data) => Response::success(Some(data)),
    Err(err) => Response::<()>::error(err),
  }
}

#[post("/agent/{agent_id}/token")]
pub async fn refresh_agent_token(
  state: Data<AppState>,
  req_data: ReqData<JwtPayload>,
  agent_id: Path<u32>,
) -> Result<HttpResponse, AppError> {
  match service::refresh_agent_token(&state, req_data.user_id.clone(), agent_id.into_inner()).await
  {
    Ok(data) => Response::success(Some(data)),
    Err(err) => Response::<()>::error(err),
  }
}

#[post("/agent/task")]
pub async fn assign_task(
  state: Data<AppState>,
  body: Json<AssignTaskRequest>,
) -> Result<HttpResponse, AppError> {
  match service::assign_task(&state, body.0.r#type, body.0.site_id, body.0.deployment_id).await {
    Ok(data) => Response::success(Some(data)),
    Err(err) => Response::<()>::error(err),
  }
}
