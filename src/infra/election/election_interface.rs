pub trait Node {
    fn name(&self) -> String;
    // {
    //     util::sony_flake_id().to_string()//fixme 返回一个id地址
    // }
}

#[async_trait::async_trait]
pub trait MasterAndWorker: Node + Send + Sync {
    async fn master_start(&self) { //当选时执行
    }
    async fn master_stop(&self) { //任期结束执行
    }
    async fn worker_start(&self) { //工作节点开始工作，默认主节点不参与工作
    }
    async fn worker_stop(&self) { //工作节点结束工作
    }
    async fn master_winners(&self, node: String) {
        //当选者变更的时候执行
        wd_log::log_debug_ln!("node[{}]  elected the master", node)
    }
}

#[async_trait::async_trait]
pub trait Election: Send + Sync {
    async fn initiate_election(&self, node: String) -> anyhow::Result<String>; //返回主节点的名字
}
