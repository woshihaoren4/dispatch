use async_trait::async_trait;
use std::future::Future;

pub struct Pool {
    cfg: PoolConfig,
}
#[derive(Default, Clone)]
pub struct PoolConfig {
    pub idle: i32,
    pub max: i32,
}

impl Pool {
    pub fn new() -> Self {
        let cfg = PoolConfig::default();
        Self { cfg }
    }
}

#[async_trait::async_trait]
impl super::PoolInterface for Pool {
    async fn push<Out, F: Future<Output = anyhow::Result<Out>> + Send>(
        &self,
        handle: F,
    ) -> anyhow::Result<Out> {
        handle.await
    }
}
