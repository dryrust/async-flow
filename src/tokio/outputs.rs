// This is free and unencumbered software released into the public domain.

use crate::{PortDirection, PortState, io::SendError};
use alloc::{borrow::Cow, boxed::Box};
use dogma::{MaybeLabeled, MaybeNamed};
use tokio::sync::mpsc::Sender;

#[derive(Clone, Default)]
pub struct Outputs<T, const N: usize = 0> {
    pub(crate) tx: Option<Sender<T>>,
}

impl<T, const N: usize> core::fmt::Debug for Outputs<T, N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Outputs").field("tx", &self.tx).finish()
    }
}

impl<T, const N: usize> Outputs<T, N> {
    pub fn close(&mut self) {
        let _ = self.tx.take();
    }

    pub fn direction(&self) -> PortDirection {
        PortDirection::Output
    }

    pub fn state(&self) -> PortState {
        if self.tx.as_ref().map(|tx| tx.is_closed()).unwrap_or(true) {
            PortState::Closed
        } else {
            PortState::Open
        }
    }

    /// Checks whether this port is currently closed.
    pub fn is_closed(&self) -> bool {
        self.state().is_closed()
    }

    /// Checks whether this port is currently open.
    pub fn is_open(&self) -> bool {
        self.state().is_open()
    }

    /// Checks whether this port is currently connected.
    pub fn is_connected(&self) -> bool {
        self.state().is_connected()
    }

    pub fn capacity(&self) -> Option<usize> {
        self.tx.as_ref().map(|tx| tx.capacity())
    }

    pub fn max_capacity(&self) -> Option<usize> {
        self.tx.as_ref().map(|tx| tx.max_capacity())
    }

    pub async fn send(&self, value: T) -> Result<(), SendError> {
        if let Some(tx) = self.tx.as_ref() {
            Ok(tx.send(value).await?)
        } else {
            Err(SendError) // TODO: SendError::Closed
        }
    }

    pub fn send_blocking(&self, value: T) -> Result<(), SendError> {
        if let Some(tx) = self.tx.as_ref() {
            Ok(tx.blocking_send(value)?)
        } else {
            Err(SendError) // TODO: SendError::Closed
        }
    }
}

impl<T, const N: usize> AsRef<Sender<T>> for Outputs<T, N> {
    fn as_ref(&self) -> &Sender<T> {
        self.tx.as_ref().unwrap()
    }
}

impl<T, const N: usize> AsMut<Sender<T>> for Outputs<T, N> {
    fn as_mut(&mut self) -> &mut Sender<T> {
        self.tx.as_mut().unwrap()
    }
}

impl<T, const N: usize> From<Sender<T>> for Outputs<T, N> {
    fn from(input: Sender<T>) -> Self {
        Self { tx: Some(input) }
    }
}

impl<T, const N: usize> From<&Sender<T>> for Outputs<T, N> {
    fn from(input: &Sender<T>) -> Self {
        Self {
            tx: Some(input.clone()),
        }
    }
}

#[async_trait::async_trait]
impl<T: Send + 'static, const N: usize> crate::io::OutputPort<T> for Outputs<T, N> {
    async fn send(&self, value: T) -> Result<(), SendError> {
        self.send(value).await
    }
}

impl<T: Send, const N: usize> crate::io::Port<T> for Outputs<T, N> {
    fn close(&mut self) {
        self.close()
    }

    fn direction(&self) -> PortDirection {
        self.direction()
    }

    fn state(&self) -> PortState {
        self.state()
    }
}

impl<T, const N: usize> MaybeNamed for Outputs<T, N> {
    fn name(&self) -> Option<Cow<'_, str>> {
        None
    }
}

impl<T, const N: usize> MaybeLabeled for Outputs<T, N> {
    fn label(&self) -> Option<Cow<'_, str>> {
        None
    }
}
