use core::cmp::Ord;
use core::fmt::Display;

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
    fn merge_ref(&mut self, other: Self) -> Result<(), Error> {
        match self {
            Self::Owned(x) => x.merge_ref(other.into_owned()),
            Self::Borrowed(x) => {
                let mut x = x.to_owned();
                x.merge_ref(other.into_owned())?;
                *self = Self::Owned(x);
                Ok(())
            }
        }
    }
}

impl<T> Merge for Box<T>
where
    T: Merge,
{
    fn merge_ref(&mut self, other: Self) -> Result<(), Error> {
        T::merge_ref(self, *other)
    }
}

impl<T> Merge for Vec<T> {
    fn merge_ref(&mut self, mut other: Self) -> Result<(), Error> {
        self.append(&mut other);
        Ok(())
    }
}

impl<T> Merge for LinkedList<T> {
    fn merge_ref(&mut self, mut other: Self) -> Result<(), Error> {
        self.append(&mut other);
        Ok(())
    }
}

impl<K, V> Merge for BTreeMap<K, V>
where
    K: Ord + Display,
    V: Merge,
{
    fn merge_ref(&mut self, other: Self) -> Result<(), Error> {
        use alloc::collections::btree_map::Entry;

        for (k, b) in other {
            match self.entry(k) {
                Entry::Vacant(x) => {
                    x.insert(b);
                }
                Entry::Occupied(x) => {
                    let (k, a) = x.remove_entry();
                    let merged = a.merge(b).with_value(|| format!("\"{k}\""))?;
                    self.insert(k, merged);
                }
            }
        }

        Ok(())
    }
}

impl<T> Merge for BTreeSet<T>
where
    T: Ord,
{
    fn merge_ref(&mut self, mut other: Self) -> Result<(), Error> {
        self.append(&mut other);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;

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
        fn from_keys(keys: &[&'static str]) -> BTreeMap<&'static str, Merged> {
            keys.iter()
                .copied()
                .map(|k| (k, Merged::default()))
                .collect()
        }

        let a = from_keys(&["key1", "key2", "key3", "key4", "key7"]);
        let b = from_keys(&["key5", "key1", "key7", "key2", "key6"]);

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

        assert_eq!(expected.len(), c.len());

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
