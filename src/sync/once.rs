//! A simple one-time channel that can `trigger` and `wait` on a single
//! `"untyped event"` between two tasks.
//!
//! Can be regarded as a thin wrapper layer over
//! [`tokio::sync::oneshot<()>`](https://docs.rs/tokio/latest/tokio/sync/oneshot/index.html)
//! channel.
//!
//! This is a simple one-time single event synchronization primitive, which consists of an
//! exclusive pair of a single `trigger` and a single `waiter`.
//!
//! If you need a more advanced event triggering/waiting primitive, such as triggering events
//! with types, or triggering events multiple times, or triggering events in multiple places,
//! or waiting for the same event in multiple places, this primitive is not suitable for you,
//! and you may need to use a more advanced `channel` type.
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

/// Triggers the event to the associated [`OnceWaiter`].
///
/// A pair of both a [`OnceTrigger`] and a [`OnceWaiter`]  are created by the
/// [`once_event`] function.
///
/// # Examples
///
/// ```
/// use est::sync::once::once_event;
///
/// #[tokio::main]
/// async fn main() {
///     let (trigger, waiter) = once_event();
///
///     tokio::spawn(async move {
///         if trigger.trigger() {
///             println!("event triggered");
///         } else {
///             println!("the waiter dropped");
///         }
///     });
///
///     if waiter.await {
///         println!("event received");
///     } else {
///         println!("the trigger dropped");
///     }
/// }
/// ```
///
/// To use a [`OnceTrigger`] from a destructor, put it in an [`Option`] and call
/// [`Option::take`].
///
/// ```
/// use est::sync::once::{once_event, OnceTrigger};
///
/// struct TriggerOnDrop {
///     trigger: Option<OnceTrigger>,
/// }
/// impl Drop for TriggerOnDrop {
///     fn drop(&mut self) {
///         if let Some(trigger) = self.trigger.take() {
///             trigger.trigger();
///         }
///     }
/// }
///
/// #[tokio::main]
/// # async fn _doc() {}
/// # #[tokio::main(flavor = "current_thread")]
/// async fn main() {
///     let (trigger, waiter) = once_event();
///
///     let trigger_on_drop = TriggerOnDrop { trigger: Some(trigger) };
///     drop(trigger_on_drop);
///
///     assert!(waiter.await);
/// }
/// ```
#[derive(Debug)]
pub struct OnceTrigger(Sender<()>);

impl OnceTrigger {
    /// Attempts to trigger the event on this one-time channel, returns whether
    /// triggering succeeded.
    ///
    /// This method consumes `self` as only one event may ever be triggered to
    /// the waiter. It is not marked async because triggering a event to a waiter
    /// never requires any form of waiting. Because of this, the `trigger`
    /// method can be used in both synchronous and asynchronous code without
    /// problems.
    ///
    /// A successful trigger occurs when it is determined that the other end of the
    /// pair has not hung up already. An unsuccessful trigger would be one where
    /// the corresponding [`OnceWaiter`] has already been deallocated. Note that a
    /// return value of `false` means that the event will never be received, but
    /// a return value of `true` does *not* mean that the event will be received.
    /// It is possible for the corresponding [`OnceWaiter`] to hang up immediately
    /// after this function returns `true`.
    ///
    /// # Examples
    ///
    /// Trigger the event to another task
    ///
    /// ```
    /// use est::sync::once::once_event;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let (trigger, waiter) = once_event();
    ///
    ///     tokio::spawn(async move {
    ///         if trigger.trigger() {
    ///             println!("event triggered");
    ///         } else {
    ///             println!("the waiter dropped");
    ///         }
    ///     });
    ///
    ///     if waiter.await {
    ///         println!("event received");
    ///     } else {
    ///         println!("the trigger dropped");
    ///     }
    /// }
    /// ```
    pub fn trigger(self) -> bool {
        self.0.send(()).is_ok()
    }

