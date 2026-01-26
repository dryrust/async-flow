// This is free and unencumbered software released into the public domain.

use core::{marker::PhantomData, ops::Bound};

/// A one-shot output port of type `T`.
///
/// Note that `Output` doesn't implement `Copy`, whereas `Input` does.
pub type Output<T> = Outputs<T, 1, 0>;

/// An output port of type `T`.
///
/// Note that `Outputs` doesn't implement `Copy`, whereas `Inputs` does.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Outputs<T, const MAX: isize = -1, const MIN: isize = 0>(PhantomData<T>);

impl<T, const MAX: isize, const MIN: isize> Default for Outputs<T, MAX, MIN> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T, const MAX: isize, const MIN: isize> Outputs<T, MAX, MIN> {
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
