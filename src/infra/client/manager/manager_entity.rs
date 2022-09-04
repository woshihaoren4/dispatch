use crate::infra::client::manager::Entity;
use crate::infra::client::manager::interface:: Dao;
use crate::infra::client::mongo::{MongoClient, MongoDao};

// pub struct ClientManagerBuild{
//     clients : HashMap<String,Arc<dyn Client>>
// }
//


#[derive(Default)]
pub struct DataSourceCenter {
    // clients : HashMap<String,Arc<dyn Client>>
    mongo : Option<MongoClient>,
}

impl DataSourceCenter {
    pub fn new()->Self{
        DataSourceCenter::default()
    }
    pub fn register_mongo(mut self, mongo : MongoClient ) ->Self{
        self.mongo = Some(mongo);self
    }


    // 拉取dao
    pub async fn get_dao< 'a, T:Entity<'a> + 'static,D:Dao<'a,T>>(&self) -> Box<dyn Dao<'a,T>>{
        if let Some(ref m) = self.mongo {
            let dao:MongoDao<T> = m.get_dao::<T>().await;
            return Box::new(dao);
        }
        return Box::new(DefaultDao);
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
}
