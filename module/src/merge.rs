use crate::error::Error;

/// A mergeable value.
///
/// This trait defines the interface by which 2 values are merged together.
pub trait Merge: Sized {
    /// Merge `self` with `other`.
    fn merge(self, other: Self) -> Result<Self, Error>;
}
