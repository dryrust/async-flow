// This is free and unencumbered software released into the public domain.

use alloc::{collections::BTreeSet, vec::Vec};
use core::{fmt::Debug, ops::RangeInclusive};

/// A set of port IDs.
#[derive(Clone)]
pub struct PortIdSet<K>(BTreeSet<K>)
where
    K: Into<isize>;

impl<K: Into<isize>> Default for PortIdSet<K> {
    fn default() -> Self {
        Self(BTreeSet::default())
    }
}

impl<K: Into<isize> + Copy + Ord> From<&Vec<K>> for PortIdSet<K> {
    fn from(input: &Vec<K>) -> Self {
        Self(BTreeSet::<K>::from_iter(input.iter().cloned()))
    }
}

impl<K: Into<isize> + Ord> PortIdSet<K> {
    pub fn new() -> Self {
        Self(BTreeSet::new())
    }

    pub fn first(&self) -> Option<&K> {
        self.0.first()
    }

    pub fn last(&self) -> Option<&K> {
        self.0.last()
    }

    pub fn insert(&mut self, id: K) -> bool {
        self.0.insert(id)
    }

    pub fn contains(&self, id: K) -> bool {
        self.0.contains(&id)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &K> {
        self.0.iter()
    }
}

impl<K: Into<isize> + Copy + Ord> PortIdSet<K> {
    pub fn range(&self) -> Option<RangeInclusive<K>> {
        let Some(&min) = self.first() else {
            return None;
        };
        let Some(&max) = self.last() else {
            unreachable!()
        };
        Some(min..=max)
    }
}

impl<K: Into<isize> + Copy> Debug for PortIdSet<K> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_list()
            .entries(
                &self
                    .0
                    .iter()
                    .map(|id| Into::<isize>::into(*id))
                    .collect::<Vec<_>>(),
            )
            .finish()
    }
}
