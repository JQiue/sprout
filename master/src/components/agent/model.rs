use entity::agent;
use sea_orm::prelude::StringLen;
use sea_orm::{
  ColumnTrait, DatabaseConnection, DeriveActiveEnum, EntityTrait, EnumIter, QueryFilter,
};
use serde::{Deserialize, Serialize};

use crate::error::AppError;

#[derive(Serialize, Deserialize)]
pub struct RegisterAgentBody {
  pub hostname: String,
  pub ip_address: String,
  pub storage_path: String,
  pub available_space: u32,
  pub status: String,
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

pub async fn get_agent(agent_id: u32, db: &DatabaseConnection) -> Result<agent::Model, AppError> {
  agent::Entity::find()
    .filter(agent::Column::Id.eq(agent_id))
    .one(db)
    .await?
    .ok_or(AppError::AgentNotFound)
}

pub async fn get_avaliable_agent(db: &DatabaseConnection) -> Result<agent::Model, AppError> {
  agent::Entity::find()
    .filter(agent::Column::Status.eq(AgentStatus::Online))
    .one(db)
    .await?
    .ok_or(AppError::AgentNotFound)
}

pub enum AgentQueryBy {
  Id(u32),
  Hostname(String),
  IpAddress(String),
}

pub async fn has_agent(query_by: AgentQueryBy, db: &DatabaseConnection) -> Result<bool, AppError> {
  let mut query = agent::Entity::find();
  match query_by {
    AgentQueryBy::Id(id) => query = query.filter(agent::Column::Id.eq(id)),
    AgentQueryBy::Hostname(hostname) => query = query.filter(agent::Column::Hostname.eq(&hostname)),
    AgentQueryBy::IpAddress(ip_address) => {
      query = query.filter(agent::Column::IpAddress.eq(&ip_address))
    }
  }
  Ok(query.one(db).await?.is_some())
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
pub struct RpcCallData {
  pub code: i32,
  pub msg: String,
  pub data: AgentHeartbeat,
}

pub async fn get_agent_heartbeat(agent_ip: String) -> Result<RpcCallData, AppError> {
  let resp = reqwest::get(format!("http://{agent_ip}/api/heartbeat")).await?;
  let data = resp.json::<RpcCallData>().await?;
  Ok(data)
}
