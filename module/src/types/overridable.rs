//! Overridable values.
//!
//! See: [`Overridable`].

use core::borrow::{Borrow, BorrowMut};
use core::cmp::Ordering;
use core::convert::{AsMut, AsRef};
use core::fmt;
use core::ops::{Deref, DerefMut};

use super::prelude::*;

/// The priority of an [`Overridable`] value.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Priority(isize);

impl From<isize> for Priority {
    fn from(x: isize) -> Self {
        Self(x)
    }
}

impl From<Priority> for isize {
    fn from(x: Priority) -> Self {
        x.0
    }
}

impl fmt::Debug for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// An overridable value based on priority.
///
/// This is a wrapper that provides the "magic" mechanism by which values can
/// be overridden across modules. After evaluating all modules, an `Override<T>`
/// will keep the value with the least priority.
///
/// If the priority of a value is not given, the default of 500 is assumed.
/// This can be changed by the `DEFAULT` type parameter. If you need to change
/// this default, it is strongly recommended you make a type alias to avoid
/// specifying the default priority on each use.
///
/// # Example
///
/// ```rust
/// # use module::types::Overridable;
/// # use module::merge::Merge;
/// let a = Overridable::with_priority("foo", 10);
/// let b = Overridable::with_priority("bar", 5);
///
/// let merged: Overridable<&str> = a.merge(b).unwrap();
///
/// assert_eq!(*merged, "bar");
/// ```
///
/// # serde
///
/// This type deserializes as one of the following:
///
/// * `T`
/// * `{ value: T }`
/// * `{ value: T, priority: isize }`
#[derive(Debug, Clone, Copy)]
pub struct Overridable<T, const DEFAULT: isize = 500> {
    value: T,
    priority: Priority,
}

impl<T, const DEFAULT: isize> Overridable<T, DEFAULT> {
    /// Create a new `value` with the default priority.
    pub fn new(value: T) -> Self {
        Self::with_priority(value, Priority(DEFAULT))
    }

    /// Create a new `value` with `priority`.
    pub fn with_priority<P>(value: T, priority: P) -> Self
    where
        P: Into<Priority>,
    {
        let priority = priority.into();
        Self { value, priority }
    }

    /// Get the priority of this value.
    pub fn priority(&self) -> Priority {
        self.priority
    }

    /// Set the priority of this value.
    pub fn set_priority<P>(&mut self, priority: P)
    where
        P: Into<Priority>,
    {
        self.priority = priority.into();
    }

    /// Destruct this [`Overridable`] and get the inner value.
    pub fn into_value(self) -> T {
        self.value
    }
}

impl<T, const DEFAULT: isize> Merge for Overridable<T, DEFAULT> {
    fn merge_ref(&mut self, other: Self) -> Result<(), Error> {
        match self.priority.cmp(&other.priority) {
            Ordering::Less => Ok(()),
            Ordering::Greater => {
                *self = other;
                Ok(())
            }
            Ordering::Equal => Err(Error::collision()),
        }
    }
}

impl<T, const DEFAULT: isize> From<T> for Overridable<T, DEFAULT> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T, const DEFAULT: isize> Default for Overridable<T, DEFAULT>
where
    T: Default,
{
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T, const DEFAULT: isize> Borrow<T> for Overridable<T, DEFAULT> {
    #[inline]
    fn borrow(&self) -> &T {
        &self.value
    }
}

impl<T, const DEFAULT: isize> BorrowMut<T> for Overridable<T, DEFAULT> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T, const DEFAULT: isize> AsRef<T> for Overridable<T, DEFAULT> {
    #[inline]
    fn as_ref(&self) -> &T {
        &self.value
    }
}

impl<T, const DEFAULT: isize> AsMut<T> for Overridable<T, DEFAULT> {
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T, const DEFAULT: isize> Deref for Overridable<T, DEFAULT> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T, const DEFAULT: isize> DerefMut for Overridable<T, DEFAULT> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use super::*;

    use serde::Deserialize;
    use serde::de::Deserializer;

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Repr<T> {
        Priority { value: T, priority: isize },
        Value { value: T },
        Raw(T),
    }

    impl<T, const DEFAULT: isize> From<Repr<T>> for Overridable<T, DEFAULT> {
        fn from(x: Repr<T>) -> Self {
            match x {
                Repr::Priority { value, priority } => Overridable::with_priority(value, priority),
                Repr::Value { value } => Overridable::new(value),
                Repr::Raw(value) => Overridable::new(value),
            }
        }
    }

    impl<'de, T, const DEFAULT: isize> Deserialize<'de> for Overridable<T, DEFAULT>
    where
        T: Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            <Repr<T> as Deserialize>::deserialize(deserializer).map(Into::into)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;

    #[inline]
    fn x<T>(value: T, priority: isize) -> Overridable<T> {
        Overridable::with_priority(value, priority)
    }

    #[test]
    fn test_commutative_ab() {
        let a = x(42, 10);
        let b = x(32, 9);

        let c = a.merge(b).unwrap();
        assert_eq!(*c, 32);
    }

    #[test]
    fn test_commutative_ba() {
        let a = x(42, 10);
        let b = x(32, 9);

        let c = b.merge(a).unwrap();
        assert_eq!(*c, 32);
    }

    #[test]
    fn test_same_priority() {
        assert_eq!(
            x(1, 10).merge(x(2, 10)).unwrap_err().kind,
            ErrorKind::Collision
        );
    }

    #[test]
    fn test_no_inner_merge() {
        let a = x(Merged(false), 10);
        let b = x(Merged(false), 9);

        let c = a.merge(b).unwrap();
        assert!(!c.0);
    }
}

#[cfg(test)]
#[cfg(feature = "serde")]
mod serde_tests {
    use super::*;

    #[test]
    fn test_deserialize_t() {
        let x: Overridable<i32> = serde_json::from_str("42").unwrap();
        assert_eq!(x.priority.0, 500);
        assert_eq!(*x, 42);
    }

    #[test]
    fn test_deserialize_value() {
        let x: Overridable<i32> = serde_json::from_str("{ \"value\": 42 }").unwrap();
        assert_eq!(x.priority.0, 500);
        assert_eq!(*x, 42);
    }

    #[test]
    fn test_deserialize_value_priority() {
        let x: Overridable<i32> =
            serde_json::from_str("{ \"value\": 42, \"priority\": -500 }").unwrap();
        assert_eq!(x.priority.0, -500);
        assert_eq!(*x, 42);
    }
}
