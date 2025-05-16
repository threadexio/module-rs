//! The [`Merge`] trait and utilities accompanying it.

mod context;
mod error;
mod impls;
mod trace;

#[cfg(test)]
mod tests;

pub use self::context::Context;
pub use self::error::{Error, ErrorKind};
pub use self::trace::{Trace, TraceIter};

/// A value that may be merged.
///
/// This trait defines the interface by which 2 values are merged together.
///
/// There are 2 ways to merge values together only differing in whether they
/// take ownership of the value.
///
/// * [`Merge::merge`]
/// * [`Merge::merge_ref`]
///
/// Both of these methods must perform merging in the same way. Any deviation
/// in the 2 implementations is considered undefined behavior and *will* lead
/// to bugs.
///
/// [`Merge::merge`] is provided by default as long as you provide the
/// implementation for [`Merge::merge_ref`]. Generally there should be no reason
/// to have to implement both of the above.
pub trait Merge: Sized {
    /// Merge `self` with `other`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use module::Merge;
    /// let a = vec![1, 3, 4];
    /// let b = vec![7, 2, 0];
    ///
    /// let c = a.merge(b).unwrap();
    ///
    /// assert_eq!(c, &[1, 3, 4, 7, 2, 0]);
    /// ```
    ///
    /// # Implementation
    ///
    /// The default implementation of this method calls out to [`Merge::merge_ref`].
    /// Don't implemenent this unless you can do something better.
    fn merge(mut self, other: Self) -> Result<Self, Error> {
        self.merge_ref(other)?;
        Ok(self)
    }

    /// Merge `self` with `other` without taking ownership of `self`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use module::Merge;
    /// let mut a = vec![1, 3, 4];
    /// let b = vec![7, 2, 0];
    ///
    /// a.merge_ref(b).unwrap();
    ///
    /// assert_eq!(a, &[1, 3, 4, 7, 2, 0]);
    /// ```
    fn merge_ref(&mut self, other: Self) -> Result<(), Error>;
}

/// Merge `this` and `other`.
///
/// Equivalent to: `this.merge(other)`.
#[inline]
pub fn merge<T>(this: T, other: T) -> Result<T, Error>
where
    T: Merge,
{
    this.merge(other)
}
