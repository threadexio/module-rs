use core::cmp::Ord;

use alloc::boxed::Box;
use alloc::collections::{BTreeMap, BTreeSet, LinkedList};
use alloc::vec::Vec;

use super::prelude::*;

impl<T> Merge for Box<T>
where
    T: Merge,
{
    fn merge(self, other: Self) -> Result<Self, Error> {
        (*self).merge(*other).map(Box::new)
    }
}

impl<T> Merge for Vec<T> {
    fn merge(mut self, mut other: Self) -> Result<Self, Error> {
        self.append(&mut other);
        Ok(self)
    }
}

impl<T> Merge for LinkedList<T> {
    fn merge(mut self, mut other: Self) -> Result<Self, Error> {
        self.append(&mut other);
        Ok(self)
    }
}

impl<K, V> Merge for BTreeMap<K, V>
where
    K: Ord,
    V: Merge,
{
    fn merge(mut self, other: Self) -> Result<Self, Error> {
        use alloc::collections::btree_map::Entry;

        for (k, b) in other {
            match self.entry(k) {
                Entry::Vacant(x) => {
                    x.insert(b);
                }
                Entry::Occupied(x) => {
                    let (k, a) = x.remove_entry();
                    self.insert(k, a.merge(b)?);
                }
            }
        }

        Ok(self)
    }
}

impl<T> Merge for BTreeSet<T>
where
    T: Ord,
{
    fn merge(mut self, mut other: Self) -> Result<Self, Error> {
        self.append(&mut other);
        Ok(self)
    }
}
