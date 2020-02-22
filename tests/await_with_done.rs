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

    wg.wait().await;

    assert_eq!(count.load(Ordering::SeqCst), 10);
}

#[tokio::test]
async fn test_await_empty() {
    let wg = WaitGroup::new();
    wg.wait().await;
}

#[tokio::test]
async fn test_await_add() {
    let count = Arc::new(AtomicUsize::new(0));
    let wg = WaitGroup::new();
    wg.add(10).await;

    for _ in 0..10 {
        let wg = wg.clone();
        let count = count.clone();

        tokio::spawn(async move {
            count.fetch_add(1, Ordering::SeqCst);
            wg.done().await;
        });
    }

    wg.wait().await;

    assert_eq!(count.load(Ordering::SeqCst), 10);
}

#[tokio::test]
async fn test_await_complex() {
    let count = Arc::new(AtomicUsize::new(0));
    let wg = WaitGroup::new();

    for _ in 0..10 {
        let wg = wg.clone();
        wg.add(2).await;
        let count = count.clone();

        tokio::spawn(async move {
            count.fetch_add(1, Ordering::SeqCst);
            Delay::new(Duration::from_millis(1)).await;
            delay_for(Duration::from_millis(1)).await;

            let wg0 = wg.clone();

            tokio::spawn(async move {
                count.fetch_add(1, Ordering::SeqCst);
                Delay::new(Duration::from_millis(1)).await;
                delay_for(Duration::from_millis(1)).await;

                wg0.done().await;
            });

            wg.done().await;
        });
    }

    wg.wait().await;

    assert_eq!(count.load(Ordering::SeqCst), 20);
}
