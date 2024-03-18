use futures_timer::Delay;
use pin_project_lite::pin_project;
use std::future::Future;
use std::pin::Pin;
use std::task::{ready, Context, Poll};
use std::time::{Duration, Instant};

pin_project! {
    pub struct SleepDelay {
        dur: Duration,
        #[pin]
        sleep: Delay
    }
}

impl SleepDelay {
    pub fn new(dur: Duration) -> Self {
        Self {
            dur,
            sleep: Delay::new(dur),
        }
    }
}

impl Future for SleepDelay {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();
        ready!(this.sleep.as_mut().poll(cx));
        this.sleep.reset((Instant::now() + *this.dur).elapsed());
        Poll::Ready(())
    }
}