    /// Waits for the associated [`OnceWaiter`] handle to drop.
    ///
    /// This function is useful when paired with `select!` to abort a
    /// computation when the [`OnceWaiter`] is no longer waiting for
    /// the event.
    ///
    /// # Return
    ///
    /// Returns a `Future` which must be awaited on.
    ///
    /// # Examples
    ///
    /// Basic usage
    ///
    /// ```
    /// use est::sync::once::once_event;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let (mut trigger, waiter) = once_event();
    ///
    ///     tokio::spawn(async move {
    ///         drop(waiter);
    ///     });
    ///
    ///     trigger.dropped().await;
    ///     println!("the waiter dropped");
    /// }
    /// ```
    ///
    /// Paired with select
    ///
    /// ```
    /// use est::sync::once::once_event;
    /// use tokio::time::{self, Duration};
    ///
    /// async fn compute() -> String {
    ///     // Complex computation returning a `String`.
    /// # "hello".to_string()
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let (mut trigger, waiter) = once_event();
    ///
    ///     tokio::spawn(async move {
    ///         tokio::select! {
    ///             _ = trigger.dropped() => {
    ///                 // The waiter dropped, no need to do any further work.
    ///             }
    ///             value = compute() => {
    ///                 // The trigger can fail if the waiter was dropped at the
    ///                 // exact same time as when compute() finished, so just ignore
    ///                 // the return value.
    ///                 trigger.trigger();
    ///             }
    ///         }
    ///     });
    ///
    ///     // Wait for up to 10 seconds.
    ///     let _ = time::timeout(Duration::from_secs(10), waiter).await;
    /// }
    /// ```
    pub async fn dropped(&mut self) {
        self.0.closed().await
    }

    /// Returns `true` if the associated [`OnceWaiter`] handle has been dropped.
    ///
    /// If `true` is returned, a call to [`trigger`] will always result in `false`.
    ///
    /// [`trigger`]: OnceTrigger::trigger
    ///
    /// # Examples
    ///
    /// ```
    /// use est::sync::once::once_event;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let (trigger, waiter) = once_event();
    ///
    ///     assert!(!trigger.is_dropped());
    ///
    ///     drop(waiter);
    ///
    ///     assert!(trigger.is_dropped());
    ///     assert!(!trigger.trigger());
    /// }
    /// ```
    pub fn is_dropped(&self) -> bool {
        self.0.is_closed()
    }

    /// Checks whether the [`OnceWaiter`] has been dropped, and if not, schedules the
    /// `Waker` in the provided `Context` to receive a notification when the [`OnceWaiter`]
    /// is dropped.
    ///
    /// Note that on multiple calls to poll, only the `Waker` from the `Context` passed
    /// to the most recent call will be scheduled to receive a wakeup.
    ///
    /// # Return value
    ///
    /// This function returns:
    ///
    ///  * `Poll::Pending` if the [`OnceWaiter`] is still alive.
    ///  * `Poll::Ready(())` if the [`OnceWaiter`] is dropped.
    ///
    /// # Examples
    ///
    /// ```
    /// use est::sync::once::once_event;
    ///
    /// use std::future::poll_fn;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let (mut trigger, mut waiter) = once_event();
    ///
    ///     tokio::spawn(async move {
    ///         drop(waiter);
    ///     });
    ///
    ///     poll_fn(|cx| trigger.poll_dropped(cx)).await;
    ///
    ///     println!("the waiter dropped");
    /// }
    /// ```
    pub fn poll_dropped(&mut self, cx: &mut Context<'_>) -> Poll<()> {
        self.0.poll_closed(cx)
    }
}

/// The triggered state type returned by [`OnceWaiter::triggered`]
/// and [`OnceWaiter::has_been_triggered`].
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Triggered {
    /// The [`OnceTrigger`] half of the exclusive-pair has not been dropped,
    /// and also has not yet triggered the event.
    #[default]
    Pending,
    /// The [`OnceTrigger`] half of the exclusive-pair has triggered the event.
    Triggered,
    /// The [`OnceTrigger`] half of the exclusive-pair was dropped without
    /// triggering the event.
    Dropped,
}

