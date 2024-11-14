use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
enum Deployment {
  Table,
  Id,               // 主键ID
  SiteId,           // 关联的站点ID
  AgentId,          // 执行部署的AgentID
  Version,          // 版本号
  Status,           // pending, building, published, failed
  StartedAt,        // 开始时间
  CompletedAt,      // 完成时间
  CommitHash,       // Git提交hash
  CommitMessage,    // 提交信息
  Branch,           // 构建分支
  BuildLogs,        // 构建日志
  ErrorMessage,     // 错误信息
  DeployPreviewUrl, // 部署预览 URL
  BuildDuration,    // 构建时间
  DeployedAt,       // 部署时间
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
          .col(pk_auto(Deployment::Id).comment("主键ID"))
          .col(string(Deployment::SiteId).comment("关联的站点ID"))
          .col(string(Deployment::AgentId).comment("执行部署的AgentID"))
          .col(string(Deployment::Version).comment("版本号"))
          .col(string(Deployment::Status).comment("部署状态: pending, building, published, failed"))
          .col(string(Deployment::CommitHash).comment("Git 提交hash"))
          .col(string(Deployment::CommitMessage).comment("提交信息"))
          .col(string(Deployment::Branch).comment("构建分支"))
          .col(string(Deployment::BuildLogs).comment("构建日志"))
          .col(string(Deployment::ErrorMessage).comment("错误信息"))
          .col(string(Deployment::DeployPreviewUrl).comment("部署预览 URL"))
          .col(string(Deployment::BuildDuration).comment("构建时间"))
          .col(timestamp(Deployment::StartedAt).comment("开始时间"))
          .col(timestamp(Deployment::CompletedAt).comment("完成时间"))
          .col(timestamp(Deployment::DeployedAt).comment("部署时间"))
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
