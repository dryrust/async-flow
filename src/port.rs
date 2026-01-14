// This is free and unencumbered software released into the public domain.

use dogma::{MaybeLabeled, MaybeNamed};

/// The common interface for ports, whether for input or output.
pub trait Port: MaybeNamed + MaybeLabeled {}
