// This is free and unencumbered software released into the public domain.

use super::{InputPortId, OutputPortId};
use alloc::vec::Vec;

pub use dogma::Named as BlockName;

/// A block definition.
pub trait BlockDefinition: BlockName {
    fn inputs(&self) -> Vec<InputPortId> {
        Vec::new()
    }

    fn outputs(&self) -> Vec<OutputPortId> {
        Vec::new()
    }
}
