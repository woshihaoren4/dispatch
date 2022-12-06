use crate::infra::client::Entity;
use crate::infra::util;
use crate::pb::AppendSubTask;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubTask {
    pub sub_task_code: String, //task_code : String,
    pub sub_task_name: String,
    pub description: String,
    pub status: u8, //status : pb::TaskStatus,
    pub r#extern: String,
    pub create_time: i64,
    pub last_dispatch_time: i64,
    pub version: u32,
    pub task_code: String,
    pub partition: i32,
}

impl Entity<'_> for SubTask {
    fn bucket() -> String {
        "sub_task".to_string()
    }

    fn set_id(&mut self, id: String) {
        wd_log::log_info_ln!("entity::Task id callback:({})", id)
    }

    fn get_id(&mut self) -> (String, String) {
        return ("sub_task_code".to_string(), self.sub_task_code.clone());
    }
}

impl SubTask {
    pub fn from(ast: AppendSubTask, task_code: String) -> Vec<SubTask> {
        let mut sts = vec![];
        let utc = Utc::now().timestamp();
        let partition = util::rand_int32();
        for i in ast.sub_tasks.into_iter() {
            let st = SubTask {
                sub_task_code: util::sony_flake_id().to_string(),
                sub_task_name: i.sub_task_name,
                description: i.description,
                status: 1,
                r#extern: i.extern_,
                create_time: utc,
                last_dispatch_time: 0,
                version: 0,
                task_code: task_code.clone(),
                partition,
            };
            sts.push(st);
        }
        return sts;
    }
}

// impl From<(pb::AppendSubTask,task_code:String)> for SubTask {
//     fn from(req : pb::AppendSubTask) -> Self {
//         let utc = Utc::now().timestamp();
//         let t = SubTask{
//             sub_task_code: "".to_string(),
//             sub_task_name: "".to_string(),
//             description: "".to_string(),
//             status: 0,
//             r#extern: "".to_string(),
//             tags: vec![],
//             create_time: 0,
//             last_dispatch_time: 0,
//             version: 0,
//             task_code: "".to_string()
//         };
//         return t;
//     }
// }
