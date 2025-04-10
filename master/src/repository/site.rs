use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

use entity::site;

#[derive(Debug, Clone)]
pub struct SiteRepository<'a> {
  pub db: &'a DatabaseConnection,
}

impl<'a> SiteRepository<'a> {
  // pub async fn get_sites(&self) -> Result<Vec<site::Model>, DbErr> {
  //   site::Entity::find().all(self.db).await
  // }

  pub async fn create_site(&self, site: site::ActiveModel) -> Result<site::Model, DbErr> {
    site.insert(self.db).await
  }

  pub async fn update_site(&self, site: site::ActiveModel) -> Result<site::Model, DbErr> {
    site.update(self.db).await
  }

  pub async fn has_site(&self, site_id: &str) -> Result<bool, DbErr> {
    Ok(
      site::Entity::find()
        .filter(site::Column::SiteId.eq(site_id))
        .one(self.db)
        .await?
        .is_some(),
    )
  }

  pub async fn get_site_by_id(&self, site_id: &str) -> Result<Option<site::Model>, DbErr> {
    site::Entity::find()
      .filter(site::Column::SiteId.eq(site_id))
      .one(self.db)
      .await
  }
}
