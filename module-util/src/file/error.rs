//! [`File`] error.
//!
//! [`File`]: super::File

use std::fmt;
use std::path::PathBuf;
use std::string::ToString;

use crate::evaluator::dfs;

///////////////////////////////////////////////////////////////////////////////

/// Error for the [`File`] evaluator.
///
/// # Example
///
/// ```rust
/// # use module_util::file::error::Error;
/// use module_util::evaluator::dfs;
///
/// let mut err = Error::new(dfs::Error::Merge({
///     let mut e = module::merge::Error::collision();
///     e.field.push_back("my");
///     e.field.push_back("super");
///     e.field.push_back("awesome");
///     e.field.push_back("field");
///     e
/// }));
///
/// err.trace.push("module 1".into());
/// err.trace.push("module 2".into());
/// err.trace.push("module 3".into());
///
/// assert_eq!(format!("{err}"),
/// r#"value collision while evaluating "my.super.awesome.field"
///
///     in module 3
///   from module 2
///   from module 1
/// "#);
///
/// assert_eq!(format!("{err}"), format!("{err:#}"));
/// ```
///
/// ```rust
/// # use module_util::file::error::Error;
/// use module_util::evaluator::dfs;
///
/// let mut err = Error::new(dfs::Error::Merge(module::merge::Error::collision()));
///
/// err.trace.push("module 1".into());
///
/// assert_eq!(format!("{err}"),
/// r#"value collision
///
///     in module 1
/// "#);
///
/// assert_eq!(format!("{err}"), format!("{err:#}"));
/// ```
///
/// ```rust
/// # use module_util::file::error::Error;
/// use module_util::evaluator::dfs;
/// use std::io;
///
/// let mut err = Error::new({
///     let mut e = module::merge::Error::other(io::Error::other("invalid data"));
///     e.field.push_back("user");
///     e.field.push_back("email");
///     dfs::Error::Merge(e)
/// });
///
/// err.trace.push("module 1".into());
/// err.trace.push("module 2".into());
///
/// assert_eq!(format!("{err}"),
/// r#"invalid data while evaluating "user.email"
///
///     in module 2
///   from module 1
/// "#);
///
/// assert_eq!(format!("{err}"), format!("{err:#}"));
/// ```
///
/// ```rust
/// # use module_util::file::error::Error;
/// use module_util::evaluator::dfs;
/// use std::io;
///
/// let err = Error::new(dfs::Error::other(io::Error::other("some other error")))
///     .with_trace({
///         let mut t = dfs::Trace::empty();
///         t.push("module 1".into());
///         t.push("module 2".into());
///         t.push("module 3".into());
///         t
///     });
///
/// assert_eq!(format!("{err}"),
/// r#"some other error
///
///     in module 3
///   from module 2
///   from module 1
/// "#);
///
/// assert_eq!(format!("{err}"), format!("{err:#}"));
/// ```
///
/// [`File`]: super::File
#[expect(clippy::manual_non_exhaustive)]
pub struct Error {
    _priv: (),

    /// Evaluation error.
    pub error: dfs::Error,

    /// Module trace.
    pub trace: dfs::Trace<PathBuf>,
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Error")
            .field("error", &self.error)
            .field("trace", &self.trace)
            .finish()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.error {
            dfs::Error::Merge(ref e) => {
                e.message(f)?;

                if !e.field.is_empty() {
                    write!(f, " while evaluating {:?}", e.field)?;
                }

                writeln!(f)?;
            }

            dfs::Error::Other(ref e) => {
                writeln!(f, "{}", e.to_string().trim_end())?;
            }
        }

        let mut trace = self.trace.iter().rev();
        if let Some(first) = trace.next() {
            writeln!(f)?;
            writeln!(f, "    in {}", first.display())?;
            for path in trace {
                writeln!(f, "  from {}", path.display())?;
            }
        }

        Ok(())
    }
}

impl std::error::Error for Error {}

impl Error {
    /// Create a new [`Error`].
    #[must_use]
    pub const fn new(error: dfs::Error) -> Self {
        Self {
            _priv: (),
            error,
            trace: dfs::Trace::empty(),
        }
    }

    /// Set the `trace` of the error.
    #[must_use]
    pub fn with_trace(mut self, trace: dfs::Trace<PathBuf>) -> Self {
        self.trace = trace;
        self
    }
}
