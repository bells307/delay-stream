pub mod interval;
pub mod sleep;

use crate::interval::IntervalDelay;
use crate::sleep::SleepDelay;
use futures::Future;
use futures::{ready, Stream};
use pin_project_lite::pin_project;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

/// Расширения для `Stream`, которые позволяют добавить ожидание по времени между отдачами элементов
/// ```
/// use delay_stream::DelayedStreamExt;
/// use futures::{stream, StreamExt};
/// use std::pin::pin;
/// use std::time::Duration;
///
/// #[tokio::test]
/// async fn sleep_delayed() {
///     let stream = stream::iter(vec![1, 3, 2, 4, 5]).sleep_delayed(Duration::from_secs(1));
///     let mut stream = pin!(stream);
///
///     while let Some(val) = stream.next().await {
///         // spent 1 sec ...
///         println!("{}", val);
///     }
/// }
///
/// #[tokio::test]
/// async fn interval_delayed() {
///     let stream = stream::iter(vec![1, 3, 2, 4, 5]).interval(Duration::from_secs(1));
///     let mut stream = pin!(stream);
///
///     while let Some(val) = stream.next().await {
///         // spent 1 sec ...
///         println!("{}", val);
///     }
/// }
/// ```
pub trait DelayedStreamExt<S: Stream> {
    /// Ожидание полного промежутка времени `Duration` между выдачей элементов `Stream`'а
    fn sleep(self, dur: Duration) -> DelayStream<S, SleepDelay>;

    /// Интервальное ожидание между отдачами элемента. Это значит, что элементы будут выдаваться
    /// **не чаще**, чем указанный `Duration`.
    fn interval(self, dur: Duration) -> DelayStream<S, IntervalDelay>;
}

impl<S: Stream> DelayedStreamExt<S> for S {
    fn sleep(self, dur: Duration) -> DelayStream<S, SleepDelay> {
        DelayStream::new(self, SleepDelay::new(dur))
    }

    fn interval(self, dur: Duration) -> DelayStream<S, IntervalDelay> {
        DelayStream::new(self, IntervalDelay::new(dur))
    }
}

pin_project! {
    /// `Stream` с задержкой получения элементов
    pub struct DelayStream<S: Stream, F: Future> {
        // Внутренний стрим
        #[pin]
        stream: S,
        #[pin]
        // Future задержки стрима
        delay_fut: F,
    }
}

impl<S: Stream, F: Future> DelayStream<S, F> {
    pub fn new(stream: S, delay_fut: F) -> Self {
        Self { stream, delay_fut }
    }
}

impl<S: Stream, F: Future> Stream for DelayStream<S, F> {
    type Item = S::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();
        // Проверяем, что ожидание завершено. Если нет - возвращаем `Poll::Pending`
        ready!(this.delay_fut.as_mut().poll(cx));
        this.stream.poll_next(cx)
    }
}
