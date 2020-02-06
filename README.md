# async-wg

[![Rustc Version](https://img.shields.io/badge/rustc-1.39+-lightgray.svg)](https://blog.rust-lang.org/2019/11/07/Rust-1.39.0.html)
[![Actions](https://github.com/jmjoy/async-wg/workflows/CI/badge.svg)](https://github.com/jmjoy/async-wg/actions?query=workflow%3ACI)
[![Crate](https://img.shields.io/crates/v/async-wg.svg)](https://crates.io/crates/async-wg)
[![API](https://docs.rs/async-wg/badge.svg)](https://docs.rs/async-wg)

Async version WaitGroup for RUST.

## Installation

With [cargo add](https://github.com/killercup/cargo-edit) installed run:

```sh
$ cargo add -s async-wg
```

## Examples

 ```rust
 #[tokio::main]
 async fn test_await() {
     use async_wg::WaitGroup;

     // Create a new wait group.
     let wg = WaitGroup::new();

     for _ in 0..10 {
         let mut wg = wg.clone();
         // Add count n.
         wg.add(1).await;

         tokio::spawn(async move {
             // Do some work.

             // Done count 1.
             wg.done().await;
         });
     }

     // Wait for done count is equal to done count.
     wg.await;
 }
 ```

 ## License

 The Unlicense.
