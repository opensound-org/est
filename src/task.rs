use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU64;
use tokio_util::task::{TaskTracker, task_tracker::TaskTrackerWaitFuture};

/// A [`TaskId`](https://docs.rs/tokio/latest/tokio/task/struct.Id.html) that can be `serde`.
#[derive(Debug, Display, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash)]
#[serde(transparent)]
pub struct TaskId(pub NonZeroU64);

impl From<tokio::task::Id> for TaskId {
    fn from(value: tokio::task::Id) -> Self {
        Self(value.to_string().parse().unwrap())
    }
}

/// Execute [`close`](https://docs.rs/tokio-util/latest/tokio_util/task/task_tracker/struct.TaskTracker.html#method.close)
/// and [`wait`](https://docs.rs/tokio-util/latest/tokio_util/task/task_tracker/struct.TaskTracker.html#method.wait)
/// for [`TaskTracker`](https://docs.rs/tokio-util/latest/tokio_util/task/task_tracker/struct.TaskTracker.html) at once.
pub trait CloseAndWait {
    fn close_and_wait(&self) -> TaskTrackerWaitFuture;
}

impl CloseAndWait for TaskTracker {
    fn close_and_wait(&self) -> TaskTrackerWaitFuture {
        self.close();
        self.wait()
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

    fn tracker_spawn() -> TaskTracker {
        let tracker = TaskTracker::new();

        for i in 0..3 {
            tracker.spawn(async move { i });
        }

        tracker
    }

    #[tokio::test]
    async fn close_and_wait() {
        use std::time::Duration;
        use tokio_util::time::FutureExt;

        let tracker = tracker_spawn();
        assert!(
            tracker
                .wait()
                .timeout(Duration::from_secs_f64(1.5))
                .await
                .is_err()
        );

        let tracker = tracker_spawn();
        tracker.close();
        assert!(
            tracker
                .wait()
                .timeout(Duration::from_secs_f64(1.5))
                .await
                .is_ok()
        );

        let tracker = tracker_spawn();
        assert!(
            tracker
                .close_and_wait()
                .timeout(Duration::from_secs_f64(1.5))
                .await
                .is_ok()
        );
    }
}
