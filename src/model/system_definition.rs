// This is free and unencumbered software released into the public domain.

use super::{BlockDefinition, InputPortId, OutputPortId, SystemBuilder};
use alloc::{collections::BTreeSet, rc::Rc, vec::Vec};
use core::fmt::Debug;

/// A system definition.
#[derive(Clone, Default)]
pub struct SystemDefinition {
    pub(crate) inputs: BTreeSet<InputPortId>,
    pub(crate) outputs: BTreeSet<OutputPortId>,
    pub(crate) blocks: Vec<BlockHandle>,
    pub(crate) connections: BTreeSet<(OutputPortId, InputPortId)>,
}

impl SystemDefinition {
    /// Returns a system builder.
    pub fn build() -> SystemBuilder {
        SystemBuilder::new()
    }

    pub(crate) fn push_block<T: BlockDefinition + 'static>(&mut self, block: &Rc<T>) {
        self.blocks.push(BlockHandle(Rc::clone(&block) as _));
    }
}

impl Debug for SystemDefinition {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SystemDefinition")
            .field(
                "inputs",
                &self.inputs.iter().map(|id| id.0).collect::<Vec<_>>(),
            )
            .field(
                "outputs",
                &self.outputs.iter().map(|id| id.0).collect::<Vec<_>>(),
            )
            .field("blocks", &self.blocks)
            .field(
                "connections",
                &self
                    .connections
                    .iter()
                    .map(|(a, b)| (a.0, b.0))
                    .collect::<Vec<_>>(),
            )
            .finish()
    }
}

#[derive(Clone)]
pub(crate) struct BlockHandle(Rc<dyn BlockDefinition>);

impl Debug for BlockHandle {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct(&self.0.name())
            .field(
                "inputs",
                &self.0.inputs().iter().map(|id| id.0).collect::<Vec<_>>(),
            )
            .field(
                "outputs",
                &self.0.outputs().iter().map(|id| id.0).collect::<Vec<_>>(),
            )
            .finish()
    }
}
