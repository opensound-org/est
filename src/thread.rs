use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU64;

/// A [`ThreadId`] that can be `serde` and `Display`ed.
///
/// [`ThreadId`]: std::thread::ThreadId
#[derive(Debug, Display, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash)]
#[serde(transparent)]
pub struct ThreadId(pub NonZeroU64);

impl From<std::thread::ThreadId> for ThreadId {
    fn from(value: std::thread::ThreadId) -> Self {
        #[derive(Deserialize)]
        #[serde(rename = "ThreadId")]
        struct Inner(NonZeroU64);

        Self(ron::from_str::<Inner>(&format!("{:?}", value)).unwrap().0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_std_thread_id() {
        let id = std::thread::current().id();
        let thread_id = ThreadId::from(id);
        let debug = format!("{:?}", id);

        assert_eq!(debug, format!("{:?}", thread_id));
        assert_eq!(debug, format!("ThreadId({})", thread_id));
    }
}
