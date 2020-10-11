mod executor;
mod task;
mod task_id;
mod waker;

use core::future::Future;
use spin::{RwLock, RwLockWriteGuard};
use conquer_once::spin::OnceCell;

pub use executor::Executor;
pub use task::Task;
pub use task_id::TaskId;

pub fn init() -> Executor {
  let executor = Executor::new();

  executor
}
