// This is free and unencumbered software released into the public domain.

use core::any::TypeId;

#[async_trait::async_trait]
pub trait Connection<T: 'static> {
    fn type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }
}
