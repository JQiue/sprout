use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
enum Agent {
  Table,
  Id,
  Hostname,       // 主机名
  IpAddress,      // IP 地址
  StoragePath,    // 存储根路径
  AvailableSpace, // 可用空间
  Status,         // 服务器状态，online，offline，busy
  Tags,           //  服务器标签，用于分组
  LastHeartbeat,  // 上次心跳时间
  CreatedAt,      //  首次注册时间
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
          .table(Agent::Table)
          .if_not_exists()
          .col(pk_auto(Agent::Id))
          .col(string(Agent::Hostname))
          .col(string(Agent::IpAddress))
          .col(string(Agent::StoragePath))
          .col(unsigned(Agent::AvailableSpace))
          .col(string(Agent::Status))
          .col(string(Agent::Tags))
          .col(timestamp(Agent::CreatedAt))
          .col(timestamp_null(Agent::UpdatedAt))
          .col(timestamp_null(Agent::LastHeartbeat))
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    // Replace the sample below with your own migration scripts

    manager
      .drop_table(Table::drop().table(Agent::Table).to_owned())
      .await
  }
}
