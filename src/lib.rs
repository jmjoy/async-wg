//! # async-wg
//!
//! Async version WaitGroup for RUST.
//!
//! ## Examples
//!
//! ```rust
//! #[tokio::main]
//! async fn test_await() {
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
//!     // Wait for done count is equal to done count.
//!     wg.await;
//! }
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
pub struct WaitGroup {
    inner: Arc<Inner>,
}

struct Inner {
    count: Mutex<isize>,
    waker: Mutex<Option<Waker>>,
}

impl WaitGroup {
    pub fn new() -> WaitGroup {
        WaitGroup {
            inner: Arc::new(Inner {
                count: Mutex::new(1),
                waker: Mutex::new(None),
            }),
        }
    }

    pub async fn add(&mut self, delta: isize) {
        let mut count = self.inner.count.lock().await;
        *count += delta;

        if *count >= isize::max_value() {
            panic!("count is too large");
        }
    }

    pub async fn done(&mut self) {
        let mut count = self.inner.count.lock().await;
        *count -= 1;

        if *count <= 1 {
            if let Some(waker) = &*self.inner.waker.lock().await {
                waker.clone().wake();
            }
        }
    }
}

impl Future for WaitGroup {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let mut count = self.inner.count.lock();
        let pin_count = Pin::new(&mut count);

        if let Poll::Ready(count) = pin_count.poll(cx) {
            if *count <= 1 {
                return Poll::Ready(());
            }
        }

        let mut waker = self.inner.waker.lock();
        let pin_waker = Pin::new(&mut waker);
        if let Poll::Ready(mut waker) = pin_waker.poll(cx) {
            *waker = Some(cx.waker().clone());
        }

        Poll::Pending
    }
}
