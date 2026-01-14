// This is free and unencumbered software released into the public domain.

use crate::{InputPort, Port};
use alloc::{borrow::Cow, boxed::Box};
use dogma::{MaybeLabeled, MaybeNamed};
use tokio::sync::mpsc::Receiver;

pub struct BoundedInputPort<T> {
    pub receiver: Receiver<T>,
}

impl<T> core::fmt::Debug for BoundedInputPort<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("BoundedInputPort")
            .field("receiver", &self.receiver)
            .finish()
    }
}

impl<T> BoundedInputPort<T> {
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

    pub async fn recv(&mut self) -> Option<T> {
        self.receiver.recv().await
    }
}

impl<T> AsRef<Receiver<T>> for BoundedInputPort<T> {
    fn as_ref(&self) -> &Receiver<T> {
        &self.receiver
    }
}

impl<T> AsMut<Receiver<T>> for BoundedInputPort<T> {
    fn as_mut(&mut self) -> &mut Receiver<T> {
        &mut self.receiver
    }
}

impl<T> From<Receiver<T>> for BoundedInputPort<T> {
    fn from(input: Receiver<T>) -> Self {
        Self { receiver: input }
    }
}

#[async_trait::async_trait]
impl<T: Send> InputPort<T> for BoundedInputPort<T> {
    fn is_empty(&self) -> bool {
        self.receiver.is_empty()
    }

    async fn recv(&mut self) -> Option<T> {
        self.recv().await
    }
}

impl<T> Port<T> for BoundedInputPort<T> {
    fn is_closed(&self) -> bool {
        self.receiver.is_closed()
    }

    fn close(&mut self) {
        self.receiver.close()
    }
}

impl<T> MaybeLabeled for BoundedInputPort<T> {
    fn label(&self) -> Option<Cow<'_, str>> {
        None
    }
}

impl<T> MaybeNamed for BoundedInputPort<T> {
    fn name(&self) -> Option<Cow<'_, str>> {
        None
    }
}
