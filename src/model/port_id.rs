// This is free and unencumbered software released into the public domain.

/// A input or output port identifier.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum PortId {
    Input(InputPortId),
    Output(OutputPortId),
}

impl PortId {
    pub fn as_isize(&self) -> isize {
        match self {
            PortId::Input(id) => id.0,
            PortId::Output(id) => id.0,
        }
    }

    pub fn as_usize(&self) -> usize {
        self.as_isize() as _
    }
}

impl TryFrom<isize> for PortId {
    type Error = &'static str;

    fn try_from(id: isize) -> Result<Self, Self::Error> {
        if id < 0 {
            Ok(Self::Input(InputPortId(id)))
        } else if id > 0 {
            Ok(Self::Output(OutputPortId(id)))
        } else {
            Err("Port IDs cannot be zero")
        }
    }
}

impl From<InputPortId> for PortId {
    fn from(input: InputPortId) -> Self {
        PortId::Input(input)
    }
}

impl From<OutputPortId> for PortId {
    fn from(input: OutputPortId) -> Self {
        PortId::Output(input)
    }
}

impl Into<isize> for PortId {
    fn into(self) -> isize {
        self.as_isize()
    }
}

impl Into<usize> for PortId {
    fn into(self) -> usize {
        self.as_usize()
    }
}

impl core::fmt::Display for PortId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            PortId::Input(id) => write!(f, "{}", id),
            PortId::Output(id) => write!(f, "{}", id),
        }
    }
}

/// An input port identifier.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InputPortId(pub(crate) isize);

impl InputPortId {
    #[doc(hidden)]
    pub fn index(&self) -> usize {
        self.0.unsigned_abs() - 1
    }
}

impl TryFrom<isize> for InputPortId {
    type Error = &'static str;

    fn try_from(id: isize) -> Result<Self, Self::Error> {
        if id < 0 {
            Ok(InputPortId(id))
        } else {
            Err("Input port IDs must be negative integers")
        }
    }
}

impl Into<isize> for InputPortId {
    fn into(self) -> isize {
        self.0
    }
}

impl Into<usize> for InputPortId {
    fn into(self) -> usize {
        self.0.unsigned_abs()
    }
}

impl core::fmt::Display for InputPortId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// An output port identifier.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OutputPortId(pub(crate) isize);

impl OutputPortId {
    #[doc(hidden)]
    pub fn index(&self) -> usize {
        (self.0 as usize) - 1
    }
}

impl TryFrom<isize> for OutputPortId {
    type Error = &'static str;

    fn try_from(input: isize) -> Result<Self, Self::Error> {
        if input > 0 {
            Ok(OutputPortId(input))
        } else {
            Err("Output port IDs must be positive integers")
        }
    }
}

impl Into<isize> for OutputPortId {
    fn into(self) -> isize {
        self.0
    }
}

impl Into<usize> for OutputPortId {
    fn into(self) -> usize {
        self.0 as usize
    }
}

impl core::fmt::Display for OutputPortId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}
