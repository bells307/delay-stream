use futures::{FutureExt, Stream};
use std::collections::LinkedList;
use std::mem;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::time;
use tokio::time::Sleep;

pub struct DelayStream<T> {
    values: LinkedList<T>,
    dur: Duration,
    sleep: Option<Pin<Box<Sleep>>>,
}

impl<T> DelayStream<T> {
    pub fn new(values: LinkedList<T>, dur: Duration) -> Self {
        Self {
            values,
            dur,
            sleep: None,
        }
    }
}

impl<T: Unpin> Stream for DelayStream<T> {
    type Item = T;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.values.is_empty() {
            return Poll::Ready(None);
        };

        match &mut self.sleep {
            Some(sleep) => match sleep.poll_unpin(cx) {
                Poll::Ready(_) => {
                    mem::swap(&mut self.sleep, &mut None);
                    self.values
                        .pop_front()
                        .map(|v| Poll::Ready(Some(v)))
                        .unwrap_or(Poll::Ready(None))
                }
                Poll::Pending => Poll::Pending,
            },
            None => {
                let mut sleep = Box::pin(time::sleep(self.dur));
                let _ = sleep.poll_unpin(cx);
                mem::swap(&mut self.sleep, &mut Some(sleep));
                Poll::Pending
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;

    #[tokio::test]
    async fn it_works() {
        let stream = DelayStream::new(
            LinkedList::from([1, 3, 2, 4, 5]),
            Duration::from_millis(2000),
        );

        assert_eq!(
            stream.collect::<LinkedList<_>>().await,
            LinkedList::from([1, 3, 2, 4, 5])
        );
    }
}
