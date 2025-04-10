use entity::agent::AgentStatus;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

use entity::agent;

pub enum AgentQueryBy {
  Id(u32),
  IpAddress(String),
}

#[derive(Debug, Clone)]
pub struct AgentRepository<'a> {
  pub db: &'a DatabaseConnection,
}

impl AgentRepository<'_> {
  pub async fn get_avaliable_agent(&self) -> Result<Option<agent::Model>, DbErr> {
    agent::Entity::find()
      .filter(agent::Column::Status.eq(AgentStatus::Online))
      .one(self.db)
      .await
  }

  // pub async fn get_agents(&self) -> Result<Vec<agent::Model>, DbErr> {
  //   agent::Entity::find().all(self.db).await
  // }

  pub async fn get_agent(&self, id: u32) -> Result<Option<agent::Model>, DbErr> {
    agent::Entity::find()
      .filter(agent::Column::Id.eq(id))
      .one(self.db)
      .await
  }

  pub async fn has_agent_by_id(&self, id: u32) -> Result<bool, DbErr> {
    self.has_agent(AgentQueryBy::Id(id)).await
  }

  pub async fn has_agent_by_ip(&self, ip: String) -> Result<bool, DbErr> {
    self.has_agent(AgentQueryBy::IpAddress(ip)).await
  }

  pub async fn has_agent(&self, query_by: AgentQueryBy) -> Result<bool, DbErr> {
    let mut select = agent::Entity::find();
    match query_by {
      AgentQueryBy::Id(id) => select = select.filter(agent::Column::Id.eq(id)),
      AgentQueryBy::IpAddress(ip) => select = select.filter(agent::Column::IpAddress.eq(ip)),
    }
    let res = select.one(self.db).await?;
    Ok(res.is_some())
  }

  pub async fn update_agent(&self, agent: agent::ActiveModel) -> Result<agent::Model, DbErr> {
    agent.update(self.db).await
  }
}
