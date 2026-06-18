use super::Error;

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
    /// Add the name of the value to the context of the error.
    ///
    /// This method adds context to the [`Error`] so that it knows in which
    /// value the error occurred. It is perfectly fine to never use it, as long
    /// as you don't mind cryptic errors.
    #[must_use]
    fn field(self, component: &str) -> Self
    where
        Self: Sized,
    {
        self.with_field(|| component)
    }

    /// The same as [`Context::field`] but lazily-evaluated.
    #[must_use]
    fn with_field<'a>(self, f: impl FnOnce() -> &'a str) -> Self
    where
        Self: Sized;
}

impl<T> Sealed for Result<T, Error> {}

impl<T> Context for Result<T, Error> {
    fn with_field<'a>(self, f: impl FnOnce() -> &'a str) -> Self
    where
        Self: Sized,
    {
        self.map_err(|mut e| {
            e.field.push_front(f());
            e
        })
    }
}
