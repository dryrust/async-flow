// This is free and unencumbered software released into the public domain.

use super::{RecvError, SendError, TryRecvError, TrySendError};
use thiserror::Error;

pub type Result<T = (), E = ()> = core::result::Result<T, Error<E>>;

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum Error<T = ()> {
    RecvError(#[from] RecvError),
    TryRecvError(#[from] TryRecvError),
    SendError(#[from] SendError<T>),
    TrySendError(#[from] TrySendError<T>),
}
