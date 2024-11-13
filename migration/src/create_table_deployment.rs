use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
enum Deployment {
  Table,
  Id,
  ProjectId,
  Version,
  Status,
  DeployedAt,
  CreatedAt,
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
          .table(Deployment::Table)
          .if_not_exists()
          .col(pk_auto(Deployment::Id))
          .col(string(Deployment::ProjectId))
          .col(string(Deployment::Version))
          .col(string(Deployment::Status))
          .col(date_time(Deployment::DeployedAt))
          .col(date_time(Deployment::CreatedAt))
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts

    manager
      .drop_table(Table::drop().table(Deployment::Table).to_owned())
      .await
  }
}
