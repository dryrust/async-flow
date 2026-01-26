// This is free and unencumbered software released into the public domain.

use core::{marker::PhantomData, ops::Bound};

/// A one-shot input port of type `T`.
///
/// Note that `Input` implements `Copy`, whereas `Output` doesn't.
pub type Input<T> = Inputs<T, 1, 0>;

/// An input port of type `T`.
///
/// Note that `Inputs` implements `Copy`, whereas `Outputs` doesn't.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Inputs<T, const MAX: isize = -1, const MIN: isize = 0>(PhantomData<T>);

impl<T, const MAX: isize, const MIN: isize> Default for Inputs<T, MAX, MIN> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T, const MAX: isize, const MIN: isize> Inputs<T, MAX, MIN> {
    /// Returns the cardinality of this connection.
    pub fn cardinality() -> (Bound<usize>, Bound<usize>) {
        assert!(MIN >= 0);
        assert!(MAX >= -1);
        use Bound::*;
        match (MIN, MAX) {
            (min, -1) => (Included(min as _), Unbounded),
            (min, max) => (Included(min as _), Included(max as _)),
        }
    }
}
