/// Extensions to [`tokio-util::task::TaskTracker`](https://docs.rs/tokio-util/latest/tokio_util/task/task_tracker/struct.TaskTracker.html)
pub mod task_tracker;

use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU64;

/// A [`TaskId`](https://docs.rs/tokio/latest/tokio/task/struct.Id.html) that can be `serde`.
#[derive(Debug, Display, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash)]
#[serde(transparent)]
pub struct TaskId(pub NonZeroU64);

impl From<tokio::task::Id> for TaskId {
    fn from(value: tokio::task::Id) -> Self {
        Self(value.to_string().parse().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn from_tokio_task_id() {
        let id = tokio::spawn(async { tokio::task::id() }).await.unwrap();
        assert_eq!(id.to_string(), TaskId::from(id).to_string());
    }
}
