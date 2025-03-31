use sea_orm::{
  ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, DeriveActiveEnum, EntityTrait,
  EnumIter, QueryFilter,
};

use entity::deployment;

#[derive(Debug, Clone)]
pub struct DeploymentRepository<'a> {
  pub db: &'a DatabaseConnection,
}

impl<'a> DeploymentRepository<'a> {
  pub async fn has_deployment(&self, id: u32) -> Result<bool, DbErr> {
    Ok(
      deployment::Entity::find()
        .filter(deployment::Column::Id.eq(id))
        .one(self.db)
        .await?
        .is_some(),
    )
  }

  // pub async fn get_deployments(&self) -> Result<Vec<deployment::Model>, DbErr> {
  //   deployment::Entity::find().all(self.db).await
  // }

  pub async fn get_deployment(
    &self,
    deployment_id: u32,
  ) -> Result<Option<deployment::Model>, DbErr> {
    deployment::Entity::find()
      .filter(deployment::Column::Id.eq(deployment_id))
      .one(self.db)
      .await
  }

  pub async fn create_deployment(
    &self,
    deployment: deployment::ActiveModel,
  ) -> Result<deployment::Model, DbErr> {
    deployment.insert(self.db).await
  }

  pub async fn update_deployment(
    &self,
    deployment: deployment::ActiveModel,
  ) -> Result<deployment::Model, DbErr> {
    deployment.update(self.db).await
  }
}
