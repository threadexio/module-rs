use core::cmp::Eq;
use core::hash::{BuildHasher, Hash};

use std::collections::{HashMap, HashSet};

use super::prelude::*;

impl<K, V, S> Merge for HashMap<K, V, S>
where
    K: Eq + Hash,
    V: Merge,
    S: BuildHasher,
{
    fn merge(mut self, other: Self) -> Result<Self, Error> {
        use std::collections::hash_map::Entry;

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

impl<T, S> Merge for HashSet<T, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
    fn merge(mut self, other: Self) -> Result<Self, Error> {
        self.extend(other);
        Ok(self)
    }
}
