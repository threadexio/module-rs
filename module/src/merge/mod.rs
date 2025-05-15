//! The [`Merge`] trait and utilities.

pub mod error;
pub use self::error::{Context, Error};

mod impls;

#[cfg(all(test, feature = "derive"))]
mod tests;

/// A mergeable value.
///
/// This trait defines the interface by which 2 values are merged together.
pub trait Merge: Sized {
    /// Merge `self` with `other`.
    fn merge(mut self, other: Self) -> Result<Self, Error> {
        self.merge_ref(other)?;
        Ok(self)
    }

    /// Merge `&mut self` with `other`.
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
