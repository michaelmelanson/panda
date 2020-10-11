use core::{future::Future, pin::Pin, sync::atomic::{AtomicBool, AtomicUsize, Ordering}, task::{Context, Poll}};

use x86_64::structures::idt::InterruptStackFrame;
use futures_util::task::AtomicWaker;

use crate::pic;


pub struct IrqWaker {
  waker: AtomicWaker,
  flag: AtomicBool
}

impl IrqWaker {
  pub const fn new() -> Self {
      Self {
          waker: AtomicWaker::new(),
          flag: AtomicBool::new(false)
      }
  }

  pub fn register(&self, cx: &mut Context<'_>) {
      self.waker.register(cx.waker())
  }

  pub fn wake(&self) {
      self.flag.store(true, Ordering::Relaxed);
      self.waker.wake();
  }
}

impl Future for &IrqWaker {
  type Output = ();

  fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
      if self.flag.compare_and_swap(true, false, Ordering::Relaxed) {
          return Poll::Ready(());
      }

      self.waker.register(cx.waker());

      if self.flag.compare_and_swap(true, false, Ordering::Relaxed) {
          Poll::Ready(())
      } else {
          Poll::Pending
      }
  }
}

const NUMBER_OF_IRQS: usize = 255-32;

pub static IRQ_COUNT: [AtomicUsize; NUMBER_OF_IRQS] = [AtomicUsize::new(0); NUMBER_OF_IRQS];
pub static IRQ_WAKER: [IrqWaker; NUMBER_OF_IRQS] = [IrqWaker::new(); NUMBER_OF_IRQS];

pub extern "x86-interrupt" fn irq_handler(_stack_frame: &mut InterruptStackFrame) -> () {
  let irq = pic::isr() as usize;

  match irq {
    1 | 2 => {},
    _ => println!("IRQ {}", irq)
  }
  
  IRQ_COUNT[irq].fetch_add(1, Ordering::Relaxed);
  IRQ_WAKER[irq].wake();

  pic::notify_end_of_irq(irq as u8);
}

pub(crate) async fn wait_irq(irq: u8) {
  (&IRQ_WAKER[irq as usize]).await
}