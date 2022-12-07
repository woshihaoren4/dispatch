mod task_allocation;
mod task_dispatch;

const MASTER_META: &'static str = "master_data";
const NODES_CLUSTER: &'static str = "nodes_cluster";

pub use task_allocation::Allocation;
pub use task_dispatch::TaskDispatch;
