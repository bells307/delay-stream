use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::task::{Context, Poll};

pub struct Counted {
    count: usize,
    polled: AtomicUsize,
}

impl Counted {
    pub fn new(count: usize) -> Self {
        Self {
            count,
            polled: AtomicUsize::new(0),
        }
    }
}

impl Future for Counted {
    type Output = bool;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.polled.fetch_add(1, Ordering::SeqCst);
        if self.polled.load(Ordering::SeqCst) <= self.count {
            Poll::Ready(false)
        } else {
            Poll::Ready(true)
        }
    }
}
