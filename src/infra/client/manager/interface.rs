use std::sync::Arc;
use serde::{Deserialize, Serialize};

pub trait Entity<'a> : Send + Sync + Serialize + Deserialize<'a> {
    fn bucket()->String;
    fn set_id(&mut self,id:String);

}

#[async_trait::async_trait]
pub trait Dao <'a, E:Entity<'a>> : Send {
    async fn insert(&self, _: E)->anyhow::Result<E>;
}