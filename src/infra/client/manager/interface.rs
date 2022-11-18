#![allow(dead_code)]

use std::time::Duration;
use serde::{ Serialize};
use serde::de::DeserializeOwned;
use serde_json::Value;

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
    async fn find(&self, _:Vec<(String, QueryOption)>,page:i64,size:i64) ->anyhow::Result<(Vec<E>,i64)>;
}

pub enum QueryOption{
    Equal(Value),
    GreaterThan(Value),
    LessThan(Value),
    BetweenAnd(Value,Value),
    Like(Value),
    Contain(Vec<Value>)
}

#[async_trait::async_trait]
pub trait Cache:Send+Sync{
    async fn set(&self,key:String,value:String,ttl:Duration)->anyhow::Result<()>;
    async fn get(&self,key:String)->anyhow::Result<String>;
}

#[derive(Default)]
pub struct Node{
    pub min:i32,
    pub max:i32,
    pub version:i64,
}

#[async_trait::async_trait]
pub trait ShareCenter:Send+Sync{
    async fn version(&self,key:String)->anyhow::Result<i64>;
    async fn set_version(&self,key:String,version:i64)->anyhow::Result<()>;
    async fn nodes(&self,node_cluster:String)->anyhow::Result<Vec<String>>;
    async fn get_node(&self,key:String)->anyhow::Result<Option<Node>>;
    async fn add_node(&self, key:String, node:Node) ->anyhow::Result<()>;
    async fn register_node(&self,cluster:String, key:String) ->anyhow::Result<()>;
    async fn del_node(&self, key:String) ->anyhow::Result<()>;
}