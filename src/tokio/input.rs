// This is free and unencumbered software released into the public domain.

use crate::io::RecvError;
use alloc::{borrow::Cow, boxed::Box};
use dogma::{MaybeLabeled, MaybeNamed};
use tokio::sync::mpsc::Receiver;

pub struct Input<T> {
    pub receiver: Receiver<T>,
}

impl<T> core::fmt::Debug for Input<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("InputPort")
            .field("receiver", &self.receiver)
            .finish()
    }
}

impl<T> Input<T> {
    pub(crate) fn as_receiver(&self) -> &Receiver<T> {
        &self.receiver
    }

    pub(crate) fn as_receiver_mut(&mut self) -> &mut Receiver<T> {
        &mut self.receiver
    }

    pub(crate) fn into_receiver(self) -> Receiver<T> {
        self.receiver
    }

    pub fn capacity(&self) -> Option<usize> {
        Some(self.receiver.capacity())
    }

    pub fn max_capacity(&self) -> Option<usize> {
        Some(self.receiver.max_capacity())
    }

    pub async fn recv(&mut self) -> Result<Option<T>, RecvError> {
        Ok(self.receiver.recv().await)
    }
}

impl<T> AsRef<Receiver<T>> for Input<T> {
    fn as_ref(&self) -> &Receiver<T> {
        &self.receiver
    }
}

impl<T> AsMut<Receiver<T>> for Input<T> {
    fn as_mut(&mut self) -> &mut Receiver<T> {
        &mut self.receiver
    }
}

impl<T> From<Receiver<T>> for Input<T> {
    fn from(input: Receiver<T>) -> Self {
        Self { receiver: input }
    }
}

#[async_trait::async_trait]
impl<T: Send> crate::io::InputPort<T> for Input<T> {
    fn is_empty(&self) -> bool {
        self.receiver.is_empty()
    }

    async fn recv(&mut self) -> Result<Option<T>, RecvError> {
        self.recv().await
    }
}

impl<T> crate::io::Port<T> for Input<T> {
    fn is_closed(&self) -> bool {
        self.receiver.is_closed()
    }

    fn close(&mut self) {
        self.receiver.close()
    }
}

impl<T> MaybeLabeled for Input<T> {
    fn label(&self) -> Option<Cow<'_, str>> {
        None
    }
}

impl<T> MaybeNamed for Input<T> {
    fn name(&self) -> Option<Cow<'_, str>> {
        None
    }
}
