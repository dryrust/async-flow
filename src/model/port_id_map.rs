// This is free and unencumbered software released into the public domain.

use alloc::collections::BTreeMap;
use core::{fmt::Debug, ops::RangeInclusive};

/// A map of port IDs to arbitrary values.
#[derive(Clone)]
pub struct PortIdMap<K, V>(BTreeMap<K, V>)
where
    K: Into<isize>,
    V: Debug;

impl<K: Into<isize>, V: Debug> Default for PortIdMap<K, V> {
    fn default() -> Self {
        Self(BTreeMap::default())
    }
}

impl<K: Into<isize> + Ord, V: Debug> PortIdMap<K, V> {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn first(&self) -> Option<&K> {
        self.0.first_key_value().map(|(k, _)| k)
    }

    pub fn last(&self) -> Option<&K> {
        self.0.last_key_value().map(|(k, _)| k)
    }

    pub fn insert(&mut self, id: K, value: V) -> bool {
        self.0.insert(id, value).is_none()
    }

    pub fn contains(&self, id: K) -> bool {
        self.0.contains_key(&id)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> alloc::collections::btree_map::Iter<'_, K, V> {
        self.0.iter()
    }
}

impl<K: Into<isize> + Copy + Ord, V: Debug> PortIdMap<K, V> {
    pub fn range(&self) -> Option<RangeInclusive<isize>> {
        let Some(&min) = self.first() else {
            return None;
        };
        let Some(&max) = self.last() else {
            unreachable!()
        };
        Some(min.into()..=max.into())
    }
}

impl<K: Into<isize> + Copy, V: Debug> Debug for PortIdMap<K, V> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_map()
            .entries(self.0.iter().map(|(k, v)| (Into::<isize>::into(*k), v)))
            .finish()
    }
}
