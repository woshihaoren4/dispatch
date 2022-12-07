use crate::app::schedule::{MASTER_META, NODES_CLUSTER};
use crate::infra::client::{DataSourceCenter, Node as ShareNode};
use crate::infra::election::{MasterAndWorker, Node};
use crate::infra::util;
use std::sync::atomic::{AtomicI8, Ordering};
use std::sync::Arc;
use std::time::Duration;

#[derive(Clone)]
pub struct TaskDispatch {
    pub alloc: Arc<super::Allocation>,
    node_name: String,
    dsc: Arc<DataSourceCenter>,
    master_status: Arc<AtomicI8>,
    worker_status: Arc<AtomicI8>,
}

impl Node for TaskDispatch {
    fn name(&self) -> String {
        return self.node_name.clone();
    }
}

impl TaskDispatch {
    pub fn new(dsc: Arc<DataSourceCenter>) -> TaskDispatch {
        let node_name = util::sony_flake_id().to_string();
        let alloc = Arc::new(super::Allocation::default());
        TaskDispatch {
            alloc,
            node_name,
            dsc,
            master_status: Arc::new(Default::default()),
            worker_status: Arc::new(Default::default()),
        }
    }
    pub fn listen(self) -> TaskDispatch {
        let dsc = self.dsc.clone();
        let master_status = self.master_status.clone();
        tokio::spawn(async move {
            Self::start_master_listen(dsc, master_status).await;
        });
        let dsc = self.dsc.clone();
        let worker_status = self.worker_status.clone();
        let name = self.name();
        let alloc = self.alloc.clone();
        tokio::spawn(async move {
            Self::start_worker_listen(dsc, worker_status, name, alloc).await;
        });
        return self;
    }
}

