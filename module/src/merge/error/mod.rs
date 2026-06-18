//! [`Merge`] error.

use core::fmt;

use alloc::boxed::Box;

pub mod field;
pub use self::field::Field;

///////////////////////////////////////////////////////////////////////////////

/// Error returned by [`Merge`].
///
/// # Example
///
/// ```rust
/// # use module::merge::error::Error;
/// use std::io;
///
/// let mut err = Error::collision();
/// assert_eq!(err.to_string(), "value collision");
///
/// err.field.push_back("user");
/// err.field.push_back("name");
/// err.field.push_back("first");
/// assert_eq!(err.to_string(), "\"user.name.first\": value collision");
///
/// let mut err = Error::other(io::Error::other("invalid name"));
/// assert_eq!(err.to_string(), "invalid name");
///
/// err.field.push_back("user");
/// err.field.push_back("name");
/// err.field.push_back("first");
/// assert_eq!(err.to_string(), "\"user.name.first\": invalid name");
/// ```
pub struct Error {
    inner: Repr,

    /// Path of the field which caused the error.
    pub field: Field,
}

enum Repr {
    Collision,
    Other(Box<dyn core::error::Error + 'static>),
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Error")
            .field(
                "kind",
                &fmt::from_fn(|f| match self.inner {
                    Repr::Collision => write!(f, "Collision"),
                    Repr::Other(ref x) => f.debug_tuple("Other").field(&x).finish(),
                }),
            )
            .field("field", &self.field)
            .finish()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.field.is_empty() {
            write!(f, "{:?}: ", self.field)?;
        }

        self.message(f)?;
        Ok(())
    }
}

impl core::error::Error for Error {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self.inner {
            Repr::Collision => None,
            Repr::Other(ref x) => Some(&**x),
        }
    }
}

impl Error {
    /// Create a new collision error.
    #[must_use]
    pub const fn collision() -> Self {
        Self::with_inner(Repr::Collision)
    }

    /// Create a new error from `error`.
    #[must_use]
    pub fn other<E>(error: E) -> Self
    where
        E: core::error::Error + 'static,
    {
        Self::with_inner(Repr::Other(Box::new(error)))
    }

    const fn with_inner(kind: Repr) -> Self {
        Self {
            inner: kind,
            field: Field::empty(),
        }
    }

    /// Format the message of the error.
    ///
    /// This does not include the name of the field that caused the error.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use module::merge::error::Error;
    /// use std::io;
    /// use std::fmt;
    ///
    /// struct FmtMessage(Error);
    ///
    /// impl fmt::Display for FmtMessage {
    ///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         self.0.message(f)
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     FmtMessage(Error::collision()).to_string(),
    ///     "value collision"
    /// );
    ///
    /// assert_eq!(
    ///     FmtMessage(Error::other(io::Error::other("my custom error"))).to_string(),
    ///     "my custom error"
    /// );
    /// ```
    pub fn message(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.inner {
            Repr::Collision => write!(f, "value collision"),
            Repr::Other(ref x) => write!(f, "{x}"),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

/// Kind of [`Error`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorKind {
    /// A collision error.
    Collision,

    /// An error of some other kind during the [`Merge`].
    Other,
}

impl Error {
    /// Get the kind of the error.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use module::merge::error::{Error, ErrorKind};
    /// use std::io;
    ///
    /// let err = Error::collision();
    /// assert_eq!(err.kind(), ErrorKind::Collision);
    ///
    /// let err = Error::other(io::Error::other("some other error"));
    /// assert_eq!(err.kind(), ErrorKind::Other);
    /// ```
    #[must_use]
    pub fn kind(&self) -> ErrorKind {
        match self.inner {
            Repr::Collision => ErrorKind::Collision,
            Repr::Other(_) => ErrorKind::Other,
        }
    }
}
