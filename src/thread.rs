use derive_more::Display;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::num::NonZeroU64;

/// A [`ThreadId`] that can be `serde` and `Display`ed.
///
/// [`ThreadId`]: std::thread::ThreadId
#[derive(Debug, Display, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
pub struct ThreadId(pub NonZeroU64);

#[cfg(feature = "serde")]
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
    #[cfg(feature = "serde")]
    use super::*;

    #[test]
    #[cfg(feature = "serde")]
    fn from_std_thread_id() {
        let id = std::thread::current().id();
        let thread_id = ThreadId::from(id);
        let debug = format!("{:?}", id);

        assert_eq!(debug, format!("{:?}", thread_id));
        assert_eq!(debug, format!("ThreadId({})", thread_id));
    }
}
