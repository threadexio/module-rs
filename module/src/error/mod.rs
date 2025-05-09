use core::fmt::{self, Debug, Display};
use core::mem::discriminant;

use alloc::boxed::Box;

pub mod context;
pub use self::context::Context;

pub mod trace;
pub use self::trace::Trace;

enum ErrorKind {
    Collision,
    Cycle,
    Custom(Box<dyn Display + Send + Sync + 'static>),
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
/// [`Merge`]: crate::Merge
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,

    /// Module evaluation trace.
    pub modules: Trace,

    /// Value name.
    pub value: Trace,
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
            kind,
            modules: Trace::new(),
            value: Trace::new(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)?;

        if f.alternate() {
            return Ok(());
        }

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

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        self.kind.eq(&other.kind)
    }
}

impl Eq for Error {}
