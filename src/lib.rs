#![cfg_attr(nightly, feature(doc_auto_cfg))]
#![allow(rustdoc::broken_intra_doc_links)]

//! **E**xtensions for the rust **S**tandard library and **T**okio.
//!
//! # Feature flags
//!
//! **The default feature will not enable anything** (based on the principle of minimum
//! dependency). At the same time, each top-level module has a feature flag with the same name
//! (currently including: `collections`, `future`, `process`, `result`, `slice`, `sync`, `task`, `thread`).
//!
//! There is also a feature flag called `full` that enables all features and introduces all
//! optional dependencies.
//!
//! In addition, there are some optional feature flags as follows:
//!
//! - `signal`: Enables `ctrl-c` signal processing in the [`task::graceful`] module.
//! - `task_tracker`: Enables the [`task::task_tracker`] module.
//! - `indexmap`: Implement [`collections::MapExt`] for [`indexmap::IndexMap`].
//! - `serde`: Enables [`serde`] support for the entire crate.

#[cfg(feature = "tokio")]
pub use tokio;

/// Extensions to the [`std::collections`] module.
#[cfg(feature = "collections")]
pub mod collections;
/// Extensions to the [`std::future`] module.
#[cfg(feature = "future")]
pub mod future;
/// Extensions to the [`std::process`] & [`tokio::process`] module.
#[cfg(feature = "process")]
pub mod process;
/// Extensions to the [`std::result`] module.
#[cfg(feature = "result")]
pub mod result;
/// Extensions to the [`slice`] type.
#[cfg(feature = "slice")]
pub mod slice;
/// Extensions to the [`std::sync`] & [`tokio::sync`] module.
#[cfg(feature = "sync")]
pub mod sync;
/// Extensions to the [`std::task`] & [`tokio::task`] module.
#[cfg(feature = "task")]
pub mod task;
/// Extensions to the [`std::thread`] module.
#[cfg(feature = "thread")]
pub mod thread;

#[cfg(feature = "result")]
pub use result::AnyRes;
