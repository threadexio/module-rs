//! The [`File`] evaluator for working with modules from files.

use std::mem::take;
use std::path::PathBuf;
use std::vec::Vec;

use module::merge::Merge;
use serde::Deserialize;

use crate::evaluator::dfs;
use crate::evaluator::{Acyclic, Dfs, Evaluator, Imports};

pub mod format;
pub use self::format::{Format, Module};

pub mod error;
pub use self::error::Error;

///////////////////////////////////////////////////////////////////////////////

/// The [`File`] evaluator.
///
/// An [`Evaluator`] accompanied by its driving logic. If you are trying to use
/// this crate, this is probably where you should be looking.
///
/// # Example
///
/// ```rust,no_run
/// # use std::path::PathBuf;
/// # use module_util::file::{File, format};
/// #[derive(Debug, serde::Deserialize, module::Merge)]
/// struct MyModule {
///     a: Option<i32>,
///     b: Option<Vec<usize>>,
/// }
///
/// let mut file: File<MyModule, format::Json> = File::default();
///
/// file.read(PathBuf::from("./module1.json")).unwrap();
/// file.read(PathBuf::from("./module2.json")).unwrap();
///
/// assert!(file.finish().is_some());
/// ```
#[derive(Debug)]
pub struct File<T, F>
where
    T: Merge,
{
    evaluator: Acyclic<Dfs<PathBuf, T>>,
    format: F,
}

impl<T, F> File<T, F>
where
    T: Merge,
{
    /// Create a new [`File`] evaluator that reads modules based on `format`.
    ///
    /// See: [`Format`].
    pub fn new(format: F) -> Self {
        Self {
            evaluator: Default::default(),
            format,
        }
    }
}

impl<T, F> Default for File<T, F>
where
    T: Merge,
    F: Default,
{
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<T, F> File<T, F>
where
    T: Merge,
    F: Format<T>,
    F::Error: Send + Sync,
{
    /// Read the module from the file at `path`.
    ///
    /// This method will evaluate `path` along with all of its imports. It
    /// should be used to declare only the root of the module tree.
    ///
    /// # Example
    ///
    /// See: [`File`].
    pub fn read(&mut self, path: PathBuf) -> Result<(), Error> {
        self.drive_to_end()?;
        self.eval_one(path)?;
        self.drive_to_end()?;
        Ok(())
    }

    /// Destruct the evaluator and get the final value.
    ///
    /// # Example
    ///
    /// See: [`File`].
    pub fn finish(self) -> Option<T> {
        self.evaluator.finish()
    }

    fn eval_one(&mut self, path: PathBuf) -> Result<(), Error> {
        let Module { imports, body } = self.format.read(&path).map_err(|e| {
            let mut trace = self.take_trace();

            // We have to push here since the underlying evaluator has not seen
            // this module yet.
            trace.push(path.clone());

            Error {
                trace,
                error: dfs::Error::other(e),
            }
        })?;

        let dirname = path
            .parent()
            .expect("read() succeeded so it should have a parent");

        let imports = {
            let mut tmp = Vec::from(imports);
            for import in &mut tmp {
                *import = dirname.join(&import);
            }

            Imports::from(tmp)
        };

        self.evaluator
            .eval(path, imports, body)
            .map_err(|error| Error {
                // We don't have to push here since the underlying evaluator has
                // seen the module and inserted it into the trace itself.
                trace: self.take_trace(),
                error,
            })?;

        Ok(())
    }

    fn drive_to_end(&mut self) -> Result<(), Error> {
        while let Some(path) = self.evaluator.next() {
            self.eval_one(path)?;
        }

        Ok(())
    }

    fn take_trace(&mut self) -> dfs::Trace<PathBuf> {
        let dfs = self.evaluator.get_mut();
        take(&mut dfs.trace)
    }
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "json")]
/// Evaluate `paths` using [`File`] and the [`Json`] format.
///
/// This is a convenience function and does no magic. Returns `Ok(None)` only if
/// `paths` is empty.
///
/// [`Json`]: format::Json
pub fn json<T, I>(paths: I) -> Result<Option<T>, Error>
where
    T: for<'de> Deserialize<'de> + Merge,
    I: IntoIterator<Item = PathBuf>,
{
    let mut file = File::new(format::Json);

    for path in paths {
        file.read(path)?;
    }

    Ok(file.finish())
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "toml")]
/// Evaluate `paths` using [`File`] and the [`Toml`] format.
///
/// This is a convenience function and does no magic. Returns `Ok(None)` only if
/// `paths` is empty.
///
/// [`Toml`]: format::Toml
pub fn toml<T, I>(paths: I) -> Result<Option<T>, Error>
where
    T: for<'de> Deserialize<'de> + Merge,
    I: IntoIterator<Item = PathBuf>,
{
    let mut file = File::new(format::Toml);

    for path in paths {
        file.read(path)?;
    }

    Ok(file.finish())
}
