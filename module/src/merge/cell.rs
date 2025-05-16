use core::mem::replace;

use super::Merge;
use super::error::Error;

/// A memory location that allows repeated merging.
///
/// This type acts like an "accumulator" for merging. It allows merging many
/// values and defering merge errors for later.
///
/// [`MergeCell`] deliberately does not implement [`Merge`]. It is not intended
/// to be merged with other values, rather it expects to have values merged
/// into it.
///
/// # Example
///
/// ```rust
/// # use module::merge::{Merge, MergeCell};
/// let mut cell = MergeCell::empty();
///
/// cell.merge(vec![1, 2]);
/// cell.merge(vec![]);
/// cell.merge(vec![0, 4, 8]);
///
/// let merged = cell.try_finish().unwrap().unwrap();
/// assert_eq!(merged, &[1, 2, 0, 4, 8]);
/// ```
#[derive(Debug)]
pub struct MergeCell<T> {
    value: Option<T>,
    result: Result<(), Error>,
}

impl<T> Default for MergeCell<T> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<T> MergeCell<T> {
    /// Create a new empty [`MergeCell`].
    pub fn empty() -> Self {
        Self {
            value: None,
            result: Ok(()),
        }
    }

    /// Create a new [`MergeCell`] that is initialized with `value`.
    pub fn new(value: T) -> Self {
        Self {
            value: Some(value),
            result: Ok(()),
        }
    }

    /// Check whether the cell is empty.
    ///
    /// The cell is empty if and only if it was created with [`empty()`] and no
    /// values have been [`merge()`]d.
    ///
    /// [`empty()`]: MergeCell::empty
    /// [`merge()`]: MergeCell::merge
    pub fn is_empty(&self) -> bool {
        self.value.is_none()
    }

    /// Check whether a previous [`merge()`] operation has failed.
    ///
    /// [`merge()`]: MergeCell::merge
    pub fn has_errored(&self) -> bool {
        self.result.is_err()
    }

    /// Destruct the [`MergeCell`] and get back the final merged value.
    ///
    /// Returns the result of all of the [`merge()`] operations on the cell.
    ///
    /// # Panics
    ///
    /// If the [`MergeCell`] is empty. For the non-panicking version of this
    /// method, see: [`try_finish()`].
    ///
    /// # Example
    ///
    /// ```rust
    /// # use module::merge::{Merge, MergeCell};
    /// let mut cell = MergeCell::empty();
    ///
    /// cell.merge(vec![1, 2]);
    /// cell.merge(vec![]);
    /// cell.merge(vec![0, 4, 8]);
    ///
    /// let merged = cell.finish().unwrap();
    /// assert_eq!(merged, &[1, 2, 0, 4, 8]);
    /// ```
    ///
    /// [`merge()`]: MergeCell::merge
    /// [`try_finish()`]: MergeCell::try_finish
    pub fn finish(self) -> Result<T, Error> {
        self.try_finish()
            .expect("tried to .finish() an empty MergeCell")
    }

    /// Destruct the [`MergeCell`] and get back the final merged value.
    ///
    /// This is the non-panicking version of [`finish()`].
    ///
    /// Returns [`None`] if the cell is empty. Otherwise, it is functionally the
    /// same as [`finish()`].
    ///
    /// # Example
    ///
    /// ```rust
    /// # use module::merge::{Merge, MergeCell};
    /// let mut cell = MergeCell::empty();
    ///
    /// cell.merge(vec![1, 2]);
    /// cell.merge(vec![]);
    /// cell.merge(vec![0, 4, 8]);
    ///
    /// let merged = cell.try_finish().unwrap().unwrap();
    /// assert_eq!(merged, &[1, 2, 0, 4, 8]);
    /// ```
    ///
    /// [`finish()`]: MergeCell::finish
    pub fn try_finish(self) -> Option<Result<T, Error>> {
        let value = self.value?;
        let r = self.result.map(|()| value);
        Some(r)
    }
}

impl<T> MergeCell<T>
where
    T: Merge,
{
    /// Merge `other` into the cell.
    ///
    /// This function will fill the cell if it is empty.
    pub fn merge(&mut self, other: T) {
        match self.value {
            Some(ref mut value) => {
                let r = replace(&mut self.result, Ok(()));
                self.result = r.and_then(|()| value.merge_ref(other));
            }

            None => self.value = Some(other),
        }
    }
}