//做业
impl TaskDispatch {
    pub async fn start_master_listen(dsc: Arc<DataSourceCenter>, master_status: Arc<AtomicI8>) {
        //todo 初始化主节点信息
        let mut last_nodes: Vec<String> = vec![];
        loop {
            tokio::time::sleep(Duration::from_secs(3)).await;
            if master_status.load(Ordering::Relaxed) != 1 {
                continue;
            }
            let share = dsc.share_center();
            let last_version = match share.version(MASTER_META.to_string()).await {
                Ok(v) => v,
                Err(e) => {
                    wd_log::log_error_ln!("master:从共享中心中拉取最新版本信息失败 {}", e);
                    continue;
                }
            };
            let last_version = match last_version {
                None => {
                    //需要尝试初始化主节点
                    match share.set_version(MASTER_META.to_string(), 1).await {
                        Ok(_) => {
                            wd_log::log_info_ln!("主版本信息初始化成功 version={}", 1);
                            1
                        }
                        Err(e) => {
                            wd_log::log_info_ln!("主版本信息初始化失败 version={} error={}", 1, e);
                            continue;
                        }
                    }
                }
                Some(s) => s,
            };
            let nodes = match share.nodes(NODES_CLUSTER.to_string()).await {
                Ok(ns) => ns,
                Err(e) => {
                    wd_log::log_error_ln!("从共享中拉取节点列表失败 {}", e);
                    continue;
                }
            };
            let mut deal_nodes = vec![];
            let mut active_nodes = vec![];
            let mut need_rebalance = false;
            //判断节点是否存活 相差60个版本视为节点死亡
            for i in nodes.into_iter() {
                let node = match share.get_node(i.to_string()).await {
                    Ok(n) => n,
                    Err(e) => {
                        wd_log::log_error_ln!("从共享中拉取节点信息失败 {}", e);
                        continue;
                    }
                };
                // wd_log::log_debug_ln!("拉取节点信息：{:?}",node);
                let node = match node {
                    None => {
                        //新加入的节点
                        need_rebalance = true;
                        active_nodes.push(i);
                        continue;
                    }
                    Some(n) => n,
                };
                if node.version < last_version - 10 {
                    //节点死亡
                    deal_nodes.push(i);
                } else {
                    active_nodes.push(i);
                }
            }
            //如果有节点死亡 则重分配节点的任务区间 并删除死亡节点
            for i in deal_nodes.into_iter() {
                need_rebalance = true;
                match share
                    .del_node(NODES_CLUSTER.to_string(), i.to_string())
                    .await
                {
                    Ok(_) => wd_log::log_info_ln!("删除死亡节点信息成功 node={}", i),
                    Err(e) => wd_log::log_info_ln!("删除死亡节点信息失败 node={} error={}", i, e),
                }
            }
            //是否是首次分配节点
            // if last_nodes.is_empty() && ! active_nodes.is_empty() {
            //     need_rebalance = true
            // }
            //是否新增节点
            'have_new_node: for i in active_nodes.iter() {
                for j in last_nodes.iter() {
                    if i.eq(j) {
                        continue 'have_new_node;
                    }
                }
                need_rebalance = true;
                break;
            }
            let mut rebalance_err_node = vec![];
            //todo 重新分配节点空间 待抽成接口 应实现多种不同算法
            if need_rebalance {
                wd_log::log_debug_ln!("重分配工作空间：{:?}", active_nodes);
                let total = active_nodes.len() as i32;

                for (i, k) in active_nodes.iter().enumerate().map(|(x, y)| (x as i32, y)) {
                    let node = ShareNode {
                        min: i * (i32::MAX / total), //向左闭合 a>= min a<max
                        max: (i + 1) * (i32::MAX / total),
                        version: last_version,
                    };
                    match share.add_node(k.to_string(), node).await {
                        Ok(_) => {}
                        Err(e) => {
                            wd_log::log_error_ln!(
                                "重分配节点区间时，设置节点信息失败，node={} error={}",
                                k,
                                e
                            );
                            rebalance_err_node.push(k.to_string()); //加入失败节点 下次循环重新分配
                        }
                    }
                }
            }
            //主版本号+1
            'add_mater_version: for i in 0..3 {
                match share
                    .set_version(MASTER_META.to_string(), last_version + 1)
                    .await
                {
                    Ok(_) => {
                        // wd_log::log_debug_ln!("主版本信息设置成功 version={}",last_version+1);
                        break 'add_mater_version;
                    }
                    Err(e) => {
                        wd_log::log_error_ln!(
                            "主版本信息设置失败 version={} error={}",
                            last_version + 1,
                            e
                        );
                    }
                }
                tokio::time::sleep(Duration::from_secs(i)).await;
            }
            //记录最新的节点
            last_nodes.clear();
            'reset: for i in active_nodes.into_iter() {
                for j in rebalance_err_node.iter() {
                    if i.eq(j) {
                        continue 'reset;
                    }
                }
                last_nodes.push(i);
            }
        }
    }
    pub async fn start_worker_listen(
        dsc: Arc<DataSourceCenter>,
        worker_status: Arc<AtomicI8>,
        name: String,
        alloc: Arc<super::Allocation>,
    ) {
        loop {
            tokio::time::sleep(Duration::from_secs(3)).await;
            if worker_status.load(Ordering::Relaxed) != 1 {
                continue;
            }
            //拉取信息
            let share = dsc.share_center();
            let node = match share.get_node(name.clone()).await {
                Ok(node) => node,
                Err(e) => {
                    wd_log::log_error_ln!("拉取节点信息失败 name={} error={}", name, e);
                    continue;
                }
            };
            let node = match node {
                None => {
                    //没有拉取到节点信息
                    if let Err(e) = share
                        .register_node(NODES_CLUSTER.to_string(), name.clone())
                        .await
                    {
                        wd_log::log_error_ln!(
                            "将节点注册到集群列表中失败 name={} error={}",
                            name,
                            e
                        );
                    }
                    continue;
                }
                Some(n) => n,
            };
            let last_version = match share.version(MASTER_META.to_string()).await {
                Ok(v) => v,
                Err(e) => {
                    wd_log::log_error_ln!("从共享中心中拉取最新版本信息失败 {}", e);
                    continue;
                }
            };
            let last_version = match last_version {
                None => {
                    wd_log::log_info_ln!("等待主节点初始化集群信息");
                    continue;
                }
                Some(s) => s,
            };
            if node.version < last_version - 60 {
                //小于60个版本就认为过期了 重新注册一下
                if let Err(e) = share
                    .register_node(NODES_CLUSTER.to_string(), name.clone())
                    .await
                {
                    wd_log::log_error_ln!("将节点注册到集群列表中失败 name={} error={}", name, e);
                }
                continue;
            }
            //更新版本信息
            if let Err(e) = share.set_version(name.clone(), last_version).await {
                wd_log::log_error_ln!("更新节点版本信息失败 name={} error={}", name, e);
            }
            //todo 需要抽成接口方便扩展
            alloc.set_scope(node.min, node.max);
            // wd_log::log_info_ln!("从节点作业范围：{}-{}", node.min, node.max);
        }
    }
}

#[async_trait::async_trait]
impl MasterAndWorker for TaskDispatch {
    async fn master_start(&self) {
        self.master_status.store(1, Ordering::Relaxed);
        wd_log::log_info_ln!("当前节点[{}] 竞选成功", self.name());
    }

    async fn master_stop(&self) {
        self.master_status.store(2, Ordering::Relaxed);
        wd_log::log_info_ln!("当前节点[{}] 竞选失败", self.name());
    }

    async fn worker_start(&self) {
        //循环将主节点版本号添加到自身节点信息中
        //根据自身节点列表 不断将任务发射出去
        self.worker_status.store(1, Ordering::Relaxed);
        wd_log::log_info_ln!("当前节点[{}] 开始工作", self.name());
    }

    async fn worker_stop(&self) {
        self.worker_status.store(2, Ordering::Relaxed);
        wd_log::log_info_ln!("当前节点[{}] 停止工作", self.name());
    }
}
