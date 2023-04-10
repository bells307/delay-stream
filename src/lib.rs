use futures::Future;
use futures::{ready, Stream};
use pin_project_lite::pin_project;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::time;
use tokio::time::{Instant, Sleep};

pin_project! {
    /// `Stream` с задержкой получения элементов
    /// ```
    /// use delay_stream::Delayed;
    /// use futures::{stream, StreamExt};
    /// use std::time::Duration;
    ///
    /// #[tokio::test]
    /// async fn delayed_stream() {
    ///     let mut stream = stream::iter(vec![1, 3, 2, 4, 5]).delayed(Duration::from_secs(1));
    ///
    ///     while let Some(val) = stream.next().await {
    ///         // spent 1 sec ...
    ///         println!("{}", val);
    ///     }
    /// }
    /// ```
    pub struct DelayStream<S: Stream> {
        // Внутренний стрим
        #[pin]
        stream: S,
        #[pin]
        // Future ожидания
        sleep: Sleep,
        // Период ожидания
        dur: Duration,
    }
}

/// Расширение для `Stream`, позволяющее добавить ожидание перед выдачей элемента
pub trait Delayed<S: Stream> {
    fn delayed(self, dur: Duration) -> DelayStream<S>;
}

impl<S: Stream> Delayed<S> for S {
    fn delayed(self, dur: Duration) -> DelayStream<S> {
        DelayStream::new(self, dur)
    }
}

impl<S: Stream> DelayStream<S> {
    pub fn new(stream: S, dur: Duration) -> Self {
        Self {
            stream,
            dur,
            sleep: time::sleep(dur),
        }
    }
}

impl<S: Stream> Stream for DelayStream<S> {
    type Item = S::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();
        // Проверяем, что ожидание завершено. Если нет - возвращаем `Poll::Pending`
        ready!(this.sleep.as_mut().poll(cx));
        // Сбрасываем таймаут ожидания
        this.sleep.reset(Instant::now() + *this.dur);
        this.stream.poll_next(cx)
    }
}
