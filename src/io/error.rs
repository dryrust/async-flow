// This is free and unencumbered software released into the public domain.

use super::{SendError, TryRecvError, TrySendError};

pub type Result<T = (), E = ()> = core::result::Result<T, SendError<E>>; // FIXME
