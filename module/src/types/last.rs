//! A no-op merge that retains the last value.
//!
//! See: [`Last`].

use super::prelude::*;

merge_thin_wrapper! {
    /// A no-op merge that retains the last value.
    ///
    /// This type provides a merge implementation that always retains the other
    /// value.
    ///
    /// The opposite of this is [`First`].
    ///
    /// # Example
    ///
    /// ```rust
    /// # use module::types::Last;
    /// # use module::merge::Merge;
    /// let a = Last(vec![0, 1, 2]);
    /// let b = Last(vec![3, 4]);
    ///
    /// let merged = a.merge(b).unwrap();
    ///
    /// assert_eq!(*merged, &[3, 4]);
    /// ```
    ///
    /// # serde
    ///
    /// This type deserializes like `T`.
    ///
    /// [`First`]: crate::types::First
    #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
    pub struct Last;
}

impl<T> Merge for Last<T> {
    #[inline]
    fn merge_ref(&mut self, other: Self) -> Result<(), Error> {
        self.0 = other.0;
        Ok(())
    }
}
