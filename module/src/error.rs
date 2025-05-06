use core::fmt;
use core::mem::discriminant;

use alloc::boxed::Box;
use alloc::collections::LinkedList;

mod private {
    pub trait Sealed {}
}
use self::private::Sealed;

/// Provides methods to add context to [`Error`]s.
///
/// This trait is **sealed** and cannot be implemented for types outside of
/// this crate.
pub trait Context: Sealed + Sized {
    /// Add the name of the module in which the error occured as context for the
    /// error.
    fn module<D>(self, module: D) -> Self
    where
        D: fmt::Display + 'static;

    /// Add the name of the module in which the error occured as context for the
    /// error.
    fn with_module<D>(self, module: impl FnOnce() -> D) -> Self
    where
        D: fmt::Display + 'static;

    /// Add the name of the value in which the error occured as context for the
    /// error.
    fn value(self, name: &'static str) -> Self;

    /// Add the name of the value in which the error occured as context for the
    /// error.
    fn with_value(self, name: impl FnOnce() -> &'static str) -> Self;
}

/// Erorr returned by [`Merge`].
///
/// [`Merge`]: crate::Merge
pub struct Error {
    module_context: LinkedList<Box<dyn fmt::Display>>,
    value_context: LinkedList<&'static str>,
    typ: ErrorType,
}

enum ErrorType {
    Collision,
    Custom(Box<dyn fmt::Display>),
}

impl Error {
    /// Raised when [`Merge`] encounters 2 values which cannot be merged using
    /// the current strategy.
    ///
    /// [`Merge`]: crate::Merge
    pub fn collision() -> Self {
        Self {
            module_context: LinkedList::new(),
            value_context: LinkedList::new(),
            typ: ErrorType::Collision,
        }
    }

    /// Raised when there is a general error when merging 2 values.
    pub fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display + 'static,
    {
        Self {
            module_context: LinkedList::new(),
            value_context: LinkedList::new(),
            typ: ErrorType::Custom(Box::new(msg)),
        }
    }

    /// Get the name of the module in which the error occured.
    ///
    /// Module names can be added via the [`Context`] trait on merge operations.
    /// If no module information is added, this method will return [`None`].
    ///
    /// # Example
    ///
    /// ```no_run
    /// use module::{Merge, Context};
    /// # let a: i32 = 0;
    /// # let b: i32 = 1;
    /// let err = a.merge(b).module("module A").unwrap_err();
    ///
    /// let module = err.module().unwrap().to_string();
    /// assert_eq!(module, "module A");
    /// ```
    pub fn module(&self) -> Option<&dyn fmt::Display> {
        self.module_context.front().map(|x| x.as_ref())
    }
}

impl<T> Sealed for core::result::Result<T, Error> {}

impl<T> Context for core::result::Result<T, Error> {
    fn module<D>(self, module: D) -> Self
    where
        D: fmt::Display + 'static,
    {
        self.with_module(|| module)
    }

    fn with_module<D>(self, module: impl FnOnce() -> D) -> Self
    where
        D: fmt::Display + 'static,
    {
        self.map_err(|mut e| {
            let item = Box::new(module());
            e.module_context.push_back(item);
            e
        })
    }

    fn value(self, name: &'static str) -> Self {
        self.with_value(|| name)
    }

    fn with_value(self, name: impl FnOnce() -> &'static str) -> Self {
        self.map_err(|mut e| {
            e.value_context.push_front(name());
            e
        })
    }
}

impl fmt::Debug for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Collision => write!(f, "Collision"),
            Self::Custom(x) => write!(f, "Custom(\"{x}\")"),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct DisplayToDebugAdapter<T>(T);

        impl<T: fmt::Display> fmt::Debug for DisplayToDebugAdapter<T> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }

        struct ModuleContext<'a>(&'a LinkedList<Box<dyn fmt::Display>>);

        impl fmt::Debug for ModuleContext<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_list()
                    .entries(self.0.iter().map(DisplayToDebugAdapter))
                    .finish()
            }
        }

        f.debug_struct("Error")
            .field("module_context", &ModuleContext(&self.module_context))
            .field("value_context", &self.value_context)
            .field("typ", &self.typ)
            .finish()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.typ {
            ErrorType::Collision => {
                write!(f, "value collision")?;

                let mut value_context = self.value_context.iter();

                if let Some(first) = value_context.next() {
                    write!(f, " at {}", first)?;
                }

                for item in value_context {
                    write!(f, ".{}", item)?;
                }

                writeln!(f)?;
            }
            ErrorType::Custom(ref msg) => writeln!(f, "{msg}")?,
        }

        let mut module_context = self.module_context.iter();

        if let Some(first) = module_context.next() {
            writeln!(f, "    in {}", first)?;
        }

        for item in module_context {
            writeln!(f, "  from {}", item)?;
        }

        writeln!(f)?;

        match self.typ {
            ErrorType::Collision => writeln!(f, "Try changing the priority of a value.")?,
            ErrorType::Custom(_) => {}
        }

        Ok(())
    }
}

impl core::error::Error for Error {}

impl PartialEq for ErrorType {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}

impl Eq for ErrorType {}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        self.typ == other.typ
    }
}

impl Eq for Error {}
