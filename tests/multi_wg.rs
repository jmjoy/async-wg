use async_wg::WaitGroup;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::delay_for;

#[tokio::test]
async fn main() {
    let a_count = Arc::new(AtomicUsize::new(0));
    let b_count = Arc::new(AtomicUsize::new(0));

    let a_wg = WaitGroup::new();
    let b_wg = WaitGroup::new();

    a_wg.add(20).await;
    b_wg.add(20).await;

    for _ in 0..10 {
        let a_wg = a_wg.clone();
        let b_wg = b_wg.clone();
        let a_count = a_count.clone();
        tokio::spawn(async move {
            delay_for(Duration::from_millis(1)).await;
            a_count.fetch_add(1, Ordering::SeqCst);
            a_wg.done().await;
            b_wg.done().await;
        });
    }

    for _ in 0..10 {
        let a_wg = a_wg.clone();
        let b_wg = b_wg.clone();
        let b_count = b_count.clone();
        tokio::spawn(async move {
            delay_for(Duration::from_millis(1)).await;
            b_count.fetch_add(1, Ordering::SeqCst);
            a_wg.done().await;
            b_wg.done().await;
        });
    }

    a_wg.wait().await;
    b_wg.wait().await;

    assert_eq!(a_count.load(Ordering::SeqCst), 10);
    assert_eq!(b_count.load(Ordering::SeqCst), 10);
}
