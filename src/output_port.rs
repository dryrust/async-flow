// This is free and unencumbered software released into the public domain.

use tokio::sync::mpsc::{Sender, error::SendError};

#[derive(Clone, Debug)]
pub struct OutputPort<T> {
    pub(crate) sender: Sender<T>,
}

impl<T> OutputPort<T> {
    pub fn as_sender(&self) -> &Sender<T> {
        &self.sender
    }

    pub async fn send(&self, value: T) -> Result<(), SendError<T>> {
        self.sender.send(value).await
    }
}

impl<T> From<Sender<T>> for OutputPort<T> {
    fn from(input: Sender<T>) -> Self {
        Self { sender: input }
    }
}

impl<T> From<&Sender<T>> for OutputPort<T> {
    fn from(input: &Sender<T>) -> Self {
        Self {
            sender: input.clone(),
        }
    }
}
