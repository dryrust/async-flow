// This is free and unencumbered software released into the public domain.

use super::{BlockDefinition, InputId, Inputs, OutputId, Outputs, SystemDefinition};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SystemBuildError {
    #[error("unregistered input port ID: {0}")]
    UnregisteredInput(InputId),
    #[error("unregistered output port ID: {0}")]
    UnregisteredOutput(OutputId),
}

/// A builder for system definitions.
///
/// # Examples
///
/// ```
/// use async_flow::model::SystemBuilder;
///
/// let mut builder = SystemBuilder::new();
/// //let block = builder.register(MyBlock::new());
/// let system = builder.build();
/// ```
#[derive(Clone, Debug, Default)]
pub struct SystemBuilder {
    system: SystemDefinition,
}

impl SystemBuilder {
    /// Creates a new system builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a block.
    pub fn register<T: BlockDefinition>(&mut self, block: T) -> T {
        for input in block.inputs() {
            self.register_input(input);
        }
        for output in block.outputs() {
            self.register_output(output);
        }
        block
    }

    /// Registers a block's input port.
    pub fn register_input(&mut self, input: impl Into<InputId>) {
        self.system.registered_inputs.insert(input.into());
    }

    /// Registers a block's output port.
    pub fn register_output(&mut self, output: impl Into<OutputId>) {
        self.system.registered_outputs.insert(output.into());
    }

    /// Connects an output port to an input port of the same type.
    ///
    /// Returns a boolean indicating whether the connection was newly
    /// inserted or already existed.
    pub fn connect<T>(
        &mut self,
        output: &Outputs<T>,
        input: &Inputs<T>,
    ) -> Result<bool, SystemBuildError> {
        if !self.system.registered_inputs.contains(&input.id()) {
            return Err(SystemBuildError::UnregisteredInput(input.id()));
        }
        if !self.system.registered_outputs.contains(&output.id()) {
            return Err(SystemBuildError::UnregisteredOutput(output.id()));
        }
        Ok(self.system.connections.insert((output.id(), input.id())))
    }

    /// Builds the system.
    pub fn build(self) -> SystemDefinition {
        self.system
    }
}
