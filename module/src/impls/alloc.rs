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

unmergeable! {
    Box<core::ffi::CStr>, Box<str>,
    alloc::ffi::CString,
    alloc::string::String
}

impl<T> Merge for Box<[T]> {
    unmergeable!();
}

impl<T> Merge for alloc::borrow::Cow<'_, T>
where
    T: ?Sized + alloc::borrow::ToOwned,
    T::Owned: Merge,
{
    fn merge(self, other: Self) -> Result<Self, Error> {
        let a = self.into_owned();
        let b = other.into_owned();

        a.merge(b).map(Self::Owned)
    }
}
