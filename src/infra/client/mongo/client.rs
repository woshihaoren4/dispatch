use std::any::Any;
use mongodb::bson::{Bson, doc, Document};
use mongodb::Collection;
use mongodb::options::{ClientOptions, UpdateModifications};
use serde_json::Value;
use crate::conf::MongoDb;
use crate::infra::client::manager::{ Dao, Entity};


pub struct MongoClient{
    // cfg:MongoDb,
    client: mongodb::Client,
}

impl MongoClient {
    pub async fn new(app_name:String,cfg:MongoDb)->anyhow::Result<Self>{
        let mut opts = ClientOptions::parse(cfg.url.clone()).await?;
        opts.app_name = Some(app_name);
        opts.max_pool_size =opts.max_pool_size;
        let client = mongodb::Client::with_options(opts)?;
        let mc = MongoClient{
            client,
            // cfg,client,
        };
        return Ok(mc)
    }

    pub async fn get_dao<'a, E:Entity<'a>>(&self) -> MongoDao<E> {
        let coll = self.client.default_database().unwrap().collection(&E::bucket());
        return MongoDao::from(coll)
    }
}

pub struct MongoDao<V>{
    coll : Collection<V>,
}

impl<V> MongoDao<V> {
    pub fn value_to_bson(value:Value)->Bson{
        match value {
            Value::Null => {Bson::Null}
            Value::Bool(b) => {Bson::Boolean(b)}
            Value::Number(n) => {
                if let Some(s) = n.as_u64() {
                    if s != 0{
                        Bson::Int64(s as i64)
                    }else{
                        Bson::Null
                    }
                }else if let Some(s) = n.as_i64() {
                    if s != 0 {
                        Bson::Int64(s)
                    }else{
                        Bson::Null
                    }
                }else if let Some(s) = n.as_f64() {
                    if s != 0f64 {
                        Bson::Double(s)
                    }else{
                        Bson::Null
                    }
                }else{
                    Bson::Null
                }
            }
            Value::String(s) => {
                if s.len() > 0 {
                    Bson::String(s)
                }else{
                    Bson::Null
                }
            }
            Value::Array(list) => {
                let mut bson_list = vec![];
                for i in list.into_iter(){
                    let b = Self::value_to_bson(i);
                    bson_list.push(b);
                }
                if bson_list.is_empty() {
                    Bson::Null
                }else{
                    Bson::Array(bson_list)
                }
            }
            Value::Object(_) => {Bson::Null}
        }
    }
    pub fn value_to_document(value:Value,filters:Vec<String>)->anyhow::Result<Document>{
        let map = match value {
            Value::Object(o)=>o,
            _=>return Err(anyhow::anyhow!("value_to_document: this is not Object"))
        };
        let mut doc = Document::new();
        'f:for (k,v) in map.into_iter(){
            for filter in filters.iter(){
                if k.eq(filter) {
                    continue 'f;
                }
            }
            let value = Self::value_to_bson(v);
            if value != Bson::Null {
                doc.insert(k,value);
            }
        }
        return Ok(doc)
    }
}

impl<V> From<Collection<V>> for MongoDao<V> {
    fn from(coll: Collection<V>) -> Self {
        Self{coll}
    }
}

#[async_trait::async_trait]
impl<'a, V> Dao<'a,V> for MongoDao<V>
where V: Entity<'a>
{
    async fn insert(&self, mut entity:V) -> anyhow::Result<V> {
        let result = self.coll.insert_one(&entity, None).await?;
        entity.set_id(result.inserted_id.as_object_id().unwrap().to_string());
        Ok(entity)
    }

    async fn find_by_code(&self, code: String) -> anyhow::Result<Option<V>> {
        let query = doc! {"task_code":code};
        let option = self.coll.find_one(query, None).await?;
        Ok(option)
    }

    async fn update_by_code(&self, mut entity:V) -> anyhow::Result<u64> {
        let (id_key,id_value) = entity.get_id();
        let value = serde_json::to_value(entity)?;
        let update_content = Self::value_to_document(value,vec!["".to_string(),id_key.clone()])?;
        let result = self.coll.update_one(doc! {id_key:id_value}, doc! {"$set":update_content}, None).await?;
        return Ok(result.modified_count)
    }
}