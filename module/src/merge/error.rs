//! [`Error`] & friends.
//!
//! This module contains all the machinery used to present nice and useful error
//! messages from merge operations.

use core::fmt::{self, Debug, Display};
use core::iter::FusedIterator;
use core::mem::discriminant;

use alloc::boxed::Box;
use alloc::collections::linked_list::{self, LinkedList};

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

type BoxedDisplay = Box<dyn Display + Send + Sync + 'static>;

/// The module backtrace.
pub struct Modules {
    list: LinkedList<BoxedDisplay>,
}

impl Modules {
    /// Create a new [`Modules`].
    pub fn new() -> Self {
        Self {
            list: LinkedList::new(),
        }
    }

    /// Get the number of modules in the backtrace.
    pub fn len(&self) -> usize {
        self.list.len()
    }

    /// Check if the backtrace has any modules.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Push `module`.
    pub fn push<D>(&mut self, module: D)
    where
        D: Display + Send + Sync + 'static,
    {
        self.list.push_front(Box::new(module));
    }

    /// Get an iterator over all modules in the backtrace.
    ///
    /// The returned iterator iterates over all modules in the reverse order
    /// they were [`push`]ed.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use module::merge::error::Modules;
    /// let mut modules = Modules::new();
    ///
    /// modules.push("module 1");
    /// modules.push("module 2");
    ///
    /// let mut iter = modules.iter().map(|x| x.to_string());
    /// assert_eq!(iter.next().as_deref(), Some("module 2"));
    /// assert_eq!(iter.next().as_deref(), Some("module 1"));
    /// assert_eq!(iter.next(), None);
    /// ```
    ///
    /// [`push`]: Modules::push
    pub fn iter(&self) -> ModulesIter<'_> {
        ModulesIter {
            iter: self.list.iter(),
        }
    }
}

impl Debug for Modules {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(self.list.iter().map(DisplayToDebug))
            .finish()
    }
}

/// Borrowing iterator for [`Modules`].
pub struct ModulesIter<'a> {
    iter: linked_list::Iter<'a, BoxedDisplay>,
}

impl Debug for ModulesIter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ModulesIter").finish_non_exhaustive()
    }
}

impl<'a> Iterator for ModulesIter<'a> {
    type Item = &'a (dyn Display + Send + Sync + 'static);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(AsRef::as_ref)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.len()))
    }
}

impl DoubleEndedIterator for ModulesIter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(AsRef::as_ref)
    }
}

impl ExactSizeIterator for ModulesIter<'_> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl FusedIterator for ModulesIter<'_> {}

/// The module backtrace.
pub struct Value {
    list: LinkedList<BoxedDisplay>,
}

impl Value {
    /// Create a new [`Value`].
    pub fn new() -> Self {
        Self {
            list: LinkedList::new(),
        }
    }

    /// Get the number of components of the [`Value`].
    pub fn len(&self) -> usize {
        self.list.len()
    }

    /// Check if the [`Value`] has any components.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Push `component`.
    pub fn push<D>(&mut self, component: D)
    where
        D: Display + Send + Sync + 'static,
    {
        self.list.push_front(Box::new(component));
    }

    /// Get an iterator over all components of the value.
    ///
    /// The returned iterator iterates over all components in the reverse order
    /// they were [`push`]ed.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use module::merge::error::Value;
    /// let mut value = Value::new();
    ///
    /// value.push("value 1");
    /// value.push("value 2");
    ///
    /// let mut iter = value.components().map(|x| x.to_string());
    /// assert_eq!(iter.next().as_deref(), Some("value 2"));
    /// assert_eq!(iter.next().as_deref(), Some("value 1"));
    /// assert_eq!(iter.next(), None);
    /// ```
    ///
    /// [`push`]: Modules::push
    pub fn components(&self) -> Components<'_> {
        Components {
            iter: self.list.iter(),
        }
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "'")?;

        let mut iter = self.components();
        if let Some(first) = iter.next() {
            write!(f, "{first}")?;
            iter.try_for_each(|x| write!(f, ".{x}"))?;
        }

        write!(f, "'")?;
        Ok(())
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}

/// Borrowing iterator for [`Value`].
pub struct Components<'a> {
    iter: linked_list::Iter<'a, BoxedDisplay>,
}

impl Debug for Components<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ModulesIter").finish_non_exhaustive()
    }
}

impl<'a> Iterator for Components<'a> {
    type Item = &'a (dyn Display + Send + Sync + 'static);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(AsRef::as_ref)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.len()))
    }
}

impl DoubleEndedIterator for Components<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(AsRef::as_ref)
    }
}

impl ExactSizeIterator for Components<'_> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl FusedIterator for Components<'_> {}

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
    /// occurred.
    pub modules: Modules,

    /// Value name.
    ///
    /// This field holds the full path of the value which caused the merge
    /// error. The path is stored as a list of components and can be accessed as
    /// an [`Iterator`].
    pub value: Value,
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
            modules: Modules::new(),
            value: Value::new(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)?;

        if !self.value.is_empty() {
            write!(f, " while evaluating {}", self.value)?;
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

struct DisplayToDebug<T>(T);

impl<T> fmt::Debug for DisplayToDebug<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
