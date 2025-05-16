//! [`Error`] & [`Context`].
//!
//! This module contains all the machinery used to present nice and useful error
//! messages from merge operations.

use core::fmt::{self, Debug, Display};
use core::mem::discriminant;

use alloc::boxed::Box;

use super::trace::Trace;

/// Kind of [`Error`].
#[non_exhaustive]
pub enum ErrorKind {
    /// Values cannot be merged.
    ///
    /// This error should be returned by [`Merge`] implementations when it is
    /// not clear how to merge the values. For example, the 2 values may have
    /// the same priority.
    ///
    /// For many types, the term "merge" does not make sense. How should one
    /// merge 2 [`i32`]s for instance? Types which do not have an obvious merge
    /// strategy or types on which the notion of "merging" cannot be defined
    /// clearly are called "unmergeable". Such types should have a [`Merge`]
    /// implementation but it should unconditionally return this error.
    ///
    /// [`Merge`]: crate::merge::Merge
    Collision,

    /// Cyclic module imports.
    ///
    /// This error should not need to be raised by [`Merge`] implementations. It
    /// is supposed to be raised by evaluators when they encounter cyclic module
    /// imports.
    ///
    /// [`Merge`]: crate::merge::Merge
    Cycle,

    /// A custom error that occurred during merging or evaluating.
    ///
    /// Contains a [`Box`]ed error object.
    Custom(Box<dyn Display + Send + Sync + 'static>),
}

impl ErrorKind {
    /// Check whether `self` is [`ErrorKind::Collision`].
    pub fn is_collision(&self) -> bool {
        matches!(self, Self::Collision)
    }

    /// Check whether `self` is [`ErrorKind::Cycle`].
    pub fn is_cycle(&self) -> bool {
        matches!(self, Self::Cycle)
    }

    /// Check whether `self` is [`ErrorKind::Custom`].
    pub fn is_custom(&self) -> bool {
        matches!(self, Self::Custom(_))
    }
}

impl Debug for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Collision => write!(f, "Collision"),
            Self::Cycle => write!(f, "Cycle"),
            Self::Custom(x) => write!(f, "Custom(\"{x}\")"),
        }
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Collision => write!(f, "value collision"),
            Self::Cycle => write!(f, "cyclic imports"),
            Self::Custom(x) => x.fmt(f),
        }
    }
}

impl PartialEq for ErrorKind {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}

impl Eq for ErrorKind {}

/// Error returned by [`Merge`].
///
/// # Display
///
/// The default [`Display`] implementation may not fit into the style of
/// your app.
///
/// ```rust
/// # use module::merge::{Merge, Error, Context};
/// # let a = 42i32;
/// # let b = 43i32;
/// let r = a.merge(b)
///     .value("count")
///     .value("settings")
///     .module("user.json")
///     .module("config.json");
///
/// let err = r.unwrap_err();
///
/// assert_eq!(err.to_string(),
/// r#"value collision while evaluating 'settings.count'
///
///     in user.json
///   from config.json
/// "#);
/// ```
///
/// For this reason, the [`Error`] type tries to make all relevant
/// information publically accessible. This way you can write another
/// [`Display`] implementation that fits more inline with your vision.
///
/// [`Merge`]: crate::Merge
#[derive(Debug)]
#[allow(clippy::manual_non_exhaustive)]
pub struct Error {
    _priv: (),

    /// Error kind.
    pub kind: ErrorKind,

    /// Module trace.
    ///
    /// This field holds information regarding the module in which the error
    /// occurred. It is a [`Trace`] containing that module and all parent
    /// modules.
    pub modules: Trace,

    /// Value name.
    ///
    /// This field holds the full path of the value which caused the merge
    /// error. The path is stored as a list of components and can be accessed as
    /// an [`Iterator`].
    pub value: Trace,
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self::with_kind(kind)
    }
}

impl Error {
    /// Raised when [`Merge`] encounters 2 values which cannot be merged using
    /// the current strategy.
    ///
    /// [`Merge`]: crate::Merge
    pub fn collision() -> Self {
        Self::with_kind(ErrorKind::Collision)
    }

    /// Raised when evaluation encounters cyclic imports.
    pub fn cycle() -> Self {
        Self::with_kind(ErrorKind::Cycle)
    }

    /// Raised when there is a general error when merging 2 values.
    pub fn custom<T>(msg: T) -> Self
    where
        T: Display + Send + Sync + 'static,
    {
        Self::with_kind(ErrorKind::Custom(Box::new(msg)))
    }

    fn with_kind(kind: ErrorKind) -> Self {
        Self {
            _priv: (),
            kind,
            modules: Trace::new(),
            value: Trace::new(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)?;

        let mut value = self.value.iter();
        if let Some(first) = value.next() {
            write!(f, " while evaluating '{first}")?;
            value.try_for_each(|x| write!(f, ".{x}"))?;
            write!(f, "'")?;
        }

        writeln!(f)?;

        let mut modules = self.modules.iter().rev();
        if let Some(first) = modules.next() {
            writeln!(f)?;
            writeln!(f, "    in {first}")?;
            modules.try_for_each(|x| writeln!(f, "  from {x}"))?;
        }

        Ok(())
    }
}

impl core::error::Error for Error {}
