use super::Pool;
use super::PoolConfig;
use std::future::Future;
#[async_trait::async_trait]
pub trait PoolInterface {
    // async fn entity(&self) -> PoolConfig;
    // fn push<Out, F: FnOnce<(), Output = Out> + Send>(&self, handle: F)
    //     -> anyhow::Result<Out>;
    async fn push<Out, F: Future<Output = anyhow::Result<Out>> + Send>(
        &self,
        handle: F,
    ) -> anyhow::Result<Out>;
}
