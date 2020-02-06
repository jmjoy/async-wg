use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::sync::Mutex;
use std::sync::{Arc, RwLock};
use std::task::{Context, Poll, Waker};

pub struct WaitGroup {
    inner: Arc<Inner>,
}

struct Inner {
    count: Mutex<usize>,
    waker: RwLock<Option<Waker>>,
}

impl WaitGroup {
    pub fn new() -> WaitGroup {
        WaitGroup {
            inner: Arc::new(Inner {
                count: Mutex::new(1),
                waker: RwLock::new(None),
            }),
        }
    }

    fn set_waker(&mut self, waker: Waker) {
        *self.inner.waker.write().unwrap() = Some(waker);
    }
}

impl Drop for WaitGroup {
    fn drop(&mut self) {
        let mut count = self.inner.count.lock().unwrap();
        *count -= 1;

        if *count <= 1 {
            if let Some(waker) = &*self.inner.waker.read().unwrap() {
                waker.clone().wake();
            }
        }
    }
}

impl Clone for WaitGroup {
    fn clone(&self) -> WaitGroup {
        let mut count = self.inner.count.lock().unwrap();
        *count += 1;

        if *count >= usize::max_value() {
            panic!("count is too large");
        }

        WaitGroup {
            inner: self.inner.clone(),
        }
    }
}

impl fmt::Debug for WaitGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let count = self.inner.count.lock().unwrap();
        write!(f, "WaitGroup {{ count: {:?} }}", *count)
    }
}

impl Future for WaitGroup {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        if *self.inner.count.lock().unwrap() <= 1 {
            Poll::Ready(())
        } else {
            self.set_waker(cx.waker().clone());
            Poll::Pending
        }
    }
}
