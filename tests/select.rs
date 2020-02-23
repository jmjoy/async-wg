use async_wg::WaitGroup;
use futures_timer::Delay;
use futures_util::future::{select, Either};
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::{sync::Arc, time::Duration};

#[tokio::test]
async fn test_select() {
    let count = Arc::new(AtomicUsize::new(0));
    let wg = WaitGroup::new();
    wg.add(1).await;

    for _ in 0..3 {
        let wg = wg.clone();
        wg.add(1).await;
        let count = count.clone();

        let fut = async move {
            Delay::new(Duration::from_secs(1)).await;
            count.fetch_add(1, Ordering::SeqCst);
            wg.done().await;
        };

        let fut2 = async move {};

        let fut = Box::pin(fut);
        let fut2 = Box::pin(fut2);

        tokio::spawn(select(fut, fut2));
    }

    match select(wg, Delay::new(Duration::from_secs(1))).await {
        Either::Left(_) => assert!(true),
        Either::Right(_) => assert!(false),
    }
}
