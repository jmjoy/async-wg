use async_wg::WaitGroup;
use futures_timer::Delay;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::delay_for;

#[tokio::test]
async fn test_await() {
    let count = Arc::new(AtomicUsize::new(0));
    let wg = WaitGroup::new();

    for _ in 0..10 {
        let wg = wg.clone();
        wg.add(1).await;
        let count = count.clone();

        tokio::spawn(async move {
            count.fetch_add(1, Ordering::SeqCst);
            wg.done().await;
        });
    }

    wg.await;

    assert_eq!(count.load(Ordering::SeqCst), 10);
}

#[tokio::test]
async fn test_await_complex_1() {
    let count = Arc::new(AtomicUsize::new(0));
    let wg = WaitGroup::new();

    for _ in 0..10 {
        let wg = wg.clone();
        wg.add(1).await;
        let count = count.clone();

        tokio::spawn(async move {
            count.fetch_add(1, Ordering::SeqCst);
            Delay::new(Duration::from_secs(1)).await;
            wg.done().await;
        });
    }

    wg.await;

    assert_eq!(count.load(Ordering::SeqCst), 10);
}

#[tokio::test]
async fn test_await_complex_2() {
    let count = Arc::new(AtomicUsize::new(0));
    let wg = WaitGroup::new();

    for _ in 0..10 {
        let wg = wg.clone();
        wg.add(1).await;
        let count = count.clone();

        tokio::spawn(async move {
            count.fetch_add(1, Ordering::SeqCst);
            delay_for(Duration::from_secs(1)).await;
            wg.done().await;
        });
    }

    wg.await;

    assert_eq!(count.load(Ordering::SeqCst), 10);
}
