use sea_orm::EnumIter;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
enum Site {
  Table,
  Id,
  SiteId,
  UserId,
  ServerId,    // 关联的服务器ID
  Name,        // 站点名称
  Domain,      // 主域名
  StorageUsed, // 存储使用量
  Status,      // 状态
  Type,        // 站点类型：导入、模板、手动上传
  RepoUrl,     // 代码仓库地址
  CreatedAt,
  UpdatedAt,
}

#[derive(Iden, EnumIter)]
pub enum SiteStatus {
  #[iden = "active"]
  Active,
  #[iden = "disabled"]
  Disabled,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(Site::Table)
          .if_not_exists()
          .col(pk_auto(Site::Id))
          .col(string(Site::SiteId))
          .col(string(Site::UserId))
          .col(integer(Site::ServerId))
          .col(string(Site::Name))
          .col(string(Site::Domain))
          .col(integer(Site::StorageUsed))
          .col(string(Site::Status).default("active"))
          .col(string(Site::Type))
          .col(string(Site::RepoUrl))
          .col(timestamp(Site::CreatedAt))
          .col(timestamp_null(Site::UpdatedAt))
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts

    manager
      .drop_table(Table::drop().table(Site::Table).to_owned())
      .await
  }
}
