pub mod counted;
pub mod interval;
pub mod sleep;

use crate::counted::Counted;
use crate::interval::IntervalDelay;
use crate::sleep::SleepDelay;
use futures::Future;
use futures::{ready, Stream};
use pin_project_lite::pin_project;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

/// Расширения для `Stream`, которые позволяют добавить ожидание по времени между отдачами элементов
///
/// **Внимание:** комбинаторы с задержкой времеи рекомендуется вызывать в конце, т.к. они могут не
/// успеть отработать
/// ```
/// use delay_stream::ThrottledStreamExt;
/// use futures::{stream, StreamExt};
/// use std::pin::pin;
/// use std::time::Duration;
///
/// #[tokio::main]
/// async fn main() {
///     let stream = stream::iter(vec![1, 3, 2, 4, 5])
///         .counted(3)
///         .interval_throttled(Duration::from_secs(1));
///
///     let mut stream = pin!(stream);
///
///     let mut count = 0;
///
///     while let Some(_) = stream.next().await {
///         // elapsed 1 sec ...
///         count += 1;
///     }
///
///     assert_eq!(count, 3);
/// }
/// ```
pub trait ThrottledStreamExt<S: Stream> {
    /// Получить определенное число элементов потока.
    ///

    fn counted(self, count: usize) -> ThrottledStream<S, Counted>;

    /// Ожидание полного промежутка времени `Duration` между выдачей элементов `Stream`'а
    fn sleep_throttled(self, dur: Duration) -> ThrottledStream<S, SleepDelay>;

    /// Интервальное ожидание между отдачами элемента. Это значит, что элементы будут выдаваться
    /// **не чаще**, чем указанный `Duration`.
    fn interval_throttled(self, dur: Duration) -> ThrottledStream<S, IntervalDelay>;
}

impl<S: Stream> ThrottledStreamExt<S> for S {
    fn counted(self, count: usize) -> ThrottledStream<S, Counted> {
        ThrottledStream::new(self, Counted::new(count))
    }

    fn sleep_throttled(self, dur: Duration) -> ThrottledStream<S, SleepDelay> {
        ThrottledStream::new(self, SleepDelay::new(dur))
    }

    fn interval_throttled(self, dur: Duration) -> ThrottledStream<S, IntervalDelay> {
        ThrottledStream::new(self, IntervalDelay::new(dur))
    }
}

pin_project! {
    /// `Stream` с задержкой получения элементов
    pub struct ThrottledStream<S: Stream, F: Future> {
        // Внутренний стрим
        #[pin]
        inner_stream: S,
        #[pin]
        // Future, которая будет вызываться в рамках опроса стрима
        inner_fut: F,
    }
}

impl<S: Stream, F: Future> ThrottledStream<S, F> {
    pub fn new(inner_stream: S, inner_fut: F) -> Self {
        Self {
            inner_stream,
            inner_fut,
        }
    }
}

/// Возвращаемый тип `bool` у `Future` говорит о том, нужно ли закончить опрос `Stream`'а
impl<S: Stream, F: Future<Output = bool>> Stream for ThrottledStream<S, F> {
    type Item = S::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();
        let stream_over = ready!(this.inner_fut.as_mut().poll(cx));
        if stream_over {
            Poll::Ready(None)
        } else {
            this.inner_stream.poll_next(cx)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ThrottledStreamExt;
    use futures::{stream, StreamExt};
    use std::pin::pin;

    #[tokio::test]
    async fn counted() {
        let stream = stream::iter(vec![1, 3, 2, 4, 5]).counted(4);
        let mut stream = pin!(stream);

        let mut count = 0;

        while let Some(_) = stream.next().await {
            count += 1;
        }

        assert_eq!(count, 4);
    }
}
