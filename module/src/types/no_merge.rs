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
    /// # Example
    ///
    /// ```rust
    /// # use module::types::NoMerge;
    /// # use module::merge::{Merge, ErrorKind};
    /// let a = NoMerge(vec![0, 1]);
    /// let b = NoMerge(vec![2]);
    ///
    /// let err = a.merge(b).unwrap_err();
    ///
    /// assert_eq!(err.kind, ErrorKind::Collision);
    /// ```
    ///
    /// # serde
    ///
    /// This type deserializes like `T`.
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

#[cfg(feature = "serde")]
mod serde_impl {
    use super::*;

    use serde::de::{Deserialize, Deserializer};

    impl<'de, T> Deserialize<'de> for NoMerge<T>
    where
        T: Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            <T as Deserialize>::deserialize(deserializer).map(Into::into)
        }
    }
}
