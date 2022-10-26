use std::sync::Arc;
use crate::infra::client::DataSourceCenter;
use crate::infra::election::{MasterAndWorker, Node};

pub struct TaskDispatch{
    dsc : Arc<DataSourceCenter>,
}

impl Node for TaskDispatch {}

#[async_trait::async_trait]
impl MasterAndWorker for TaskDispatch {
    async fn master_start(&self) {
        //1 捞出所有需要被调度的任务
        //2 找到所有活跃的节点
        //3 将任务分配给这些节点
    }

    async fn master_stop(&self) {
        todo!()
    }

    async fn worker_start(&self) {
        //1 设置自身为活跃节点
        //2 或者自身对应的任务
        //3 将任务分配到缓存中
    }

    async fn worker_stop(&self) {
        todo!()
    }
}