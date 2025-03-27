#![cfg_attr(nightly, feature(doc_auto_cfg))]

//! **E**xtensions for the rust **S**tandard library and **T**okio.

pub use tokio;

/// Extensions to the [`std::collections`] module.
pub mod collections;
/// Extensions to the [`std::future`] module.
pub mod future;
/// Extensions to the [`std::process`] & [`tokio::process`] module.
pub mod process;
/// Extensions to the [`std::result`] module.
pub mod result;
/// Extensions to the [`std::sync`] & [`tokio::sync`] module.
pub mod sync;
/// Extensions to the [`std::task`] & [`tokio::task`] module.
pub mod task;
/// Extensions to the [`std::thread`] module.
pub mod thread;

pub use result::AnyRes;
