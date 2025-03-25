use sea_orm::prelude::StringLen;
use sea_orm::{
  ColumnTrait, DatabaseConnection, DeriveActiveEnum, EntityTrait, EnumIter, QueryFilter,
};
use serde::{Deserialize, Serialize};

use crate::entities::deployment::{self};
use crate::response::StatusCode;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(
  rs_type = "String",
  db_type = "String(StringLen::None)",
  rename_all = "lowercase"
)]
#[serde(rename_all = "lowercase")]
pub enum DeploymentStatus {
  Pending,
  Building,
  Uploading,
  Reviewing,
  Published,
  Failed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDeploymentBody {
  pub deployment_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDeploymentStatusBody {
  pub agent_id: i32,
  pub agent_token: String,
  pub deployment_id: i32,
  pub status: DeploymentStatus,
}

pub enum DeploymentQueryBy {
  Id(i32),
}

pub async fn has_deployment(
  query_by: DeploymentQueryBy,
  db: &DatabaseConnection,
) -> Result<bool, StatusCode> {
  let mut query = deployment::Entity::find();
  match query_by {
    DeploymentQueryBy::Id(id) => query = query.filter(deployment::Column::Id.eq(id)),
  }
  Ok(
    query
      .one(db)
      .await
      .map_err(|_| StatusCode::DbError)?
      .ok_or(StatusCode::DeploymentNotFound)
      .is_ok(),
  )
}

pub async fn get_deployment(
  query_by: DeploymentQueryBy,
  db: &DatabaseConnection,
) -> Result<deployment::Model, StatusCode> {
  let mut query = deployment::Entity::find();
  match query_by {
    DeploymentQueryBy::Id(id) => query = query.filter(deployment::Column::Id.eq(id)),
  }
  query
    .one(db)
    .await
    .map_err(|_| StatusCode::DbError)?
    .ok_or(StatusCode::DeploymentNotFound)
}
