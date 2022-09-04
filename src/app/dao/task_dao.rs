use crate::app::entity::*;

#[async_trait::async_trait]
pub trait TaskDao{
    async fn create_task(&self,task:Task)->anyhow::Result<String>;
}