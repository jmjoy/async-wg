use async_wg::WaitGroup;
use futures_timer::Delay;
use futures_util::future::{select, Either};
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::{sync::Arc, time::Duration};

#[tokio::test]
async fn test_await() {
    let count = Arc::new(AtomicUsize::new(0));
    let wg = WaitGroup::new();
    wg.add(1).await;

    for _ in 0..3 {
        let wg = wg.clone();
        wg.add(1).await;
        let count = count.clone();

        tokio::spawn(async move {
            count.fetch_add(1, Ordering::SeqCst);
            wg.done().await;
        });
    }

    match select(wg, Delay::new(Duration::from_secs(1))).await {
        Either::Left(_) => assert!(false),
        Either::Right(_) => assert!(true),
    }
}
