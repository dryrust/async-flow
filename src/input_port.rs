// This is free and unencumbered software released into the public domain.

use crate::Port;
use alloc::boxed::Box;

#[async_trait::async_trait]
pub trait InputPort<T>: Port<T> {
    /// Checks if a port is empty.
    fn is_empty(&self) -> bool;

    async fn recv(&mut self) -> Option<T>;

    // TODO: try_recv
}
