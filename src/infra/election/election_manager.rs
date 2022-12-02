use std::sync::{Arc, Mutex};
use std::sync::atomic::{ AtomicU8, Ordering};
use super::{Election, MasterAndWorker};

pub struct ElectionManager{
    status : Arc<AtomicU8>, // 状态：
    name : String, //当前节点名字
    master : Arc<Mutex<String>>, //主节点名字
    election : Arc<dyn Election>, //选举器
    master_is_worker: bool, //是否允许主节点工作
    handler : Arc<dyn MasterAndWorker>, //调用函数
    ele_interval: std::time::Duration,
}

pub struct ElectionManagerClose{
    status : Arc<AtomicU8>,
}

impl ElectionManagerClose {
    fn new(status:Arc<AtomicU8>)->Self{
        Self{status}
    }
    pub async fn stop(&self,timeout:std::time::Duration)->anyhow::Result<()>{
        let status = self.status.clone();
        let closing = status.load(Ordering::Relaxed);
        status.fetch_add(1,Ordering::Relaxed);
        let result = tokio::time::timeout(timeout, async move {
            while status.load(Ordering::Relaxed) == closing+1 {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        }).await;
        return match result {
            Ok(s) => Ok(s),
            Err(e) => Err(anyhow::anyhow!("close election manager error:{}",e.to_string()))
        }
    }
    pub fn sync_stop(&self){
        self.status.fetch_add(1,Ordering::Relaxed);
        wd_log::log_info_ln!("election stop");
    }
}

impl Drop for ElectionManagerClose {
    fn drop(&mut self) {
        self.sync_stop();
    }
}

impl ElectionManager {
    pub fn build<E,M>(e:E,m:M)->Self
    where E:Election+ 'static,
    M: MasterAndWorker + 'static,
    {
        let status = Arc::new(AtomicU8::new(0));
        let name = m.name();
        let master = Arc::new(Mutex::new(String::new()));
        let election = Arc::new(e);
        let handler = Arc::new(m);
        let ele_interval = std::time::Duration::from_secs(60*3);

        Self{
            status,
            name,
            master,
            election,
            master_is_worker: false,
            handler,
            ele_interval,
        }
    }

    pub fn set_master_is_worker(mut self)->Self{
        self.master_is_worker = true;self
    }
    pub fn set_name(mut self,name:String)->Self{
        self.name = name;self
    }
    pub fn set_ele_interval(mut self,interval:std::time::Duration)->Self{
        self.ele_interval = interval;self
    }

    pub fn get_status(&self)->bool{
        self.status.load(Ordering::Relaxed) == 0
    }

    fn set_master_name(&self,name:String)->String{
        let mut master = self.master.lock();
        let master_name = master.as_deref().unwrap().clone();
        *(master.as_deref_mut().unwrap()) = name.clone();
        return master_name
    }

    async fn election_result_handle(&self,name:String){
        let master_name = self.set_master_name(name.clone());
        if name.eq(&self.name) && !name.eq(&master_name) { //当选主节点
            if !self.master_is_worker {
                self.handler.worker_stop().await;
            }
            self.handler.master_start().await;
        }else if !name.eq(&self.name) && self.name.eq(&master_name) { //当前节点不再是主节点
            self.handler.master_stop().await;
            if !self.master_is_worker {
                self.handler.worker_start().await;
            }
        }
        if !name.eq(&master_name) {
            self.handler.master_winners(name).await;
        }
    }

    pub async fn close(&self){
        self.handler.worker_stop().await;
        self.handler.master_stop().await;
    }

    pub async fn start(self)->ElectionManagerClose{
        let status = (&self).status.clone();
        let em = Arc::new(self);
        let iem = em.clone();
        let sem = em.clone();
        //初始化
        if em.master_is_worker {
            tokio::spawn( async move{
                iem.handler.worker_start().await;
            });
        }
        //开始状态管理
        tokio::spawn(async move{ //等待任务结束
            while sem.get_status() {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
            sem.close().await;
            sem.status.fetch_add(1,Ordering::Relaxed);
        });
        //开启不断选举
        tokio::spawn(async move{ //不断进行选举
            while em.get_status() {
                let result = em.election.initiate_election(em.name.clone()).await;
                match result {
                    Ok(s) => {
                        wd_log::log_debug_ln!("initiate election, master:{}",s);
                        em.election_result_handle(s).await;
                    },
                    Err(e) => {
                        wd_log::log_error_ln!("initiate election error:{}",e);
                    }
                }
                tokio::time::sleep(em.ele_interval).await;
            }
        });
        return ElectionManagerClose::new(status);
    }
}