use async_wg::WaitGroup;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;

#[tokio::test]
async fn test_await() {
    let count = Arc::new(AtomicUsize::new(0));
    let wg = WaitGroup::new();

    for _ in 0..10 {
        let mut wg = wg.clone();
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
