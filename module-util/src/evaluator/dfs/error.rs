//! [`Evaluator`] error.
//!
//! [`Evaluator`]: super::Evaluator

use core::fmt;

use alloc::boxed::Box;

///////////////////////////////////////////////////////////////////////////////

/// Error returned by [`Dfs`].
///
/// [`Dfs`]: super::Dfs
#[derive(Debug)]
pub enum Error {
    /// An error during the [`Merge`] operation.
    ///
    /// [`Merge`]: module::merge::Merge
    Merge(module::merge::Error),

    /// An error during some other operation during evaluation.
    Other(Box<dyn core::error::Error + 'static>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Merge(x) => fmt::Display::fmt(x, f),
            Self::Other(x) => fmt::Display::fmt(&**x, f),
        }
    }
}

impl core::error::Error for Error {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::Merge(x) => Some(x),
            Self::Other(x) => Some(&**x),
        }
    }
}

impl Error {
    /// Create new error from `error`.
    #[must_use]
    pub fn other<E>(error: E) -> Self
    where
        E: core::error::Error + 'static,
    {
        Self::Other(Box::new(error))
    }

    /// Check whether the error is a merge error.
    ///
    /// Equivalent to: `matches!(self, Self::Merge(_))`.
    #[inline]
    #[must_use]
    pub fn is_merge(&self) -> bool {
        matches!(self, Self::Merge(_))
    }

    /// Get a reference to the inner merge error.
    #[must_use]
    pub fn as_merge(&self) -> Option<&module::merge::Error> {
        match self {
            Self::Merge(x) => Some(x),
            _ => None,
        }
    }

    /// Get a mutable reference to the inner merge error.
    #[must_use]
    pub fn as_merge_mut(&mut self) -> Option<&mut module::merge::Error> {
        match self {
            Self::Merge(x) => Some(x),
            _ => None,
        }
    }

    /// Check whether the error is of another kind.
    ///
    /// Equivalent to: `matches!(self, Self::Other(_))`.
    #[inline]
    #[must_use]
    pub fn is_other(&self) -> bool {
        matches!(self, Self::Other(_))
    }

    /// Get a reference to the inner error.
    #[must_use]
    pub fn as_other(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::Other(x) => Some(&**x),
            _ => None,
        }
    }

    /// Get a mutable reference to the inner error.
    #[must_use]
    pub fn as_other_mut(&mut self) -> Option<&mut (dyn core::error::Error + 'static)> {
        match self {
            Self::Other(x) => Some(&mut **x),
            _ => None,
        }
    }
}
