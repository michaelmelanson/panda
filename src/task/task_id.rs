use core::sync::atomic::{AtomicU64, Ordering};

#[derive(Copy, Clone, PartialOrd, PartialEq, Eq, Ord)]
pub struct TaskId(u64);

impl TaskId {
  pub fn new() -> TaskId {
    static NEXT_ID: AtomicU64 = AtomicU64::new(0);
    TaskId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
  }
}