//! A no-op merge that retains the first value.
//!
//! See: [`First`].

use super::prelude::*;

merge_thin_wrapper! {
    /// A no-op merge that retains the first value.
    ///
    /// This type provides a merge implementation that _does nothing_. Merging
    /// anything with a value of this type results in the other value getting
    /// discarded.
    ///
    /// The opposite of this is [`Last`].
    ///
    /// # Example
    ///
    /// ```rust
    /// # use module::types::First;
    /// # use module::merge::Merge;
    /// let a = First(vec![0, 1, 2]);
    /// let b = First(vec![3, 4]);
    ///
    /// let merged = a.merge(b).unwrap();
    ///
    /// assert_eq!(*merged, &[0, 1, 2]);
    /// ```
    ///
    /// # serde
    ///
    /// This type deserializes like `T`.
    ///
    /// [`Last`]: crate::types::Last
    #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
    pub struct First;
}

impl<T> Merge for First<T> {
    #[inline]
    fn merge_ref(&mut self, _: Self) -> Result<(), Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_ab() {
        let a = First(42);
        let b = First(43);

        let merged = a.merge(b).unwrap();
        assert_eq!(*merged, 42);
    }

    #[test]
    fn test_merge_ba() {
        let a = First(42);
        let b = First(43);

        let merged = b.merge(a).unwrap();
        assert_eq!(*merged, 43);
    }
}

#[cfg(test)]
#[cfg(feature = "serde")]
mod serde_tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        let x: First<i32> = serde_json::from_str("42").unwrap();
        assert_eq!(*x, 42);
    }
}
