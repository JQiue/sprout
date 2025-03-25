use crate::entities::agent;
use crate::response::StatusCode;
use sea_orm::prelude::StringLen;
use sea_orm::{
  ColumnTrait, DatabaseConnection, DeriveActiveEnum, EntityTrait, EnumIter, QueryFilter,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RegisterAgentBody {
  pub hostname: String,
  pub ip_address: String,
  pub storage_path: String,
  pub available_space: u32,
  pub status: AgentStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(
  rs_type = "String",
  db_type = "String(StringLen::None)",
  rename_all = "lowercase"
)]
#[serde(rename_all = "lowercase")]
pub enum AgentStatus {
  Online,
  Offline,
  Busy,
}

pub async fn get_agent(agent_id: i32, db: &DatabaseConnection) -> Result<agent::Model, StatusCode> {
  agent::Entity::find()
    .filter(agent::Column::Id.eq(agent_id))
    .one(db)
    .await
    .map_err(|_| StatusCode::DbError)?
    .ok_or(StatusCode::AgentNotFound)
}

pub async fn get_avaliable_agent(db: &DatabaseConnection) -> Result<agent::Model, StatusCode> {
  agent::Entity::find()
    .filter(agent::Column::Status.eq(AgentStatus::Online))
    .one(db)
    .await
    .map_err(|_| StatusCode::DbError)?
    .ok_or(StatusCode::AgentNotFound)
}

pub enum AgentQueryBy {
  Id(i32),
  Hostname(String),
  IpAddress(String),
}

pub async fn has_agent(
  query_by: AgentQueryBy,
  db: &DatabaseConnection,
) -> Result<bool, StatusCode> {
  let mut query = agent::Entity::find();
  match query_by {
    AgentQueryBy::Id(id) => query = query.filter(agent::Column::Id.eq(id)),
    AgentQueryBy::Hostname(hostname) => query = query.filter(agent::Column::Hostname.eq(hostname)),
    AgentQueryBy::IpAddress(ip_address) => {
      query = query.filter(agent::Column::IpAddress.eq(ip_address))
    }
  }
  Ok(
    query
      .one(db)
      .await
      .map_err(|_| StatusCode::DbError)?
      .is_some(),
  )
}

#[derive(Serialize, Deserialize)]
pub struct AgentHeartbeat {
  pub cpu_cores: i8,
  pub cpu_usage: f32,
  pub total_memory: i32,
  pub free_memory: i32,
  pub memory_usage: f32,
}

#[derive(Serialize, Deserialize)]
pub struct GetAgentHeartBeatJson {
  pub code: i32,
  pub msg: String,
  pub data: AgentHeartbeat,
}

pub async fn get_agent_heartbeat(agent_ip: String) -> Result<GetAgentHeartBeatJson, String> {
  let resp = reqwest::get(format!("http://{agent_ip}/api/heartbeat"))
    .await
    .map_err(|_| "发送请求失败".to_string())?;

  resp
    .json::<GetAgentHeartBeatJson>()
    .await
    .map_err(|_| "响应体序列化失败".to_string())
}
