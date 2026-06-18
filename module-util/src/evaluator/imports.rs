//! Module import list.

use core::iter::FusedIterator;
use core::slice;

use alloc::vec::{self, Vec};

///////////////////////////////////////////////////////////////////////////////

/// A list of imports.
///
/// # serde
///
/// This type deserializes as a "sequence".
///
/// See: [serde data model](https://serde.rs/data-model.html).
#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct Imports<T>(Vec<T>);

impl<T> From<Vec<T>> for Imports<T> {
    fn from(x: Vec<T>) -> Self {
        Self(x)
    }
}

impl<T> From<Imports<T>> for Vec<T> {
    fn from(x: Imports<T>) -> Self {
        x.0
    }
}

impl<T> FromIterator<T> for Imports<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl<T> Imports<T> {
    /// Create a new empty import list.
    pub const fn empty() -> Self {
        Self(Vec::new())
    }

    /// Get the number of imports in the list.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check whether the import list is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Add `import` to the import list.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use module_util::evaluator::imports::Imports;
    /// let mut imports = Imports::empty();
    /// imports.push("module 1");
    /// imports.push("module 2");
    /// imports.push("module 3");
    ///
    /// assert_eq!(
    ///     imports.iter().copied().collect::<Vec<_>>(),
    ///     &[
    ///         "module 1",
    ///         "module 2",
    ///         "module 3"
    ///     ]
    /// );
    /// ```
    pub fn push(&mut self, import: T) {
        self.0.push(import);
    }

    /// Get an iterator over the import list.
    ///
    /// The iterator visits each import in an _unspecified_ order. Do not rely
    /// on the ordering of items.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use module_util::evaluator::imports::Imports;
    /// let mut imports = Imports::empty();
    /// imports.push("module 1");
    /// imports.push("module 2");
    /// imports.push("module 3");
    ///
    /// let mut iter = imports.iter();
    /// assert_eq!(iter.next().copied(), Some("module 1"));
    /// assert_eq!(iter.next().copied(), Some("module 2"));
    /// assert_eq!(iter.next().copied(), Some("module 3"));
    /// assert_eq!(iter.next().copied(), None);
    /// ```
    pub fn iter(&self) -> Iter<'_, T> {
        Iter(self.0.iter())
    }
}

impl<T> IntoIterator for Imports<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.0.into_iter())
    }
}

///////////////////////////////////////////////////////////////////////////////

/// Borrowing iterator over [`Imports`].
///
/// See: [`Imports::iter`].
#[derive(Debug, Clone)]
pub struct Iter<'a, T>(slice::Iter<'a, T>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a, T> FusedIterator for Iter<'a, T> {}

///////////////////////////////////////////////////////////////////////////////

/// Owning iterator over [`Imports`].
#[derive(Debug, Clone)]
pub struct IntoIter<T>(vec::IntoIter<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<T> FusedIterator for IntoIter<T> {}
