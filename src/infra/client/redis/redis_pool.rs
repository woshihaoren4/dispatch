pub struct Redis{
    pool:mobc::Pool<mobc_redis::RedisConnectionManager>
}

impl Redis {
    pub async fn new(cfg: crate::conf::Redis) -> anyhow::Result<Redis> {
        let client = redis::Client::open(cfg.url)?;
        let manager = mobc_redis::RedisConnectionManager::new(client);
        let pool = mobc::Pool::builder().max_open(cfg.max_conn_size).max_idle(cfg.max_idle_conn).build(manager);
        return Ok(Redis { pool })
    }
    pub fn generate_election<S:ToString>(&self,cluster:S)->super::RedisElection{
        super::RedisElection::new(cluster.to_string(),self.pool.clone())
    }
}
