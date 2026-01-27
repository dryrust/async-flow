// This is free and unencumbered software released into the public domain.

use super::{BlockDefinition, InputId, OutputId, SystemBuilder};
use alloc::{collections::BTreeSet, rc::Rc, vec::Vec};
use derive_more::Debug;

/// A system definition.
#[derive(Clone, Debug, Default)]
pub struct SystemDefinition {
    #[debug(skip)]
    pub(crate) blocks: Vec<Rc<dyn BlockDefinition>>,
    pub(crate) connections: BTreeSet<(OutputId, InputId)>,
}

impl SystemDefinition {
    /// Returns a system builder.
    pub fn build() -> SystemBuilder {
        SystemBuilder::new()
    }
}
