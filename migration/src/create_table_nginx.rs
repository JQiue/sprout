use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
enum Nginx {
  Table,
  Id,
  SiteId,
  AgentId,
  ConfigContent,
  CreatedAt,
  UpdatedAt,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(Nginx::Table)
          .if_not_exists()
          .col(pk_auto(Nginx::Id))
          .col(string(Nginx::SiteId).unique_key().comment("站点 ID"))
          .col(string(Nginx::AgentId).unique_key().comment("Agent ID"))
          .col(string(Nginx::ConfigContent))
          .col(timestamp(Nginx::CreatedAt))
          .col(timestamp_null(Nginx::UpdatedAt))
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts

    manager
      .drop_table(Table::drop().table(Nginx::Table).to_owned())
      .await
  }
}
