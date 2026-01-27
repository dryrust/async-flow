// This is free and unencumbered software released into the public domain.

use super::{InputId, OutputId};
use alloc::vec::Vec;

pub use dogma::Named as BlockName;

/// A block definition.
pub trait BlockDefinition: BlockName {
    fn inputs(&self) -> Vec<InputId> {
        Vec::new()
    }

    fn outputs(&self) -> Vec<OutputId> {
        Vec::new()
    }
}
