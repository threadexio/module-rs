use super::cell::MergeCell;
use super::{Error, Merge};

/// Extension trait for [`Iterator`].
///
/// This trait is automatically implemented for every [`Iterator`].
pub trait IteratorExt: Iterator {
    /// Takes an iterator and merges together its items.
    ///
    /// # Panics
    ///
    /// If the iterator is empty. For the non-panicking version of this
    /// method, see: [`try_merge()`].
    ///
    /// # Example
    ///
    /// ```rust
    /// # use module::merge::{Merge, IteratorExt};
    /// let iter = [
    ///     vec![0, 6],
    ///     vec![2, 3, 7],
    ///     vec![],
    ///     vec![1, 5],
    ///     vec![4]
    /// ].into_iter();
    ///
    /// let merged = iter.merge().unwrap();
    ///
    /// assert_eq!(merged, &[0, 6, 2, 3, 7, 1, 5, 4]);
    /// ```
    ///
    /// [`try_merge()`]: Self::try_merge
    fn merge(self) -> Result<Self::Item, Error>
    where
        Self::Item: Merge,
        Self: Sized;

    /// Takes an iterator and merges together its items.
    ///
    /// This is the non-panicking version of [`merge()`].
    ///
    /// Returns [`None`] if the iterator yields no items.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use module::merge::{Merge, IteratorExt};
    /// let iter = std::iter::empty::<()>();
    ///
    /// let r = iter.try_merge();
    ///
    /// assert!(r.is_none());
    /// ```
    ///
    /// [`merge()`]: Self::merge
    fn try_merge(self) -> Option<Result<Self::Item, Error>>
    where
        Self::Item: Merge,
        Self: Sized;
}

impl<I> IteratorExt for I
where
    I: Iterator,
{
    fn merge(self) -> Result<Self::Item, Error>
    where
        Self::Item: Merge,
        Self: Sized,
    {
        self.try_merge()
            .expect("tried to .merge() an empty iterator")
    }

    fn try_merge(self) -> Option<Result<Self::Item, Error>>
    where
        Self::Item: Merge,
        Self: Sized,
    {
        let mut cell = MergeCell::empty();

        for item in self {
            cell.merge(item);
        }

        cell.try_finish()
    }
}
