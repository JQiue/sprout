use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
enum Agent {
  Table,
  Id,             // 主键ID
  Hostname,       // 主机名
  IpAddress,      // IP 地址
  StoragePath,    // 存储根路径
  AvailableSpace, // 可用空间
  Status,         // 服务器状态，online，offline，busy
  Tags,           // 服务器标签，用于分组
  LastHeartbeat,  // 上次心跳时间
  Token,          // 注册时生成的 Token 用于 Master 进行验证
  CreatedAt,      // 首次注册时间
  UpdatedAt,      // 更新时间
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
          .col(string(Agent::Hostname).comment("主机名"))
          .col(string(Agent::IpAddress).comment("IP 地址"))
          .col(string(Agent::StoragePath).comment("存储根路径"))
          .col(unsigned(Agent::AvailableSpace).comment("可用空间"))
          .col(string(Agent::Status).comment("服务器状态，online，offline，busy"))
          .col(string_null(Agent::Tags).comment("服务器标签，用于分组"))
          .col(string(Agent::Token).comment("注册时生成的 Token 用于 Master 进行验证"))
          .col(timestamp(Agent::CreatedAt).comment("首次注册时间"))
          .col(timestamp_null(Agent::UpdatedAt).comment("更新时间"))
          .col(timestamp_null(Agent::LastHeartbeat).comment("上次心跳时间"))
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
