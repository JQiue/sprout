use entity::agent::AgentStatus;
use helpers::time::utc_now;
use sea_orm::{ActiveValue::Set, IntoActiveModel};

use crate::{error::AppError, repository::RepositoryManager};

pub async fn scheduled_task(db: &RepositoryManager) -> Result<(), AppError> {
  check_agents_status(&db).await?;
  Ok(())
}

async fn check_agents_status(db: &RepositoryManager) -> Result<(), AppError> {
  let agents = db.agent().get_agents().await?;
  let agent_rpc = rpc::AgentRpc::new()?;

  for agent in agents {
    let new_status = if let Ok(_) = agent_rpc.get_agent_heartbeat(&agent.ip_address).await {
      AgentStatus::Online
    } else {
      AgentStatus::Offline
    };
    tracing::debug!("Agent {} is {}", agent.ip_address, agent.status);

    if agent.status != new_status {
      let mut active_agent = agent.into_active_model();
      active_agent.status = Set(new_status);
      active_agent.updated_at = Set(Some(utc_now()));
      db.agent().update_agent(active_agent).await?;
    }
  }
  Ok(())
}
