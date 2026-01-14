// This is free and unencumbered software released into the public domain.

use thiserror::Error;

pub use tokio::sync::mpsc::error::TryRecvError;

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("RecvError")]
pub struct RecvError;
