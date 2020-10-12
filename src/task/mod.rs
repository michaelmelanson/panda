mod executor;
mod task;
mod task_id;
mod waker;

pub use executor::Executor;
pub use task::Task;
pub use task_id::TaskId;

pub fn init() -> Executor {
    let executor = Executor::new();

    executor
}
