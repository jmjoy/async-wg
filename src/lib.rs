//! # async-wg
//!
//! Async version WaitGroup for RUST.
//!
//! ## Examples
//!
//! ```rust
//! #[tokio::main]
//! async fn main() {
//!     use async_wg::WaitGroup;
//!
//!     // Create a new wait group.
//!     let wg = WaitGroup::new();
//!
//!     for _ in 0..10 {
//!         let mut wg = wg.clone();
//!         // Add count n.
//!         wg.add(1).await;
//!
//!         tokio::spawn(async move {
//!             // Do some work.
//!
//!             // Done count 1.
//!             wg.done().await;
//!         });
//!     }
//!
//!     // Wait for done count is equal to add count.
//!     wg.await;
//! }
//! ```
//!
//! ## Benchmarks
//!
//! Simple benchmark comparison run on github actions.
//!
//! Code: [benchs/main.rs](https://github.com/jmjoy/async-wg/blob/master/benches/main.rs)
//!
//! ```text
//! test bench_join_handle ... bench:      34,485 ns/iter (+/- 18,969)
//! test bench_wait_group  ... bench:      36,916 ns/iter (+/- 7,555)
//! ```
//!
//! ## License
//!
//! The Unlicense.
//!

use futures_util::lock::Mutex;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Waker};

#[derive(Clone)]
/// Enables multiple tasks to synchronize the beginning or end of some computation.
pub struct WaitGroup {
    inner: Arc<Inner>,
}

struct Inner {
    count: Mutex<isize>,
    waker: Mutex<Option<Waker>>,
}

impl WaitGroup {
    /// Creates a new wait group and returns the single reference to it.
    ///
    /// # Examples
    /// ```rust
    /// use async_wg::WaitGroup;
    /// let wg = WaitGroup::new();
    /// ```
    pub fn new() -> WaitGroup {
        WaitGroup {
            inner: Arc::new(Inner {
                count: Mutex::new(0),
                waker: Mutex::new(None),
            }),
        }
    }

    /// Add count n.
    ///
    /// # Panic
    /// 1. The argument `delta` must be a positive number (> 0).
    /// 2. The max count must be less than `isize::max_value()`.
    pub async fn add(&mut self, delta: isize) {
        if delta <= 0 {
            panic!("The argument `delta` of wait group `add` must be a positive number");
        }

        let mut count = self.inner.count.lock().await;
        *count += delta;

        if *count >= isize::max_value() {
            panic!("wait group count is too large");
        }
    }

    /// Done count 1.
    pub async fn done(&mut self) {
        let mut count = self.inner.count.lock().await;
        *count -= 1;

        if *count <= 0 {
            if let Some(waker) = &*self.inner.waker.lock().await {
                waker.clone().wake();
            }
        }
    }

    /// Get the inner count of `WaitGroup`, the primary count is `0`.
    pub async fn count(&self) -> isize {
        *self.inner.count.lock().await
    }
}

impl Future for WaitGroup {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let mut count = self.inner.count.lock();
        let pin_count = Pin::new(&mut count);
        if let Poll::Ready(count) = pin_count.poll(cx) {
            if *count <= 0 {
                return Poll::Ready(());
            }
        }
        drop(count);

        let mut waker = self.inner.waker.lock();
        let pin_waker = Pin::new(&mut waker);
        if let Poll::Ready(mut waker) = pin_waker.poll(cx) {
            *waker = Some(cx.waker().clone());
        }

        Poll::Pending
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic]
    async fn add_zero() {
        let mut wg = WaitGroup::new();
        wg.add(0).await;
    }

    #[tokio::test]
    #[should_panic]
    async fn add_neg_one() {
        let mut wg = WaitGroup::new();
        wg.add(-1).await;
    }

    #[tokio::test]
    #[should_panic]
    async fn add_very_max() {
        let mut wg = WaitGroup::new();
        wg.add(isize::max_value()).await;
    }

    #[tokio::test]
    async fn add() {
        let mut wg = WaitGroup::new();
        wg.add(1).await;
        wg.add(10).await;
        assert_eq!(*wg.inner.count.lock().await, 11);
    }

    #[tokio::test]
    async fn done() {
        let mut wg = WaitGroup::new();
        wg.done().await;
        wg.done().await;
        assert_eq!(*wg.inner.count.lock().await, -2);
    }

    #[tokio::test]
    async fn count() {
        let mut wg = WaitGroup::new();
        assert_eq!(wg.count().await, 0);
        wg.add(10).await;
        assert_eq!(wg.count().await, 10);
        wg.done().await;
        assert_eq!(wg.count().await, 9);
    }
}
