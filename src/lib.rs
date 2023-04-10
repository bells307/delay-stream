pub mod interval;
pub mod sleep;

use futures::Future;
use futures::{ready, Stream};
use pin_project_lite::pin_project;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::time::Sleep;

pin_project! {
    /// `Stream` с задержкой получения элементов
    /// ```
    /// use delay_stream::interval::IntervalDelayed;
    /// use delay_stream::sleep::SleepDelayed;
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
    ///     let stream = stream::iter(vec![1, 3, 2, 4, 5]).interval_delayed(Duration::from_secs(1));
    ///     let mut stream = pin!(stream);
    ///
    ///     while let Some(val) = stream.next().await {
    ///         // spent 1 sec ...
    ///         println!("{}", val);
    ///     }
    /// }
    /// ```
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
