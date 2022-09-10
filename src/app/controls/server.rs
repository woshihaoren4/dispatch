use std::sync::Arc;
use crate::infra::client::DataSourceCenter;
use crate::pb::TaskStatus;

pub struct Server {
    pub dsc : Arc<DataSourceCenter>
}

impl Server {
    pub fn new(dsc : Arc<DataSourceCenter>) -> Self {
        return Self {dsc};
    }
}

impl Server{
    pub fn fsm(old:TaskStatus,new:TaskStatus)->bool{
        match old {
            TaskStatus::Created => {
                match new {
                    TaskStatus::Initialized | TaskStatus::Close=>true,
                    _=>false,
                }
            }
            TaskStatus::Initialized => {
                match new {
                    TaskStatus::Close | TaskStatus::Stop=>true,
                    _=>false
                }
            }
            TaskStatus::Launching => {
                match new {
                    TaskStatus::Stop | TaskStatus::Over | TaskStatus::Close=>true,
                    _=>false
                }
            }
            TaskStatus::Stop => {
                match new {
                    TaskStatus::Initialized | TaskStatus::Close =>true,
                    _=>false
                }
            }
            TaskStatus::Over => false,
            TaskStatus::Close => false,
            TaskStatus::Keep => false,
        }
    }
}