/// Wait on the event triggered from the associated [`OnceTrigger`].
///
/// A pair of both a [`OnceTrigger`] and a [`OnceWaiter`] are created by the
/// [`once_event`] function.
///
/// This waiter has no `wait` method because the waiter itself implements the
/// [`Future`] trait. To wait for the event, `.await` the [`OnceWaiter`] object
/// directly.
///
/// The `poll` method on the `Future` trait is allowed to spuriously return
/// `Poll::Pending` even if the event has been triggered. If such a spurious
/// failure happens, then the caller will be woken when the spurious failure has
/// been resolved so that the caller can attempt to wait on the event again.
/// Note that receiving such a wakeup does not guarantee that the next call will
/// succeed — it could fail with another spurious failure. (A spurious failure
/// does not mean that the event is lost. It is just delayed.)
///
/// [`Future`]: trait@std::future::Future
///
/// # Examples
///
/// ```
/// use est::sync::once::once_event;
///
/// #[tokio::main]
/// async fn main() {
///     let (trigger, waiter) = once_event();
///
///     tokio::spawn(async move {
///         if trigger.trigger() {
///             println!("event triggered");
///         } else {
///             println!("the waiter dropped");
///         }
///     });
///
///     if waiter.await {
///         println!("event received");
///     } else {
///         println!("the trigger dropped");
///     }
/// }
/// ```
///
/// To use a [`OnceWaiter`] in a [`tokio::select!`](https://docs.rs/tokio/latest/tokio/macro.select.html)
/// loop, add `&mut` in front of the waiter.
///
/// ```
/// use est::sync::once::once_event;
/// use tokio::time::{interval, sleep, Duration};
///
/// #[tokio::main]
/// # async fn _doc() {}
/// # #[tokio::main(flavor = "current_thread", start_paused = true)]
/// async fn main() {
///     let (shutdown_t, mut shutdown_w) = once_event();
///     let mut interval = interval(Duration::from_millis(100));
///
///     # let handle =
///     tokio::spawn(async move {
///         sleep(Duration::from_secs(1)).await;
///         shutdown_t.trigger();
///     });
///
///     loop {
///         tokio::select! {
///             _ = interval.tick() => println!("Another 100ms"),
///             _ = &mut shutdown_w => {
///                 println!("Shutdown!");
///                 break;
///             }
///         }
///     }
///     # handle.await.unwrap();
/// }
/// ```
#[derive(Debug)]
pub struct OnceWaiter {
    recv: Receiver<()>,
    triggered: Triggered,
}

impl OnceWaiter {
    /// Obtain whether [`OnceTrigger`] has triggered the event.
    ///
    /// This function is useful to call from outside the context of an
    /// asynchronous task.
    ///
    /// Note that unlike the `poll` method, the `triggered` method cannot fail
    /// spuriously. Any `trigger` or `drop` event that happens before this call
    /// to `triggered` will be correctly returned to the caller.
    ///
    /// If the method does not return [`Triggered::Pending`], if you call `poll`
    /// or `bloking_wait` next, they will return immediately with the corresponding
    /// boolean value. At the same time, if you call `(&mut waiter).await`, the
    /// next call to `triggered` will also correctly return the corresponding
    /// [`Triggered::Triggered`] or [`Triggered::Dropped`] value.
    ///
    /// # Return
    ///
    /// - [`Triggered::Pending`] if the `trigger` has not been dropped, and has
    ///   not yet triggered the event.
    /// - [`Triggered::Triggered`] if the event has been triggered.
    /// - [`Triggered::Dropped`] if the `trigger` has dropped without triggering
    ///   the event.
    ///
    /// # Examples
    ///
    /// `triggered` before triggering the event, then after.
    ///
    /// ```
    /// use est::sync::once::{once_event, Triggered};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///    let (trigger, mut waiter) = once_event();
    ///
    ///     match waiter.triggered() {
    ///         // The event is currently pending
    ///         Triggered::Pending => {}
    ///         _ => unreachable!(),
    ///     }
    ///
    ///     // Trigger the event
    ///     trigger.trigger();
    ///
    ///     match waiter.triggered() {
    ///         // The event has been triggered
    ///         Triggered::Triggered => {}
    ///         _ => unreachable!(),
    ///     }
    /// }
    /// ```
    ///
    /// `triggered` when the `trigger` dropped before triggering the event.
    ///
    /// ```
    /// use est::sync::once::{once_event, Triggered};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let (trigger, mut waiter) = once_event();
    ///
    ///     drop(trigger);
    ///
    ///     match waiter.triggered() {
    ///         // The event will never be triggered.
    ///         Triggered::Dropped => {}
    ///         _ => unreachable!(),
    ///     }
    /// }
    /// ```
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

