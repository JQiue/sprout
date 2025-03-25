use sea_orm::EnumIter;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
enum Site {
  Table,
  Id,          // 主键ID
  SiteId,      // 站点ID
  UserId,      // 用户ID
  ServerId,    // 关联的服务器ID
  Name,        // 站点名称
  Domain,      // 主域名
  StorageUsed, // 存储使用量
  Status,      // 状态
  Type,        // 站点类型：imported、template、manual
  RepoUrl,     // 代码仓库地址
  CreatedAt,   // 创建时间
  UpdatedAt,   // 更新时间
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
          .col(string(Site::SiteId).comment("站点ID"))
          .col(string(Site::UserId).comment("用户ID"))
          .col(integer_null(Site::ServerId).comment("关联的服务器ID"))
          .col(string(Site::Name).comment("站点名称"))
          .col(string_null(Site::Domain).comment("主域名"))
          .col(integer(Site::StorageUsed).comment("存储使用量").default(0))
          .col(string(Site::Status).default("active").comment("状态"))
          .col(string(Site::Type).comment("站点类型：imported、template、manual"))
          .col(string_null(Site::RepoUrl).comment("代码仓库地址"))
          .col(timestamp(Site::CreatedAt).comment("创建时间"))
          .col(timestamp_null(Site::UpdatedAt).comment("更新时间"))
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
