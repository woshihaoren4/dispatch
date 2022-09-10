#[derive(Debug,Clone,Serialize,Deserialize,Default)]
pub struct SubTask{
    pub sub_task_code : String, //task_code : String,
    pub sub_task_name : String,
    pub description : String,
    pub status : u8, //status : pb::TaskStatus,
    pub r#extern : String,
    pub tags : Vec<String>,
    pub create_time: i64,
    pub last_dispatch_time: i64,
    pub version: u32,
}