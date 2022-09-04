use crate::pb;

pub struct Task{
    task_code : String,
    task_name : String,
    description : String,
    start_time : i64,
    end_time : i64,
    r#type : pb::TaskType,
    status : pb::TaskStatus,
    config : String,
    tags : Vec<String>
}

// string task_code = 1;
// string task_name = 2;
// string description = 3; //描述
// int64 start_time = 4;  //utc 开始时间
// int64 end_time = 5; //utc 结束时间
// TaskType type = 6;
// TaskStatus status = 7;
// string config = 8; //任务触发的时候会传递给执行者
// repeated string tags = 9;
// DispatchPolicy policy = 10;
//
// repeated SubTask sub_tasks = 100; //子任务 被调度的单元