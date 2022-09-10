use serde::{ Serialize};
use serde::de::DeserializeOwned;

pub trait Entity<'a> : Send + Sync + Serialize  + DeserializeOwned + Unpin  {//+ Deserialize<'a>
    fn bucket()->String;
    fn set_id(&mut self,id:String);
    fn get_id(&mut self)->(String,String);

}

#[async_trait::async_trait]
pub trait Dao <'a, E:Entity<'a>> : Send + Sync {
    async fn insert(&self, _: E)->anyhow::Result<E>;
    async fn find_by_code(&self,code:String)->anyhow::Result<Option<E>>;
    async fn update_by_code(&self,_:E) -> anyhow::Result<u64>;
    async fn insert_many(&self, _: Vec<E>)->anyhow::Result<Vec<E>>;
}