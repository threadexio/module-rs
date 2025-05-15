use core::cmp::Eq;
use core::fmt::Display;
use core::hash::{BuildHasher, Hash};

use alloc::boxed::Box;

use std::collections::{HashMap, HashSet};

use super::prelude::*;

unmergeable! {
    Box<std::ffi::OsStr>, Box<std::path::Path>,
    std::ffi::OsString, std::path::PathBuf,
    std::time::SystemTime
}

impl<K, V, S> Merge for HashMap<K, V, S>
where
    K: Eq + Hash + Display,
    V: Merge,
    S: BuildHasher,
{
    fn merge_ref(&mut self, other: Self) -> Result<(), Error> {
        use std::collections::hash_map::Entry;

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

impl<T, S> Merge for HashSet<T, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
    fn merge_ref(&mut self, other: Self) -> Result<(), Error> {
        self.extend(other);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;

    use alloc::vec::Vec;

    #[test]
    fn test_hash_map() {
        fn from_keys(keys: &[&'static str]) -> HashMap<&'static str, Merged> {
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
    fn test_hash_set() {
        let a: HashSet<i32> = [1, 2, 5, 7, 0, 10].into_iter().collect();
        let b: HashSet<i32> = [2, 8, 9, 10, 5].into_iter().collect();

        let mut c: Vec<i32> = a.merge(b).unwrap().into_iter().collect();
        c.sort_unstable();
        assert_eq!(c, &[0, 1, 2, 5, 7, 8, 9, 10]);
    }
}
