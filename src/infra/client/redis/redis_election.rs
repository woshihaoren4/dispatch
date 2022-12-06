use crate::infra::election::Election;
use redis::{AsyncCommands, RedisResult};

pub struct RedisElection {
    cluster: String,
    pool: mobc::Pool<mobc_redis::RedisConnectionManager>,
    term: usize,
}

impl RedisElection {
    pub fn new(cluster: String, pool: mobc::Pool<mobc_redis::RedisConnectionManager>) -> Self {
        let term = 120usize;
        RedisElection {
            cluster,
            pool,
            term,
        }
    }
    #[allow(dead_code)]
    pub fn set_term_interval(mut self, sec: usize) -> Self {
        self.term = sec;
        self
    }
}

#[async_trait::async_trait]
impl Election for RedisElection {
    async fn initiate_election(&self, node: String) -> anyhow::Result<String> {
        let mut conn = self.pool.get().await?;
        let master: Option<String> = conn.get(&self.cluster).await?;

        if let Some(s) = master {
            let _: () = conn.set_ex(&self.cluster, node, self.term).await?;
            return Ok(s);
        }
        //没有主节点 开始设置自己为主节点
        let mut conn = conn.into_inner();
        let result = redis::cmd("set")
            .arg(format!("{}-lock", &self.cluster))
            .arg(&node)
            .arg("ex")
            .arg(5)
            .arg("nx")
            .query_async::<_, Option<String>>(&mut conn)
            .await?;
        if result.is_none() {
            //有节点选过了，等一会重新拿一次
            let master: String = conn.get(&self.cluster).await?;
            return Ok(master);
        }
        let result: RedisResult<()> = conn.set_ex(&self.cluster, &node, self.term).await;
        let lock: String = conn.get(format!("{}-lock", &self.cluster)).await?;
        if lock.eq(&node) {
            let _: () = conn.del(format!("{}-lock", &self.cluster)).await?;
        }
        return match result {
            Ok(_) => Ok(node),
            Err(e) => Err(anyhow::anyhow!("redis election error:{}", e.to_string())),
        };
    }
}

#[cfg(test)]
mod test {
    use crate::infra::election::Election;

    #[tokio::test]
    async fn test_election() {
        let cfg = crate::conf::Redis::default();
        let client = super::super::Redis::new(cfg).await.unwrap();
        let client = client.generate_election("test-cluster");
        let result = client.initiate_election("node1".to_string()).await;
        match result {
            Ok(n) => {
                println!("node -> {}", n)
            }
            Err(e) => {
                println!("error -> {}", e)
            }
        }
    }
}
