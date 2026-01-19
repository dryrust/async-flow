// This is free and unencumbered software released into the public domain.

use crate::{PortDirection, PortEvent, PortState, io::RecvError};
use alloc::{borrow::Cow, boxed::Box};
use dogma::{MaybeLabeled, MaybeNamed};
use tokio::sync::mpsc::Receiver;

#[derive(Default)]
pub struct Inputs<T, const N: usize = 0> {
    pub(crate) rx: Option<Receiver<PortEvent<T>>>,
}

impl<T, const N: usize> core::fmt::Debug for Inputs<T, N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Inputs").field("rx", &self.rx).finish()
    }
}

impl<T, const N: usize> Inputs<T, N> {
    pub fn close(&mut self) {
        if let Some(rx) = self.rx.as_mut() {
            if !rx.is_closed() {
                rx.close() // idempotent
            }
        }
    }

    pub fn direction(&self) -> PortDirection {
        PortDirection::Input
    }

    pub fn state(&self) -> PortState {
        if self.rx.as_ref().map(|rx| rx.is_closed()).unwrap_or(true) {
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

    pub fn is_empty(&self) -> bool {
        self.rx.as_ref().map(|rx| rx.is_empty()).unwrap_or(true)
    }

    pub fn capacity(&self) -> Option<usize> {
        self.rx.as_ref().map(|rx| rx.capacity())
    }

    pub fn max_capacity(&self) -> Option<usize> {
        self.rx.as_ref().map(|rx| rx.max_capacity())
    }

    pub async fn recv(&mut self) -> Result<Option<T>, RecvError> {
        loop {
            return match self.recv_event().await? {
                Some(PortEvent::Message(m)) => Ok(Some(m)),
                Some(PortEvent::Connect) => continue, // TODO
                Some(PortEvent::Disconnect) => Ok(None),
                None => Ok(None),
            };
        }
    }

    pub async fn recv_event(&mut self) -> Result<Option<PortEvent<T>>, RecvError> {
        match self.rx.as_mut() {
            Some(rx) => Ok(rx.recv().await),
            None => Ok(None),
        }
    }

    pub fn recv_blocking(&mut self) -> Result<Option<T>, RecvError> {
        todo!() // TODO
    }
}

impl<T, const N: usize> AsRef<Receiver<PortEvent<T>>> for Inputs<T, N> {
    fn as_ref(&self) -> &Receiver<PortEvent<T>> {
        self.rx.as_ref().unwrap()
    }
}

impl<T, const N: usize> AsMut<Receiver<PortEvent<T>>> for Inputs<T, N> {
    fn as_mut(&mut self) -> &mut Receiver<PortEvent<T>> {
        self.rx.as_mut().unwrap()
    }
}

impl<T, const N: usize> From<Receiver<PortEvent<T>>> for Inputs<T, N> {
    fn from(input: Receiver<PortEvent<T>>) -> Self {
        Self { rx: Some(input) }
    }
}

#[async_trait::async_trait]
impl<T: Send + 'static, const N: usize> crate::io::InputPort<T> for Inputs<T, N> {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    async fn recv(&mut self) -> Result<Option<T>, RecvError> {
        self.recv().await
    }
}

impl<T: Send, const N: usize> crate::io::Port<T> for Inputs<T, N> {
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

impl<T, const N: usize> MaybeNamed for Inputs<T, N> {
    fn name(&self) -> Option<Cow<'_, str>> {
        None
    }
}

impl<T, const N: usize> MaybeLabeled for Inputs<T, N> {
    fn label(&self) -> Option<Cow<'_, str>> {
        None
    }
}
