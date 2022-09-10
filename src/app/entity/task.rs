use chrono::Utc;
use crate::infra::*;
use crate::pb;
use serde::{Serialize,Deserialize};
use crate::infra::client::Entity;
use crate::pb::TaskStatus;

#[derive(Debug,Clone,Serialize,Deserialize,Default)]
pub struct Task{
    pub task_code : String, //task_code : String,
    pub task_name : String,
    pub description : String,
    pub start_time : i64,
    pub end_time : i64,
    pub r#type : u8,   //r#type : pb::TaskType,
    pub status : u8, //status : pb::TaskStatus,
    pub config : String,
    pub tags : Vec<String>,
    pub create_time: i64
}

impl Task {
    pub fn number_to_task_status(n:u8)->TaskStatus{
        match n {
            1=>TaskStatus::Created,
            2=>TaskStatus::Initialized,
            3=>TaskStatus::Launching,
            4=>TaskStatus::Stop,
            5=>TaskStatus::Over,
            6=>TaskStatus::Close,
            _=>TaskStatus::Keep,
        }
    }
}

impl Entity<'_> for Task {
    fn bucket() -> String {
        "task".to_string()
    }

    fn set_id(&mut self, id: String) {
        wd_log::log_info_ln!("entity::Task id callback:({})",id)
    }

    fn get_id(&mut self) -> (String, String) {
        return ("task_code".to_string(),self.task_code.clone())
    }
}

impl From<pb::CreateTaskRequest> for Task {
    fn from(req : pb::CreateTaskRequest) -> Self {
        let utc = Utc::now().timestamp();
        let t = Task{
            task_code: util::sony_flake_id().to_string(),
            task_name: req.task_name,
            description: req.description,
            r#type: req.r#type as u8,
            config: req.config,
            start_time: req.start_time,
            end_time: req.end_time,
            create_time: utc,
            tags: req.tags,
            status: 1, //状态默认是1
            ..Task::default()
        };
        return t;
    }
}

// impl Drop for Task {
//     fn drop(&mut self) {
//         wd_log::log_info_ln!("drop----------------------> entity:Task");
//     }
// }

#[cfg(test)]
mod test{

    #[test]
    fn test_utc_time(){
        let utc = chrono::offset::Utc::now();
        println!("{}", utc.format("%Y年%m月%d日 %H:%M:%S"));
        println!("{}", utc.timestamp());
    }
}