// This is free and unencumbered software released into the public domain.

use crate::error::SendError;
use alloc::boxed::Box;
use core::any::TypeId;

#[async_trait::async_trait]
pub trait OutputPort<T: Send + 'static> {
    fn type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }

    async fn send(&self, message: T) -> Result<(), SendError>;

    // TODO: send_event
    // TODO: send_deadline
    // TODO: send_timeout
    // TODO: try_send
}
