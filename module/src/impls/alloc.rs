use core::cmp::Ord;

use alloc::boxed::Box;
use alloc::collections::{BTreeMap, BTreeSet, LinkedList};
use alloc::vec::Vec;

use super::prelude::*;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    struct Merged(bool);

    impl Merge for Merged {
        fn merge(self, _: Self) -> Result<Self, Error> {
            Ok(Self(true))
        }
    }

    #[test]
    fn test_box() {
        let a = Box::new(Merged(false));
        let b = Box::new(Merged(false));

        let c = a.merge(b).unwrap();
        assert!((*c).0);
    }

    #[test]
    fn test_vec() {
        use alloc::vec;

        let a = vec![1, 2, 5, 7, 0];
        let b = vec![2, 8, 9, 10];

        let c = a.merge(b).unwrap();
        assert_eq!(c, &[1, 2, 5, 7, 0, 2, 8, 9, 10]);
    }

    #[test]
    fn test_linked_list() {
        let a: LinkedList<i32> = [1, 2, 5, 7, 0].into_iter().collect();
        let b: LinkedList<i32> = [2, 8, 9, 10].into_iter().collect();

        let c = a.merge(b).unwrap();
        assert!(c.iter().eq(&[1, 2, 5, 7, 0, 2, 8, 9, 10]));
    }

    #[test]
    fn test_btree_map() {
        let a: BTreeMap<&'static str, Merged> = [
            ("key1", Merged(false)),
            ("key2", Merged(false)),
            ("key3", Merged(false)),
            ("key4", Merged(false)),
            ("key7", Merged(false)),
        ]
        .into_iter()
        .collect();

        let b: BTreeMap<&'static str, Merged> = [
            ("key5", Merged(false)),
            ("key1", Merged(false)),
            ("key7", Merged(false)),
            ("key2", Merged(false)),
            ("key6", Merged(false)),
        ]
        .into_iter()
        .collect();

        let c = a.merge(b).unwrap();

        let expected = [
            ("key1", Merged(true)),
            ("key2", Merged(true)),
            ("key3", Merged(false)),
            ("key4", Merged(false)),
            ("key5", Merged(false)),
            ("key6", Merged(false)),
            ("key7", Merged(true)),
        ];

        for (k, v) in expected {
            assert_eq!(c[k].0, v.0, "key: {k}");
        }
    }

    #[test]
    fn test_btree_set() {
        let a: BTreeSet<i32> = [1, 2, 5, 7, 0, 10].into_iter().collect();
        let b: BTreeSet<i32> = [2, 8, 9, 10, 5].into_iter().collect();

        let mut c: Vec<i32> = a.merge(b).unwrap().into_iter().collect();
        c.sort_unstable();
        assert_eq!(c, &[0, 1, 2, 5, 7, 8, 9, 10]);
    }
}
