//! The [`Merge`] trait and utilities.

use crate::error::Error;

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

#[cfg(all(test, feature = "derive"))]
mod tests {
    use super::*;

    use crate::Merge;

    #[derive(Default)]
    struct Merged(bool);

    impl Merge for Merged {
        fn merge_ref(&mut self, _: Self) -> Result<(), Error> {
            self.0 = true;
            Ok(())
        }
    }

    #[test]
    fn test_derive_merge_unit() {
        #[derive(Merge)]
        struct Unit;

        let a = Unit;
        let b = Unit;

        let _: Unit = a.merge(b).unwrap();
    }

    #[test]
    fn test_derive_merge_tuple() {
        #[derive(Default, Merge)]
        struct MyType(Merged, Merged);

        let a = MyType::default();
        let b = MyType::default();

        let c = a.merge(b).unwrap();

        assert!(c.0.0);
        assert!(c.1.0);
    }

    #[test]
    fn test_derive_merge_named() {
        #[derive(Default, Merge)]
        struct MyType {
            a: Merged,
            b: Merged,
            c: Merged,
        }

        let a = MyType::default();
        let b = MyType::default();

        let c = a.merge(b).unwrap();

        assert!(c.a.0);
        assert!(c.b.0);
        assert!(c.c.0);
    }
}
