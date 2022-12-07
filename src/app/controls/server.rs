use crate::app::schedule::Allocation;
use crate::infra::client::DataSourceCenter;
use crate::pb::TaskStatus;
use std::sync::Arc;

#[derive(Clone)]
pub struct Server {
    pub dsc: Arc<DataSourceCenter>,
    alloc: Arc<Allocation>,
}

impl Server {
    pub fn new(dsc: Arc<DataSourceCenter>, alloc: Arc<Allocation>) -> Self {
        return Self { dsc, alloc };
    }
}

impl Server {
    pub fn fsm(old: TaskStatus, new: TaskStatus) -> bool {
        match old {
            TaskStatus::Created => match new {
                TaskStatus::Initialized | TaskStatus::Close => true,
                _ => false,
            },
            TaskStatus::Initialized => match new {
                TaskStatus::Close | TaskStatus::Stop => true,
                _ => false,
            },
            TaskStatus::Launching => match new {
                TaskStatus::Stop | TaskStatus::Over | TaskStatus::Close => true,
                _ => false,
            },
            TaskStatus::Stop => match new {
                TaskStatus::Initialized | TaskStatus::Close => true,
                _ => false,
            },
            TaskStatus::Over => false,
            TaskStatus::Close => false,
            TaskStatus::Keep => false,
        }
    }
}
