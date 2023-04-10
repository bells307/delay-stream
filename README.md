# delay-stream
Данная библиотека представляет собой реализацию `Stream` с задержкой возврата значения

## Example
```rust
use delay_stream::Delayed;
use futures::{stream, StreamExt};
use std::time::Duration;

#[tokio::test]
async fn delayed_stream() {
    let mut stream = stream::iter(vec![1, 3, 2, 4, 5]).delayed(Duration::from_secs(1));

    while let Some(val) = stream.next().await {
        // spent 1 sec ...
        println!("{}", val);
    }
}
```