// This is free and unencumbered software released into the public domain.

use tokio::sync::mpsc::Receiver;

#[derive(Debug)]
pub struct InputPort<T> {
    pub(crate) receiver: Receiver<T>,
}

impl<T> InputPort<T> {
    pub fn as_receiver(&self) -> &Receiver<T> {
        &self.receiver
    }

    pub fn as_receiver_mut(&mut self) -> &mut Receiver<T> {
        &mut self.receiver
    }

    pub async fn recv(&mut self) -> Option<T> {
        self.receiver.recv().await
    }
}

impl<T> From<Receiver<T>> for InputPort<T> {
    fn from(input: Receiver<T>) -> Self {
        Self { receiver: input }
    }
}