    /// Similar to [`OnceWaiter::triggered`], but will consume `self`.
    ///
    /// This method is very similar to calling `triggered` first and then
    /// dropping [`OnceWaiter`] immediately.
    ///
    /// Any `trigger` operation which happens after calling `has_been_triggered`
    /// is guaranteed to fail.
    ///
    /// This function is useful to perform a graceful shutdown and obtain whether
    /// the event has been triggered or not, then ensure that the event will
    /// never be triggered afterwards.
    ///
    /// # Examples
    ///
    /// Prevent the event from being triggered afterwards.
    ///
    /// ```
    /// use est::sync::once::{once_event, Triggered};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let (trigger, waiter) = once_event();
    ///
    ///     assert!(!trigger.is_dropped());
    ///     assert_eq!(waiter.has_been_triggered(), Triggered::Pending);
    ///     assert!(trigger.is_dropped());
    ///     assert!(!trigger.trigger());
    /// }
    /// ```
    ///
    /// Obtain whether the event has been triggered **before** calling `has_been_triggered`.
    ///
    /// ```
    /// use est::sync::once::{once_event, Triggered};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let (trigger, waiter) = once_event();
    ///
    ///     assert!(trigger.trigger());
    ///     assert_eq!(waiter.has_been_triggered(), Triggered::Triggered);
    /// }
    /// ```
    pub fn has_been_triggered(mut self) -> Triggered {
        self.triggered()
    }

    /// Blocking wait to call outside of asynchronous contexts.
    ///
    /// # Panics
    ///
    /// This function panics if called within an asynchronous execution
    /// context.
    ///
    /// # Examples
    ///
    /// ```
    /// use est::sync::once::once_event;
    /// use std::thread;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let (trigger, waiter) = once_event();
    ///
    ///     let sync_code = thread::spawn(move || {
    ///         assert!(waiter.blocking_wait());
    ///     });
    ///
    ///     assert!(trigger.trigger());
    ///     sync_code.join().unwrap();
    /// }
    /// ```
    pub fn blocking_wait(self) -> bool {
        if self.triggered != Triggered::Pending {
            return self.triggered == Triggered::Triggered;
        }

        self.recv.blocking_recv().is_ok()
    }
}

impl Future for OnceWaiter {
    type Output = bool;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.triggered != Triggered::Pending {
            return Poll::Ready(self.triggered == Triggered::Triggered);
        }

