use serde::{Deserialize, Serialize};
use std::path::Path;

macro_rules! field_generate {
    ($cfg:tt; $($name:tt,$ty:ty,$default:expr,$default_desc:tt);*) => {
        #[derive(Debug,Serialize,Deserialize,Clone)]
        pub struct $cfg{
            $(
            #[serde(default=$default_desc)]
            pub $name : $ty,
            )*

        }
        impl $cfg{
            $(
            fn $name()->$ty{
                $default
            }
            )*
        }
        impl Default for $cfg{
            fn default() -> Self {
                Self{
                $(
                    $name : $default,
                )*
                }
            }
        }
    };
}

field_generate!(Server;
    name,String,String::from("dispatch"),"Server::name";
    host_port,String,String::from("0.0.0.0:670"),"Server::host_port");

field_generate!(MongoDb;
    url,String,String::from("mongodb://dispatch_admin:1443965173@10.37.129.190:27019/dispatch"),"MongoDb::url";
    max_conn_size,u32,20u32,"MongoDb::max_conn_size");

field_generate!(Redis;
    url,String,String::from("redis://:passwd@10.37.129.190:6379/0"),"Redis::url";
    max_conn_size,u64,20u64,"Redis::max_conn_size";
    max_idle_conn,u64,1u64,"Redis::max_idle_conn");

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum DataSourceDriver {
    Mysql,
    Postgresql,
    Mongo(MongoDb),
}

field_generate!(DataSource;
    driver,DataSourceDriver,DataSourceDriver::Mongo(MongoDb::default()),"DataSource::driver");

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(default = "Server::default")]
    pub server: Server,
    #[serde(default = "DataSource::default")]
    pub data_source: DataSource,
    #[serde(default = "Redis::default")]
    pub cache: Redis,
}

impl Config {
    pub fn from_file_by_path(path: impl AsRef<Path>) -> anyhow::Result<Config> {
        match wd_run::load_config(path) {
            Err(e) => return Err(anyhow::anyhow!(e)),
            Ok(o) => Ok(o),
        }
    }
}

impl ToString for Config {
    fn to_string(&self) -> String {
        match serde_json::to_string(self) {
            Ok(o) => o,
            Err(e) => e.to_string(),
        }
    }
}
