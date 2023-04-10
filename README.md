# delay-stream
Данная библиотека представляет расширения для `Stream` с задержкой возврата значения
```rust
/// Расширения для `Stream`, которые позволяют добавить ожидание по времени между отдачами элементов
pub trait DelayedStreamExt<S: Stream> {
    /// Ожидание полного промежутка времени `Duration` между выдачей элементов `Stream`'а
    fn sleep(self, dur: Duration) -> DelayStream<S, SleepDelay>;

    /// Интервальное ожидание между отдачами элемента. Это значит, что элементы будут выдаваться
    /// **не чаще**, чем указанный `Duration`.
    fn interval(self, dur: Duration) -> DelayStream<S, IntervalDelay>;
}
```

## sleep-delayed
Расширение для `Stream`, позволяющее добавить ожидание полного промежутка времени `Duration`
между выдачей элементов `Stream`'а

### Example
```rust
use delay_stream::sleep::SleepDelayed;
use futures::{stream, StreamExt};
use std::pin::pin;
use std::time::Duration;

#[tokio::test]
async fn sleep_delayed() {
    let stream = stream::iter(vec![1, 3, 2, 4, 5]).sleep_delayed(Duration::from_secs(1));
    let mut stream = pin!(stream);

    while let Some(val) = stream.next().await {
        // spent 1 sec ...
        println!("{}", val);
    }
}
```

## interval-delayed
Расширение для `Stream`, позволяющее добавить интервальное ожидание между отдачами элемента.
Это значит, что элементы будут выдаваться **не чаще**, чем указанный `Duration`.

### Example
```rust
use delay_stream::interval::IntervalDelayed;
use futures::{stream, StreamExt};
use std::pin::pin;
use std::time::Duration;

#[tokio::test]
async fn interval_delayed() {
    let stream = stream::iter(vec![1, 3, 2, 4, 5]).interval_delayed(Duration::from_secs(1));
    let mut stream = pin!(stream);

    while let Some(val) = stream.next().await {
        // spent 1 sec ...
        println!("{}", val);
    }
}
```