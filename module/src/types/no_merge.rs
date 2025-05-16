//! Explicitly disallow merging of values.
//!
//! See: [`NoMerge`].

use super::prelude::*;

merge_thin_wrapper! {
    /// An unmergeable value.
    ///
    /// This wrapper can wrap any type and make it "unmergeable". This means
    /// that any attempt to merge it will result in a collision error.
    ///
    /// # serde
    ///
    /// This type deserializes just like a normal `T`.
    pub struct NoMerge;
}

impl<T> Merge for NoMerge<T> {
    fn merge(self, _other: Self) -> Result<Self, Error> {
        Err(Error::collision())
    }

    fn merge_ref(&mut self, _other: Self) -> Result<(), Error> {
        Err(Error::collision())
    }
}
