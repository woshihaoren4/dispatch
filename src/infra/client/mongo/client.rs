use mongodb::Collection;
use mongodb::options::ClientOptions;
use crate::conf::MongoDb;
use crate::infra::client::manager::{ Dao, Entity};


pub struct MongoClient{
    cfg:MongoDb,
    client: mongodb::Client,
}

impl MongoClient {
    pub async fn new(app_name:String,cfg:MongoDb)->anyhow::Result<Self>{
        let mut opts = ClientOptions::parse(cfg.url.clone()).await?;
        opts.app_name = Some(app_name);
        opts.max_pool_size =opts.max_pool_size;
        let client = mongodb::Client::with_options(opts)?;
        let mc = MongoClient{
            cfg,client,
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
}