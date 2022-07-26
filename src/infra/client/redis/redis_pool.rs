use crate::infra::client::{Node, ShareCenter};
use redis::{AsyncCommands, AsyncIter};
use std::collections::HashMap;
use std::i32;

pub struct Redis {
    pool: mobc::Pool<mobc_redis::RedisConnectionManager>,
}

impl Redis {
    pub async fn new(cfg: crate::conf::Redis) -> anyhow::Result<Redis> {
        let client = redis::Client::open(cfg.url)?;
        let manager = mobc_redis::RedisConnectionManager::new(client);
        let pool = mobc::Pool::builder()
            .max_open(cfg.max_conn_size)
            .max_idle(cfg.max_idle_conn)
            .build(manager);
        return Ok(Redis { pool });
    }
    pub fn generate_election<S: ToString>(&self, cluster: S) -> super::RedisElection {
        super::RedisElection::new(cluster.to_string(), self.pool.clone())
    }
    pub fn get_cache(&self) -> RedisCache {
        let pool = self.pool.clone();
        RedisCache { pool }
    }
}

pub struct RedisCache {
    pool: mobc::Pool<mobc_redis::RedisConnectionManager>,
}

#[async_trait::async_trait]
impl ShareCenter for RedisCache {
    async fn version(&self, key: String) -> anyhow::Result<Option<i64>> {
        let mut conn = self.pool.get().await?;
        let version: Option<i64> = conn.hget(key, "version").await?;
        Ok(version)
    }

    async fn set_version(&self, key: String, version: i64) -> anyhow::Result<()> {
        let mut conn = self.pool.get().await?;
        let _: () = conn.hset(key, "version", version).await?;
        Ok(())
    }

    async fn nodes(&self, node_cluster: String) -> anyhow::Result<Vec<String>> {
        let mut conn = self.pool.get().await?;
        let mut iter: AsyncIter<String> = conn.sscan(node_cluster).await?;
        let mut list = vec![];
        while let Some(element) = iter.next_item().await {
            list.push(element);
        }
        return Ok(list);
    }

    async fn get_node(&self, key: String) -> anyhow::Result<Option<Node>> {
        let mut conn = self.pool.get().await?;
        let map: Option<HashMap<String, String>> = conn.hgetall(key).await?;
        let map = match map {
            None => return Ok(None),
            Some(s) => {
                if s.is_empty() {
                    return Ok(None);
                }
                s
            }
        };
        let mut node = Node::default();
        node.max = map.get("max").unwrap_or(&"None".to_string()).parse()?;
        node.min = map.get("min").unwrap_or(&"None".to_string()).parse()?;
        node.version = map.get("version").unwrap_or(&"None".to_string()).parse()?;
        return Ok(Some(node));
    }

    async fn add_node(&self, key: String, node: Node) -> anyhow::Result<()> {
        let mut conn = self.pool.get().await?;
        let mut map = vec![];
        map.push(("min", node.min.to_string()));
        map.push(("max", node.max.to_string()));
        map.push(("version", node.version.to_string()));
        let _: () = conn.hset_multiple(key, map.as_slice()).await?;
        Ok(())
    }

    async fn register_node(&self, cluster: String, key: String) -> anyhow::Result<()> {
        let mut conn = self.pool.get().await?;
        let _: () = conn.sadd(cluster, key).await?;
        Ok(())
    }

    async fn del_node(&self, cluster: String, key: String) -> anyhow::Result<()> {
        let mut conn = self.pool.get().await?;
        let _: () = conn.del(key.clone()).await?;
        let _: Option<i32> = conn.srem(cluster, key).await?;
        Ok(())
    }

    // async fn set(& self, key: String, value: String, seconds:usize) -> anyhow::Result<()> {
    //     let mut conn = self.pool.get().await?;
    //     conn.set_ex(key, value, seconds).await?;Ok(())
    // }
    //
    // async fn get(&self, key: String) -> anyhow::Result<String> {
    //     let mut conn = self.pool.get().await?;
    //     let s:String = conn.get(key).await?;Ok(s)
    // }
}
