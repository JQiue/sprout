use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
enum Server {
  Table,
  Id,
  Hostname,       // 主机名
  IpAddress,      // IP 地址
  StoragePath,    // 存储根路径
  AvailableSpace, // 可用空间
  Status,         // 服务器状态
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
          .table(Server::Table)
          .if_not_exists()
          .col(pk_auto(Server::Id))
          .col(string(Server::Hostname))
          .col(string(Server::IpAddress))
          .col(string(Server::StoragePath))
          .col(unsigned(Server::AvailableSpace))
          .col(string(Server::Status))
          .col(timestamp(Server::CreatedAt))
          .col(timestamp_null(Server::UpdatedAt))
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts

    manager
      .drop_table(Table::drop().table(Server::Table).to_owned())
      .await
  }
}
