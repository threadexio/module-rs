//! Evaluation module trace.

use core::iter::FusedIterator;
use core::slice;

use alloc::vec::Vec;

///////////////////////////////////////////////////////////////////////////////

/// A stack-like structure to hold the module backtrace during evaluation.
///
/// A module trace is to the evaluator what [`Backtrace`] is to a program. In
/// the trace, modules are stored in import-order. This means that the module
/// where the error was caused is always the last one in the trace.
///
/// [`Backtrace`]: std::backtrace::Backtrace
#[derive(Debug, Clone)]
pub struct Trace<T>(Vec<T>);

impl<T> Default for Trace<T> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<T> Trace<T> {
    /// Create a new empty [`Trace`].
    #[must_use]
    pub const fn empty() -> Self {
        Self(Vec::new())
    }

    /// Get the number of modules in the trace.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check whether the trace is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Get a reference to the current module.
    ///
    /// The current module is always the one that was [`push`]ed last. Returns
    /// [`None`] if the trace is empty.
    ///
    /// [`push`]: Trace::push
    #[must_use]
    pub fn current(&self) -> Option<&T> {
        self.0.last()
    }

    /// Add a module to the trace.
    ///
    /// Pushing a module makes that module the "deepest" level of the trace.
    /// See: [`Trace`].
    ///
    /// # Example
    ///
    /// ```rust
    /// # use module_util::evaluator::trace::Trace;
    /// let mut trace = Trace::empty();
    /// trace.push("module 1");
    /// trace.push("module 2");
    /// trace.push("module 3");
    ///
    /// assert_eq!(
    ///     trace.iter().copied().collect::<Vec<_>>(),
    ///     &[
    ///         "module 1",
    ///         "module 2",
    ///         "module 3"
    ///     ]
    /// );
    /// ```
    pub fn push(&mut self, id: T) {
        self.0.push(id);
    }

    /// Remove the last module from the trace and return it.
    ///
    /// Also see: [`push`].
    ///
    /// # Example
    /// ```rust
    /// # use module_util::evaluator::trace::Trace;
    /// let mut trace = Trace::empty();
    /// trace.push("module 1");
    /// trace.push("module 2");
    /// trace.push("module 3");
    ///
    /// assert_eq!(trace.pop(), Some("module 3"));
    /// assert_eq!(trace.pop(), Some("module 2"));
    /// assert_eq!(trace.pop(), Some("module 1"));
    /// assert_eq!(trace.pop(), None);
    /// ```
    ///
    /// [`push`]: Trace::push
    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    /// Get an iterator over all modules in the trace.
    ///
    /// The returned iterator traverses the trace from the deepest module to the
    /// shallowest. The returned iterator implements [`DoubleEndedIterator`] so
    /// you can use [`Iterator::rev`].
    ///
    /// # Example
    ///
    /// ```rust
    /// # use module_util::evaluator::trace::Trace;
    /// let mut trace = Trace::empty();
    /// trace.push("module 1");
    /// trace.push("module 2");
    /// trace.push("module 3");
    ///
    /// let mut iter = trace.iter();
    /// assert_eq!(iter.next().copied(), Some("module 1"));
    /// assert_eq!(iter.next().copied(), Some("module 2"));
    /// assert_eq!(iter.next().copied(), Some("module 3"));
    /// assert_eq!(iter.next().copied(), None);
    /// ```
    #[must_use]
    pub fn iter(&self) -> Iter<'_, T> {
        Iter(self.0.iter())
    }
}

impl<'a, T> IntoIterator for &'a Trace<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

///////////////////////////////////////////////////////////////////////////////

/// Borrowing iterator over [`Trace`].
///
/// See: [`Trace::iter`].
#[derive(Debug, Clone)]
pub struct Iter<'a, T>(slice::Iter<'a, T>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<T> DoubleEndedIterator for Iter<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

impl<T> ExactSizeIterator for Iter<'_, T> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<T> FusedIterator for Iter<'_, T> {}
