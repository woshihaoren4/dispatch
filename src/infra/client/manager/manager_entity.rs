use crate::infra::client::manager::Entity;
use crate::infra::client::manager::interface:: Dao;
use crate::infra::client::mongo::{MongoClient, MongoDao};
use crate::infra::client::{Cache, QueryOption, ShareCenter};
use crate::infra::client::redis::Redis;
use crate::infra::election::Election;

// pub struct ClientManagerBuild{
//     clients : HashMap<String,Arc<dyn Client>>
// }
//


#[derive(Default)]
pub struct DataSourceCenter {
    // clients : HashMap<String,Arc<dyn Client>>
    mongo : Option<MongoClient>,
    rds : Option<Redis>,
}

impl DataSourceCenter {
    pub fn new()->Self{
        DataSourceCenter::default()
    }
    pub fn register_mongo(mut self, mongo : MongoClient ) ->Self{
        self.mongo = Some(mongo);self
    }
    pub fn register_redis(mut self, rds : Redis)->Self{
        self.rds = Some(rds);self
    }

    // 拉取dao
    pub async fn get_dao< 'a, T:Entity<'a> + 'static>(&self) -> Box<dyn Dao<'a,T>>{
        if let Some(ref m) = self.mongo {
            let dao:MongoDao<T> = m.get_dao::<T>().await;
            return Box::new(dao);
        }
        return Box::new(DefaultDao);
    }
    pub fn get_election_impl<S:ToString>(&self,cluster:S)->impl Election{
        if let Some(ref rds) = self.rds{
            return rds.generate_election(cluster)
        }
        wd_log::log_panic!("get_election_impl not found any impl")
    }

    pub fn share_center(&self) ->Box<dyn ShareCenter>{
        if let Some(ref rds) = self.rds{
            return Box::new(rds.get_cache())
        }
        wd_log::log_panic!("get_election_impl not found any impl")
    }
}

pub struct DefaultDao;

#[async_trait::async_trait]
impl<'a, V> Dao<'a,V> for DefaultDao
    where for<'async_trait> V: Entity<'a>+ 'async_trait
{
    async fn insert(&self, _:V) -> anyhow::Result<V> {
        return Err(anyhow::anyhow!("DefaultDao"))
    }

    async fn find_by_code(&self, _code: String) -> anyhow::Result<Option<V>> {
        return Err(anyhow::anyhow!("DefaultDao"))
    }

    async fn update_by_code(&self, _: V) -> anyhow::Result<u64> {
        return Err(anyhow::anyhow!("DefaultDao"))
    }
    async fn insert_many(&self, _: Vec<V>)->anyhow::Result<Vec<V>>{
        return Err(anyhow::anyhow!("DefaultDao"))
    }
    async fn find(&self, _:Vec<(String, QueryOption)>,page:i64,size:i64) ->anyhow::Result<(Vec<V>,i64)>{
        return Err(anyhow::anyhow!("DefaultDao"))
    }
}
