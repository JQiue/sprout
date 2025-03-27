use sea_orm::prelude::StringLen;
use sea_orm::{
  ColumnTrait, DatabaseConnection, DeriveActiveEnum, EntityTrait, EnumIter, QueryFilter,
};
use serde::{Deserialize, Serialize};

use entity::deployment::{self};

use crate::error::AppError;

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
  pub deployment_id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDeploymentStatusBody {
  pub agent_id: u32,
  pub agent_token: String,
  pub deployment_id: u32,
  pub status: String,
}

pub enum DeploymentQueryBy {
  Id(u32),
}

pub async fn has_deployment(
  query_by: DeploymentQueryBy,
  db: &DatabaseConnection,
) -> Result<bool, AppError> {
  let mut query = deployment::Entity::find();
  match query_by {
    DeploymentQueryBy::Id(id) => query = query.filter(deployment::Column::Id.eq(id)),
  }
  Ok(
    query
      .one(db)
      .await?
      .ok_or(AppError::DeploymentNotFound)
      .is_ok(),
  )
}

pub async fn get_deployment(
  query_by: DeploymentQueryBy,
  db: &DatabaseConnection,
) -> Result<deployment::Model, AppError> {
  let mut query = deployment::Entity::find();
  match query_by {
    DeploymentQueryBy::Id(id) => query = query.filter(deployment::Column::Id.eq(id)),
  }
  query.one(db).await?.ok_or(AppError::DeploymentNotFound)
}
