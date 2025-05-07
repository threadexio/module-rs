use core::fmt;
use core::mem::discriminant;

use alloc::boxed::Box;

enum ErrorKind {
    Collision,
    Custom(Box<dyn fmt::Display + Send + Sync + 'static>),
}

impl fmt::Debug for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Collision => write!(f, "Collision"),
            Self::Custom(x) => write!(f, "Custom(\"{x}\")"),
        }
    }
}

impl PartialEq for ErrorKind {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}

impl Eq for ErrorKind {}

/// Erorr returned by [`Merge`].
///
/// [`Merge`]: crate::Merge
#[derive(Debug, PartialEq, Eq)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    /// Raised when [`Merge`] encounters 2 values which cannot be merged using
    /// the current strategy.
    ///
    /// [`Merge`]: crate::Merge
    pub fn collision() -> Self {
        Self {
            kind: ErrorKind::Collision,
        }
    }

    /// Raised when there is a general error when merging 2 values.
    pub fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display + Send + Sync + 'static,
    {
        Self {
            kind: ErrorKind::Custom(Box::new(msg)),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ErrorKind::Collision => f.write_str("value collision"),
            ErrorKind::Custom(ref x) => x.fmt(f),
        }
    }
}

impl core::error::Error for Error {}