        match Pin::new(&mut self.recv).poll(cx) {
            Poll::Ready(Ok(_)) => {
                self.triggered = Triggered::Triggered;
                Poll::Ready(true)
            }
            Poll::Ready(Err(_)) => {
                self.triggered = Triggered::Dropped;
                Poll::Ready(false)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

/// Creates a new one-time exclusive-pair for triggering & waiting on single untyped
/// event across asynchronous tasks.
///
/// The function returns separate "trigger" and "waiter" handles. The [`OnceTrigger`]
/// handle is used by the producer to trigger the event. The [`OnceWaiter`] handle is
/// used by the consumer to wait for the event.
///
/// Each handle can be used on separate tasks.
///
/// # Examples
///
/// ```
/// use est::sync::once::once_event;
///
/// #[tokio::main]
/// async fn main() {
///     let (trigger, waiter) = once_event();
///
///     tokio::spawn(async move {
///         if trigger.trigger() {
///             println!("event triggered");
///         } else {
///             println!("the waiter dropped");
///         }
///     });
///
///     if waiter.await {
///         println!("event received");
///     } else {
///         println!("the trigger dropped");
///     }
/// }
/// ```
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

        let (trigger, mut waiter) = once_event();
        tokio::spawn(async move {
            assert!(trigger.trigger());
        });
        while waiter.triggered() == Triggered::Pending {}
        assert_eq!(waiter.triggered(), Triggered::Triggered);
        assert!(waiter.await);

        let (trigger, mut waiter) = once_event();
        drop(trigger);
        while waiter.triggered() == Triggered::Pending {}
        assert_eq!(waiter.triggered(), Triggered::Dropped);
        assert!(!waiter.await);

        let (trigger, mut waiter) = once_event();
        assert_eq!(waiter.triggered(), Triggered::Pending);
        tokio::spawn(async move {
            assert!(trigger.trigger());
        });
        assert!((&mut waiter).await);
        assert_eq!(waiter.triggered(), Triggered::Triggered);
        assert_eq!(waiter.has_been_triggered(), Triggered::Triggered);

        let (trigger, mut waiter) = once_event();
        assert_eq!(waiter.triggered(), Triggered::Pending);
        drop(trigger);
        assert!(!(&mut waiter).await);
        assert_eq!(waiter.triggered(), Triggered::Dropped);
        assert_eq!(waiter.has_been_triggered(), Triggered::Dropped);
    }

    #[test]
    fn blocking_wait() {
        use std::thread;

        let (trigger, waiter) = once_event();
        thread::spawn(move || {
            assert!(trigger.trigger());
        });
        assert!(waiter.blocking_wait());

        let (trigger, waiter) = once_event();
        drop(waiter);
        assert!(!trigger.trigger());

        let (trigger, waiter) = once_event();
        drop(trigger);
        assert!(!waiter.blocking_wait());

        let (trigger, mut waiter) = once_event();
        thread::spawn(move || {
            assert!(trigger.trigger());
        });
        while waiter.triggered() == Triggered::Pending {}
        assert_eq!(waiter.triggered(), Triggered::Triggered);
        assert!(waiter.blocking_wait());

        let (trigger, mut waiter) = once_event();
        drop(trigger);
        while waiter.triggered() == Triggered::Pending {}
        assert_eq!(waiter.triggered(), Triggered::Dropped);
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
        assert!(!trigger.is_dropped());
        assert_eq!(waiter.has_been_triggered(), Triggered::Pending);
        assert!(trigger.is_dropped());
        assert!(!trigger.trigger());

        let (trigger, waiter) = once_event();
        assert!(trigger.trigger());
        assert_eq!(waiter.has_been_triggered(), Triggered::Triggered);

        let (trigger, waiter) = once_event();
        drop(trigger);
        assert_eq!(waiter.has_been_triggered(), Triggered::Dropped);
    }

    #[test]
    fn is_dropped() {
        let (trigger, waiter) = once_event();
        assert!(!trigger.is_dropped());
        drop(waiter);
        assert!(trigger.is_dropped());
        assert!(!trigger.trigger());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn dropped() {
        let (mut trigger, waiter) = once_event();
        assert!(!trigger.is_dropped());

        tokio::spawn(async move {
            drop(waiter);
        });

        trigger.dropped().await;
        assert!(trigger.is_dropped());
        assert!(!trigger.trigger());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn poll_dropped() {
        use std::future::poll_fn;

        let (mut trigger, waiter) = once_event();
        assert!(!trigger.is_dropped());

        tokio::spawn(async move {
            drop(waiter);
        });

        poll_fn(|cx| trigger.poll_dropped(cx)).await;
        assert!(trigger.is_dropped());
        assert!(!trigger.trigger());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn select_waiter() {
        use std::time::Duration;
        use tokio::time::{interval as _interval, sleep};

        let mut ticks = 0;
        let mut interval = _interval(Duration::from_millis(200));
        let (trigger, mut waiter) = once_event();

        tokio::spawn(async move {
            sleep(Duration::from_millis(500)).await;
            trigger.trigger();
        });

        loop {
            tokio::select! {
                _ = interval.tick() => ticks += 1,
                _ = &mut waiter => break,
            }

            assert_eq!(waiter.triggered(), Triggered::Pending);
        }

        assert_eq!(ticks, 3);
        assert_eq!(waiter.triggered(), Triggered::Triggered);
        assert_eq!(waiter.has_been_triggered(), Triggered::Triggered);

        let mut ticks = 0;
        let mut interval = _interval(Duration::from_millis(200));
        let (trigger, mut waiter) = once_event();

        tokio::spawn(async move {
            sleep(Duration::from_millis(500)).await;
            drop(trigger);
        });

        loop {
            tokio::select! {
                _ = interval.tick() => ticks += 1,
                _ = &mut waiter => break,
            }

            assert_eq!(waiter.triggered(), Triggered::Pending);
        }

        assert_eq!(ticks, 3);
        assert_eq!(waiter.triggered(), Triggered::Dropped);
        assert_eq!(waiter.has_been_triggered(), Triggered::Dropped);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn select_dropped() {
        use std::time::Duration;
        use tokio::time::sleep;
        use tokio_util::time::FutureExt;

        let timeout = Duration::from_millis(100);

        let (mut trigger, waiter) = once_event();
        tokio::spawn(async move {
            tokio::select! {
                _ = trigger.dropped() => (),
                _ = sleep(Duration::from_millis(500)) => {
                    trigger.trigger();
                }
            }
        });
        assert!(waiter.timeout(timeout).await.is_err());

        let (mut trigger, waiter) = once_event();
        tokio::spawn(async move {
            tokio::select! {
                _ = trigger.dropped() => (),
                _ = sleep(Duration::from_millis(5)) => {
                    drop(trigger);
                }
            }
        });
        assert_eq!(waiter.timeout(timeout).await, Ok(false));

        let (mut trigger, waiter) = once_event();
        tokio::spawn(async move {
            tokio::select! {
                _ = trigger.dropped() => (),
                _ = sleep(Duration::from_millis(5)) => {
                    trigger.trigger();
                }
            }
        });
        assert_eq!(waiter.timeout(timeout).await, Ok(true));
    }
}
