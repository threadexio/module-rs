//! [`Error`] & friends.
//!
//! This module contains all the machinery used to present nice and useful error
//! messages from merge operations.

use core::fmt::{self, Debug, Display, Write};
use core::iter::FusedIterator;
use core::mem::discriminant;

use alloc::boxed::Box;
use alloc::collections::linked_list::{self, LinkedList};
use alloc::string::ToString;

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
            Self::Collision => f.write_str("Collision"),
            Self::Cycle => f.write_str("Cycle"),
            Self::Custom(x) => write!(f, "Custom(\"{x}\")"),
        }
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Collision => f.write_str("value collision"),
            Self::Cycle => f.write_str("cyclic imports"),
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

/// The module backtrace.
#[derive(Clone)]
pub struct Trace {
    modules: LinkedList<Box<str>>,
}

impl Debug for Trace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.modules()).finish()
    }
}

impl<D> FromIterator<D> for Trace
where
    D: Display,
{
    fn from_iter<T: IntoIterator<Item = D>>(iter: T) -> Self {
        Self {
            modules: iter
                .into_iter()
                .map(|x| x.to_string().into_boxed_str())
                .collect(),
        }
    }
}

impl Trace {
    /// Create a new [`Modules`].
    pub const fn new() -> Self {
        Self {
            modules: LinkedList::new(),
        }
    }

    /// Get the number of modules in the backtrace.
    pub fn len(&self) -> usize {
        self.modules.len()
    }

    /// Check if the backtrace has any modules.
    pub fn is_empty(&self) -> bool {
        self.modules.is_empty()
    }

    /// Push `module` to the front.
    pub fn push_front<D>(&mut self, module: D)
    where
        D: Display,
    {
        self.modules.push_front(module.to_string().into_boxed_str());
    }

    /// Push `module` to the back.
    pub fn push_back<D>(&mut self, module: D)
    where
        D: Display,
    {
        self.modules.push_back(module.to_string().into_boxed_str());
    }

    /// Get an iterator over all modules in the backtrace.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use module::merge::error::Trace;
    /// let mut trace = Trace::new();
    ///
    /// trace.push_back("module 1");
    /// trace.push_back("module 2");
    ///
    /// let mut iter = trace.modules();
    /// assert_eq!(iter.next(), Some("module 1"));
    /// assert_eq!(iter.next(), Some("module 2"));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn modules(&self) -> Modules<'_> {
        Modules(self.modules.iter())
    }
}

/// Borrowing iterator for [`Modules`].
pub struct Modules<'a>(linked_list::Iter<'a, Box<str>>);

impl Debug for Modules<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Modules").finish_non_exhaustive()
    }
}

impl<'a> Iterator for Modules<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|x| &**x)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl DoubleEndedIterator for Modules<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(|x| &**x)
    }
}

impl ExactSizeIterator for Modules<'_> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl FusedIterator for Modules<'_> {}

/// The value name.
#[derive(Clone)]
pub struct Value {
    components: LinkedList<Box<str>>,
}

impl Value {
    /// Create a new [`Value`].
    pub const fn new() -> Self {
        Self {
            components: LinkedList::new(),
        }
    }

    /// Get the number of components of the [`Value`].
    pub fn len(&self) -> usize {
        self.components.len()
    }

    /// Check if the [`Value`] has any components.
    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }

    /// Push `component` to the front.
    pub fn push_front<D>(&mut self, component: D)
    where
        D: Display,
    {
        self.components
            .push_front(component.to_string().into_boxed_str());
    }

    /// Push `component` to the back.
    pub fn push_back<D>(&mut self, component: D)
    where
        D: Display,
    {
        self.components
            .push_back(component.to_string().into_boxed_str());
    }

    /// Get an iterator over all components of the value.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use module::merge::error::Value;
    /// let mut value = Value::new();
    ///
    /// value.push_back("value 1");
    /// value.push_back("value 2");
    ///
    /// let mut iter = value.components();
    /// assert_eq!(iter.next(), Some("value 1"));
    /// assert_eq!(iter.next(), Some("value 2"));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn components(&self) -> Components<'_> {
        Components(self.components.iter())
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sep = if f.align().is_some() { f.fill() } else { '.' };

        if !f.alternate() {
            f.write_char('\"')?;
        }

        for (i, component) in self.components().enumerate() {
            if i > 0 {
                f.write_char(sep)?;
            }

            f.write_str(component)?;
        }

        if !f.alternate() {
            f.write_char('\"')?;
        }

        Ok(())
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}

/// Borrowing iterator for [`Value`].
pub struct Components<'a>(linked_list::Iter<'a, Box<str>>);

impl Debug for Components<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Components").finish_non_exhaustive()
    }
}

impl<'a> Iterator for Components<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|x| &**x)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl DoubleEndedIterator for Components<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(|x| &**x)
    }
}

impl ExactSizeIterator for Components<'_> {
    fn len(&self) -> usize {
        self.0.len()
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
/// assert_eq!(format!("{err}"),
/// r#"value collision while evaluating "settings.count"
///
///     in user.json
///   from config.json
/// "#);
///
/// // without quotes...
/// assert_eq!(format!("{err:#}"),
/// r#"value collision while evaluating settings.count
///
///     in user.json
///   from config.json
/// "#);
///
/// // or with a custom separator...
/// assert_eq!(format!("{err:/<}"),
/// r#"value collision while evaluating "settings/count"
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
    pub trace: Trace,

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
            trace: Trace::new(),
            value: Value::new(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.kind, f)?;

        if !self.value.is_empty() {
            f.write_str(" while evaluating ")?;
            Display::fmt(&self.value, f)?;
        }

        f.write_char('\n')?;
        f.write_char('\n')?;

        for (i, module) in self.trace.modules().rev().enumerate() {
            if i == 0 {
                f.write_str("    in ")?;
            } else {
                f.write_str("  from ")?;
            }

            f.write_str(module)?;
            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl core::error::Error for Error {}
