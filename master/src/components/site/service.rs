use super::model::{init_upload_session, DeploySource, SiteType};
use crate::{
  app::AppState,
  components::{agent::model::get_avaliable_agent, deployment::model::DeploymentStatus},
  entities::{
    deployment::{self},
    site,
  },
  response::StatusCode,
};
use helpers::{time::utc_now, uuid::Alphabet};
use sea_orm::{ActiveModelTrait, DatabaseConnection, IntoActiveModel, Set};
use serde_json::{json, Value};

/// Creates a new site for a user.
///
/// # Arguments
///
/// * `state` - A reference to the application state.
/// * `user_id` - The unique identifier of the user creating the site.
/// * `site_name` - The name of the site.
/// * `site_type` - The type of site being created.
/// * `repo_url` - An optional URL to the repository associated with the site.
///
/// # Returns
///
/// A `Result` containing a `Value` with the site creation status on success,
/// or a `String` error message on failure.
pub async fn create_site(
  state: &AppState,
  user_id: String,
  site_name: String,
  site_type: SiteType,
  repo_url: Option<String>,
  template_id: Option<String>,
) -> Result<Value, StatusCode> {
  let agent = get_avaliable_agent(&state.db).await?;
  let site_id = helpers::uuid::uuid_segmented(&Alphabet::NUMBERS_UPPER, 25, '-', 5);
  let site = site::ActiveModel {
    site_id: Set(site_id.clone()),
    name: Set(site_name),
    user_id: Set(user_id),
    r#type: Set(site_type.clone()),
    repo_url: Set(repo_url.clone()),
    created_at: Set(utc_now()),
    ..Default::default()
  }
  .insert(&state.db)
  .await
  .map_err(|_| StatusCode::DbError)?;
  let deploy_source = if template_id.is_some() {
    DeploySource::Template
  } else if repo_url.is_some() {
    DeploySource::Repository
  } else {
    DeploySource::Manual
  };
  let deployment = deployment::ActiveModel {
    status: Set(DeploymentStatus::Pending),
    agent_id: Set(agent.id),
    site_id: Set(site.clone().site_id),
    execution_time: Set(utc_now()),
    created_at: Set(utc_now()),
    ..Default::default()
  }
  .insert(&state.db)
  .await
  .map_err(|_| StatusCode::DbError)?;
  match deploy_source {
    DeploySource::Manual => {
      setup_manual_management(deployment, &site, agent.ip_address, &state.db).await
    }
    DeploySource::Template => {
      deploy_from_template(
        template_id.ok_or(StatusCode::ParamError)?,
        deployment,
        &site,
      )
      .await
    }
    DeploySource::Repository => {
      deploy_from_repository(repo_url.ok_or(StatusCode::ParamError)?, deployment, &site).await
    }
  }
}

// 下面是样例的部署函数
async fn deploy_from_template(
  _template_id: String,
  deployment: deployment::Model,
  site: &site::Model,
) -> Result<Value, StatusCode> {
  // 触发 agent 模板构建任务
  // 实现从模板生成站点的逻辑
  // 例如，复制文件、初始化配置等
  Ok(json!({
     "site_id": site.site_id,
      "agent_id": deployment.agent_id,
      "deploy_id": deployment.id,
  }))
}

async fn deploy_from_repository(
  _repo_url: String,
  deployment: deployment::Model,
  site: &site::Model,
) -> Result<Value, StatusCode> {
  // 触发 agent 仓库构建任务
  Ok(json!({
      "site_id": site.site_id,
      "agent_id": deployment.agent_id,
      "deploy_id": deployment.id,
  }))
}

async fn setup_manual_management(
  deployment: deployment::Model,
  site: &site::Model,
  ip_address: String,
  db: &DatabaseConnection,
) -> Result<Value, StatusCode> {
  // 提供 agent 上传接口，用于上传文件
  let init_response = init_upload_session(deployment.clone(), db).await?;
  let mut active_deployment = deployment.into_active_model();
  active_deployment.status = Set(DeploymentStatus::Uploading);
  active_deployment.upload_token = Set(Some(init_response.data.upload_token.clone()));
  let deployment = active_deployment
    .update(db)
    .await
    .map_err(|_| StatusCode::DbError)?;
  Ok(json!({
      "upload_url": ip_address,
      "upload_token": init_response.data.upload_token,
      "site_id": site.site_id,
      "agent_id": deployment.agent_id,
      "deploy_id": deployment.id,
  }))
}
