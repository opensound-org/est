#![cfg_attr(nightly, feature(doc_auto_cfg))]

//! **E**xtensions for the rust **S**tandard library and **T**okio.

pub use tokio;

/// Extensions to the [`std::collections`](https://doc.rust-lang.org/stable/std/collections/index.html) module.
pub mod collections;
/// Extensions to the [`std::future`](https://doc.rust-lang.org/stable/std/future/index.html) module.
pub mod future;
/// Extensions to the [`std::process`](https://doc.rust-lang.org/stable/std/process/index.html) &
/// [`tokio::process`](https://docs.rs/tokio/latest/tokio/process/index.html) module.
pub mod process;
/// Extensions to the [`std::result`](https://doc.rust-lang.org/stable/std/result/index.html) module.
pub mod result;
/// Extensions to the [`std::sync`](https://doc.rust-lang.org/stable/std/sync/index.html) &
/// [`tokio::sync`](https://docs.rs/tokio/latest/tokio/sync/index.html) module.
pub mod sync;
/// Extensions to the [`std::task`](https://doc.rust-lang.org/stable/std/task/index.html) &
/// [`tokio::task`](https://docs.rs/tokio/latest/tokio/task/index.html) module.
pub mod task;
/// Extensions to the [`std::thread`](https://doc.rust-lang.org/stable/std/thread/index.html) module.
pub mod thread;

pub use result::AnyRes;
