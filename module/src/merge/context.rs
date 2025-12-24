use core::fmt::Display;

use super::Error;
use super::error::Trace;

mod private {
    pub trait Sealed {}
}
use self::private::Sealed;

/// Extension trait for [`Result`].
///
/// Adds methods for adding context to an [`Error`].
///
/// This trait is **sealed** and cannot be implemented for any other types.
///
/// [`Error`]: super::Error
pub trait Context: Sealed {
    /// Add the name of this module to the context of the error.
    ///
    /// This method adds context to the [`Error`] so that it knows in which
    /// module the error occurred. It is perfectly fine to never use it, as long
    /// as you don't mind cryptic errors.
    ///
    /// Modules should be added in the following order, from the module where
    /// the error occured and then its parent module up until the root module.
    fn module<D>(self, name: D) -> Self
    where
        D: Display,
        Self: Sized;

    /// The same as [`Context::module`] but lazily-evaluated.
    fn with_module<D>(self, f: impl FnOnce() -> D) -> Self
    where
        D: Display;

    /// Set the trace of the error.
    ///
    /// This method overwrites the module trace completely with `trace`.
    fn trace(self, trace: Trace) -> Self;

    /// Set the trace of the error.
    ///
    /// The same as [`Context::trace`] but lazily-evaluated.
    fn with_trace(self, f: impl FnOnce() -> Trace) -> Self;

    /// Add the name of the value to the context of the error.
    ///
    /// This method adds context to the [`Error`] so that it knows in which
    /// value the error occurred. It is perfectly fine to never use it, as long
    /// as you don't mind cryptic errors.
    fn value<D>(self, name: D) -> Self
    where
        D: Display,
        Self: Sized;

    /// The same as [`Context::value`] but lazily-evaluated.
    fn with_value<D>(self, f: impl FnOnce() -> D) -> Self
    where
        D: Display,
        Self: Sized;
}

impl<T> Sealed for core::result::Result<T, Error> {}

impl<T> Context for core::result::Result<T, Error> {
    fn module<D>(self, name: D) -> Self
    where
        D: Display,
        Self: Sized,
    {
        self.with_module(|| name)
    }

    fn with_module<D>(self, f: impl FnOnce() -> D) -> Self
    where
        D: Display,
    {
        self.map_err(|mut e| {
            e.trace.push_front(f());
            e
        })
    }

    fn trace(self, trace: Trace) -> Self {
        self.with_trace(|| trace)
    }

    fn with_trace(self, f: impl FnOnce() -> Trace) -> Self {
        self.map_err(|mut e| {
            e.trace = f();
            e
        })
    }

    fn value<D>(self, name: D) -> Self
    where
        D: Display,
        Self: Sized,
    {
        self.with_value(|| name)
    }

    fn with_value<D>(self, f: impl FnOnce() -> D) -> Self
    where
        D: Display,
        Self: Sized,
    {
        self.map_err(|mut e| {
            e.value.push_front(f());
            e
        })
    }
}
