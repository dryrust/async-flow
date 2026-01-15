// This is free and unencumbered software released into the public domain.

use crate::io::Result;
use alloc::string::{String, ToString};

#[cfg(feature = "tokio")]
pub async fn stdout<T: ToString>(mut inputs: crate::tokio::Inputs<T>) -> Result {
    while let Some(input) = inputs.recv().await? {
        std::println!("{}", input.to_string());
    }

    Ok(())
}
