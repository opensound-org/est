/// Extensions to [`tokio_util::task::TaskTracker`]
pub mod task_tracker;

use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU64;

/// A [`TaskId`] that can be `serde`.
///
/// [`TaskId`]: tokio::task::Id
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
