use super::TaskId;
use crate::{
    future::IntoFutureWithArgs,
    sync::once::{OnceTrigger, once_event},
};
use serde::{Deserialize, Serialize};
use std::{
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use tokio::{
    signal::ctrl_c,
    sync::{
        Mutex,
        watch::{Receiver, channel},
    },
    task::{JoinError, JoinHandle},
};

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash)]
pub enum GracefulKind {
    CtrlC,
    Explicit,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash)]
pub enum FinishKind {
    Active,
    Passive(GracefulKind),
}

#[derive(Debug)]
pub struct TaskOutput<T> {
    pub finish_kind: FinishKind,
    pub join_result: Result<T, JoinError>,
}

#[derive(Debug, Clone)]
pub struct ShutdownTrigger(Arc<Mutex<Option<OnceTrigger>>>);

impl ShutdownTrigger {
    pub fn trigger(&self) -> bool {
        match self.0.try_lock() {
            Err(_) => false,
            Ok(mut guard) => match guard.take() {
                None => false,
                Some(trigger) => trigger.trigger(),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct ShutdownReceiver(RecvInner);

#[derive(Debug, Clone)]
enum RecvInner {
    Pending(Receiver<Option<GracefulKind>>),
    Shutdown(GracefulKind),
}

impl ShutdownReceiver {
    pub async fn recv(&mut self) -> GracefulKind {
        match &mut self.0 {
            RecvInner::Pending(receiver) => {
                let init = *receiver.borrow_and_update();
                let kind = match init {
                    Some(kind) => kind,
                    None => {
                        // The `Sender` will never drop before the `Receiver` drops, so
                        // calling `changed()` here will always resolve to `Ok(())`. Therefore,
                        // the next `borrow_and_update()` call must return `Some`, so it can be
                        // unwrapped safely.
                        receiver.changed().await.ok();
                        receiver.borrow_and_update().unwrap()
                    }
                };

                self.0 = RecvInner::Shutdown(kind);
                kind
            }
            RecvInner::Shutdown(kind) => *kind,
        }
    }
}

impl IntoFuture for ShutdownReceiver {
    type Output = GracefulKind;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output>>>;

    fn into_future(mut self) -> Self::IntoFuture {
        Box::pin(async move { self.recv().await })
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct GracefulTaskBuilder {
    ctrlc_shutdown: bool,
}

impl GracefulTaskBuilder {
    pub fn ctrlc_shutdown(self) -> Self {
        Self {
            ctrlc_shutdown: true,
        }
    }

    pub fn spawn<T, F>(self, ifwa: T) -> GracefulTask<F::Output>
    where
        T: IntoFutureWithArgs<ShutdownReceiver, F>,
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let ctrlc_shutdown = self.ctrlc_shutdown;
        let (sender, recver) = channel(None);
        let (trigger, waiter) = once_event();
        let trigger = ShutdownTrigger(Arc::new(Mutex::new(Some(trigger))));
        let mut inner_task =
            tokio::spawn(ifwa.into_future_with_args(ShutdownReceiver(RecvInner::Pending(recver))));

        let inner = inner_task.id().into();
        let graceful = trigger.clone();
        let task = tokio::spawn(async move {
            let (finish_kind, join_result) = tokio::select! {
                _ = ctrl_c(), if ctrlc_shutdown => {
                    trigger.trigger();
                    let kind = GracefulKind::CtrlC;
                    sender.send(Some(kind)).ok();
                    (FinishKind::Passive(kind), inner_task.await)
                },
                _ = waiter => {
                    let kind = GracefulKind::Explicit;
                    sender.send(Some(kind)).ok();
                    (FinishKind::Passive(kind), inner_task.await)
                },
                join_result = &mut inner_task => (FinishKind::Active, join_result),
            };

            TaskOutput {
                finish_kind,
                join_result,
            }
        });
        let outer = task.id().into();

        GracefulTask {
            inner,
            outer,
            graceful,
            task,
        }
    }
}

#[derive(Debug)]
pub struct GracefulTask<T> {
    inner: TaskId,
    outer: TaskId,
    graceful: ShutdownTrigger,
    task: JoinHandle<TaskOutput<T>>,
}

impl<T> GracefulTask<T> {
    pub fn builder_default() -> GracefulTaskBuilder {
        GracefulTaskBuilder::default()
    }

    pub fn ids(&self) -> (TaskId, TaskId) {
        (self.outer, self.inner)
    }

    pub fn trigger_graceful_shutdown(&self) -> bool {
        self.graceful.trigger()
    }

    pub async fn graceful_shutdown(self) -> TaskOutput<T> {
        self.trigger_graceful_shutdown();
        self.await
    }

    pub fn shutdown_handle(&self) -> ShutdownTrigger {
        self.graceful.clone()
    }

    pub fn is_finished(&self) -> bool {
        self.task.is_finished()
    }
}

impl<T> Future for GracefulTask<T> {
    type Output = TaskOutput<T>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // `task` will never panic or be aborted, so it can be unwrapped safely.
        Pin::new(&mut self.task).poll(cx).map(Result::unwrap)
    }
}
