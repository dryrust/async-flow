// This is free and unencumbered software released into the public domain.

use crate::{InputPort, OutputPort};

pub fn bounded<T>(buffer: usize) -> (OutputPort<T>, InputPort<T>) {
    let (tx, rx) = tokio::sync::mpsc::channel(buffer);
    let output = OutputPort::from(tx);
    let input = InputPort::from(rx);
    (output, input)
}
