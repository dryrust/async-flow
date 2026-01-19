// This is free and unencumbered software released into the public domain.

/// The dataflow direction of a port.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum PortDirection {
    Input,
    Output,
}

impl PortDirection {
    /// Checks whether the port is an input port.
    pub fn is_input(&self) -> bool {
        *self == PortDirection::Input
    }

    /// Checks whether the port is an output port.
    pub fn is_output(&self) -> bool {
        *self == PortDirection::Output
    }

    pub fn as_str(&self) -> &str {
        use PortDirection::*;
        match self {
            Input => "input",
            Output => "output",
        }
    }
}

impl AsRef<str> for PortDirection {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
