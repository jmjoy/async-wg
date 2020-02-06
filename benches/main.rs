#![feature(test)]

extern crate test;

use async_wg::WaitGroup;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use test::Bencher;

#[tokio::test]
async fn wait_group_version() {
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

#[tokio::test]
async fn join_handle_version() {
    let count = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::with_capacity(10);

    for _ in 0..10 {
        let count = count.clone();

        let handle = tokio::spawn(async move {
            count.fetch_add(1, Ordering::SeqCst);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    assert_eq!(count.load(Ordering::SeqCst), 10);
}

#[bench]
fn bench_join_handle(b: &mut Bencher) {
    b.iter(|| join_handle_version());
}

#[bench]
fn bench_wait_group(b: &mut Bencher) {
    b.iter(|| wait_group_version());
}
