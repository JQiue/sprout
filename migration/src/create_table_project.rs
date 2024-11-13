use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
enum Project {
  Table,
  Id,
  UserId,
  ServerId,    // 关联的服务器ID
  Name,        // 项目名
  Domain,      // 项目域名
  StorageUsed, // 存储使用量
  Status,      // 项目状态
  CreatedAt,
  UpdatedAt,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts

    manager
      .create_table(
        Table::create()
          .table(Project::Table)
          .if_not_exists()
          .col(pk_auto(Project::Id))
          .col(string(Project::UserId))
          .col(integer(Project::ServerId))
          .col(string(Project::Name))
          .col(string(Project::Domain))
          .col(integer(Project::StorageUsed))
          .col(string(Project::Status))
          .col(timestamp(Project::CreatedAt))
          .col(timestamp_null(Project::UpdatedAt))
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts

    manager
      .drop_table(Table::drop().table(Project::Table).to_owned())
      .await
  }
}
