pub mod task;

use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use futures_util::task::AtomicWaker;

static SCANCODE_QUEUE: OnceCell<ArrayQueue<Scancode>> = OnceCell::uninit();
static WAKER: AtomicWaker = AtomicWaker::new();

#[derive(Debug)]
pub struct Scancode(u8);
