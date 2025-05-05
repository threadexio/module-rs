use core::borrow::{Borrow, BorrowMut};
use core::cmp::Ordering;
use core::convert::{AsMut, AsRef};
use core::ops::{Deref, DerefMut};

use crate::error::Error;
use crate::merge::Merge;

/// The priority of an [`Overridable`] value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl<T> Merge for Overridable<T> {
    fn merge(self, other: Self) -> Result<Self, Error> {
        match self.priority.cmp(&other.priority) {
            Ordering::Less => Ok(self),
            Ordering::Greater => Ok(self),
            Ordering::Equal => Err(Error::collision()),
        }
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
        Raw(T),
    }

    impl<T, const DEFAULT: isize> From<Repr<T>> for Overridable<T, DEFAULT> {
        fn from(x: Repr<T>) -> Self {
            match x {
                Repr::Priority { value, priority } => Overridable::with_priority(value, priority),
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
