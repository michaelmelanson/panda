use core::task::{Context, Poll, Waker};

use alloc::{collections::BTreeMap, sync::Arc};
use crossbeam_queue::ArrayQueue;

use super::{task::Task, task_id::TaskId, waker::TaskWaker};

pub struct Executor {
  tasks: BTreeMap<TaskId, Task>,
  task_queue: Arc<ArrayQueue<TaskId>>,
  waker_cache: BTreeMap<TaskId, Waker>
}

impl Executor {
  pub fn new() -> Executor {
    Executor { 
      tasks: BTreeMap::new(),
      task_queue: Arc::new(ArrayQueue::new(100)),
      waker_cache: BTreeMap::new()      
    }
  }

  pub fn spawn(&mut self, task: Task) {
    let task_id = task.id;

    if self.tasks.insert(task.id, task).is_some() {
      panic!("can't spawn a task that's already running");
    }

    self.task_queue.push(task_id).expect("task queue full");
  }

  fn run_ready_tasks(&mut self) {
    let Self { 
      tasks, 
      task_queue, 
      waker_cache 
    } = self;

    while let Ok(task_id) = task_queue.pop() {
      let task = match tasks.get_mut(&task_id) {
        Some(task) => task,
        None => continue
      };

      let waker = waker_cache
        .entry(task_id)
        .or_insert_with(|| TaskWaker::new(task_id, task_queue.clone()));
      
      let mut context = Context::from_waker(waker);
      match task.poll(&mut context) {
        Poll::Ready(()) => {
          tasks.remove(&task_id);
          waker_cache.remove(&task_id);
        },
        Poll::Pending => {}
      }
    }
  }

  pub fn run(&mut self) -> ! {
    loop {
      self.run_ready_tasks();
      self.sleep_if_idle();
    }
  }

  fn sleep_if_idle(&self) {
    use x86_64::instructions::interrupts;

    interrupts::disable();

    if self.task_queue.is_empty() {
      interrupts::enable_interrupts_and_hlt();
    } else {
      interrupts::enable();
    }
  }
}