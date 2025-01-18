//! A simple one-time channel that can `trigger` and `wait` on a single
//! `"untyped event"` between two tasks.
//!
//! Can be regarded as a thin wrapper layer over
//! [`tokio::sync::oneshot<()>`](https://docs.rs/tokio/latest/tokio/sync/oneshot/index.html)
//! channel.
//!
//! The [`once_event`] function is used to create a [`OnceTrigger`] and [`OnceWaiter`]
//! handle pair that form the channel.
//!
//! The [`OnceTrigger`] handle is used by the producer to trigger the event.
//! The [`OnceWaiter`] handle is used by the consumer to wait for the event.
//!
//! Each handle can be used on separate tasks.
//!
//! Since the [`OnceTrigger::trigger`] method is not async, it can be used anywhere.
//! This includes triggering between two runtimes, and using it from non-async code.
//!
//! # Examples
//!
//! ```
//! use est::sync::once::once_event;
//!
//! #[tokio::main]
//! async fn main() {
//!     let (trigger, waiter) = once_event();
//!
//!     tokio::spawn(async move {
//!         if trigger.trigger() {
//!             println!("event triggered");
//!         } else {
//!             println!("the waiter dropped");
//!         }
//!     });
//!
//!     if waiter.await {
//!         println!("event received");
//!     } else {
//!         println!("the trigger dropped");
//!     }
//! }
//! ```
//!
//! To use a [`OnceWaiter`] in a [`tokio::select!`](https://docs.rs/tokio/latest/tokio/macro.select.html)
//! loop, add `&mut` in front of the waiter.
//!
//! ```
//! use est::sync::once::once_event;
//! use tokio::time::{interval, sleep, Duration};
//!
//! #[tokio::main]
//! # async fn _doc() {}
//! # #[tokio::main(flavor = "current_thread", start_paused = true)]
//! async fn main() {
//!     let (shutdown_t, mut shutdown_w) = once_event();
//!     let mut interval = interval(Duration::from_millis(100));
//!
//!     # let handle =
//!     tokio::spawn(async move {
//!         sleep(Duration::from_secs(1)).await;
//!         shutdown_t.trigger();
//!     });
//!
//!     loop {
//!         tokio::select! {
//!             _ = interval.tick() => println!("Another 100ms"),
//!             _ = &mut shutdown_w => {
//!                 println!("Shutdown!");
//!                 break;
//!             }
//!         }
//!     }
//!     # handle.await.unwrap();
//! }
//! ```
//!
//! To use a [`OnceTrigger`] from a destructor, put it in an [`Option`] and call
//! [`Option::take`].
//!
//! ```
//! use est::sync::once::{once_event, OnceTrigger};
//!
//! struct TriggerOnDrop {
//!     trigger: Option<OnceTrigger>,
//! }
//! impl Drop for TriggerOnDrop {
//!     fn drop(&mut self) {
//!         if let Some(trigger) = self.trigger.take() {
//!             trigger.trigger();
//!         }
//!     }
//! }
//!
//! #[tokio::main]
//! # async fn _doc() {}
//! # #[tokio::main(flavor = "current_thread")]
//! async fn main() {
//!     let (trigger, waiter) = once_event();
//!
//!     let trigger_on_drop = TriggerOnDrop { trigger: Some(trigger) };
//!     drop(trigger_on_drop);
//!
//!     assert!(waiter.await);
//! }
//! ```

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::sync::oneshot::{channel, error::TryRecvError, Receiver, Sender};

#[derive(Debug)]
pub struct OnceTrigger(Sender<()>);

impl OnceTrigger {
    pub fn trigger(self) -> bool {
        self.0.send(()).is_ok()
    }
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Triggered {
    #[default]
    Pending,
    Triggered,
    Dropped,
}

#[derive(Debug)]
pub struct OnceWaiter {
    recv: Receiver<()>,
    triggered: Triggered,
}

impl OnceWaiter {
    pub fn triggered(&mut self) -> Triggered {
        match self.triggered {
            Triggered::Pending => {
                let triggered = match self.recv.try_recv() {
                    Ok(_) => Triggered::Triggered,
                    Err(TryRecvError::Closed) => Triggered::Dropped,
                    _ => Triggered::Pending,
                };
                self.triggered = triggered;
                triggered
            }
            triggered => triggered,
        }
    }

    pub fn has_been_triggered(mut self) -> Triggered {
        self.triggered()
    }

    pub fn blocking_wait(self) -> bool {
        self.recv.blocking_recv().is_ok()
    }
}

impl Future for OnceWaiter {
    type Output = bool;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.recv).poll(cx).map(|r| r.is_ok())
    }
}

pub fn once_event() -> (OnceTrigger, OnceWaiter) {
    let triggered = Default::default();
    let (send, recv) = channel();

    (OnceTrigger(send), OnceWaiter { recv, triggered })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn async_wait() {
        let (trigger, waiter) = once_event();
        tokio::spawn(async move {
            assert!(trigger.trigger());
        });
        assert!(waiter.await);

        let (trigger, waiter) = once_event();
        drop(waiter);
        assert!(!trigger.trigger());

        let (trigger, waiter) = once_event();
        drop(trigger);
        assert!(!waiter.await);
    }

    #[test]
    fn blocking_wait() {
        let (trigger, waiter) = once_event();
        std::thread::spawn(move || {
            assert!(trigger.trigger());
        });
        assert!(waiter.blocking_wait());

        let (trigger, waiter) = once_event();
        drop(waiter);
        assert!(!trigger.trigger());

        let (trigger, waiter) = once_event();
        drop(trigger);
        assert!(!waiter.blocking_wait());
    }

    #[test]
    fn triggered() {
        let (trigger, mut waiter) = once_event();
        assert_eq!(waiter.triggered(), Triggered::Pending);
        assert_eq!(waiter.triggered(), Triggered::Pending);
        assert!(trigger.trigger());
        assert_eq!(waiter.triggered(), Triggered::Triggered);
        assert_eq!(waiter.triggered(), Triggered::Triggered);

        let (trigger, mut waiter) = once_event();
        drop(trigger);
        assert_eq!(waiter.triggered(), Triggered::Dropped);
        assert_eq!(waiter.triggered(), Triggered::Dropped);
    }

    #[test]
    fn has_been_triggered() {
        let (trigger, waiter) = once_event();
        assert_eq!(waiter.has_been_triggered(), Triggered::Pending);
        assert!(!trigger.trigger());

        let (trigger, waiter) = once_event();
        assert!(trigger.trigger());
        assert_eq!(waiter.has_been_triggered(), Triggered::Triggered);

        let (trigger, waiter) = once_event();
        drop(trigger);
        assert_eq!(waiter.has_been_triggered(), Triggered::Dropped);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn tokio_select() {
        use std::time::Duration;
        use tokio::time::{interval, sleep};

        let mut ticks = 0;
        let mut interval = interval(Duration::from_millis(500));
        let (trigger, mut waiter) = once_event();

        tokio::spawn(async move {
            sleep(Duration::from_millis(1250)).await;
            trigger.trigger();
        });

        loop {
            tokio::select! {
                _ = interval.tick() => ticks += 1,
                _ = &mut waiter => break,
            }
        }

        assert_eq!(ticks, 3);
    }
}
