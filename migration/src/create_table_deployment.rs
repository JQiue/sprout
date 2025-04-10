use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
enum Deployment {
  Table,
  Id,               // 主键 ID
  SiteId,           // 关联的站点 ID
  AgentId,          // 执行部署的 AgentID
  Status,           // pending, published, failed
  DeployToken,      // 上传 Token
  DeployUrl,        // 上传地址
  BuildLogs,        // 构建日志
  DeployPreviewUrl, // 部署预览 URL
  ExecutionTime,    // 执行时间（可以存储多个时间点）
  CreatedAt,        // 创建时间
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(Deployment::Table)
          .if_not_exists()
          .col(pk_auto(Deployment::Id).unsigned().comment("主键 ID"))
          .col(string(Deployment::SiteId).comment("关联的站点 ID"))
          .col(unsigned_null(Deployment::AgentId).comment("执行部署的 AgentID"))
          .col(string(Deployment::Status).comment("部署状态: pending, building, published, failed"))
          .col(string_null(Deployment::BuildLogs).comment("构建日志"))
          .col(string_null(Deployment::DeployPreviewUrl).comment("部署预览 URL"))
          .col(string_null(Deployment::DeployToken).comment("上传 token"))
          .col(string_null(Deployment::DeployUrl).comment("上传地址"))
          .col(timestamp(Deployment::ExecutionTime).comment("执行时间（可以存储多个时间点）"))
          .col(timestamp(Deployment::CreatedAt).comment("创建时间"))
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
