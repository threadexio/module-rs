use std::collections::HashSet;
use std::fmt;
use std::path::{Path, PathBuf};

use module::{Error, Merge};
use serde::de::DeserializeOwned;

use crate::Result;
use crate::evaluator::Evaluator;
use crate::file::Module;

use super::Format;

/// An evaluator for files.
///
/// This evaluator reads modules from files of a specific format. It uses
/// [`Module`] as the top-level format of the module and [`serde`] to parse the
/// contents of the file.
///
/// * [`File`] is capable of detecting import-cycles between modules.
///
/// * Import paths are resolved relative to the path of the importer module.
///
/// # Example
///
/// ```rust,no_run
/// # use module_util::file::File;
/// use module::Merge;
/// use serde::Deserialize;
///
/// #[derive(Deserialize, Merge)]
/// struct Config {
///     key: String,
///     items: Vec<i32>,
/// }
///
/// let mut file = File::json();
///
/// // `config.json`:
/// // --------------
/// // {
/// //   "key": "424242",
/// //   "items": [1]
/// // }
/// assert!(file.read("config.json").is_ok());
///
/// // `config-extra.json`:
/// // --------------------
/// // {
/// //   "items": [3, 6, 0]
/// // }
/// assert!(file.read("config-extra.json").is_ok());
///
/// let config: Config = file.finish().unwrap();
/// assert_eq!(config.key, "424242");
/// assert_eq!(config.items, &[1, 3, 6, 0]);
/// ```
#[derive(Debug)]
pub struct File<T, F> {
    inner: Evaluator<DisplayPath, T>,
    evaluated: HashSet<PathBuf>,
    format: F,
}

impl<T, F> File<T, F> {
    /// Create a new [`File`] that reads files according to `format`.
    pub fn new(format: F) -> Self {
        Self {
            inner: Evaluator::new(),
            evaluated: HashSet::new(),
            format,
        }
    }

    /// Get a reference to the [`Format`] used.
    pub fn format(&self) -> &F {
        &self.format
    }

    /// Get a mutable reference to the [`Format`] used.
    pub fn format_mut(&mut self) -> &mut F {
        &mut self.format
    }

    /// Finish the evaluation and return the final value.
    ///
    /// Returns [`None`] if no file has been [`read()`] successfully. Otherwise,
    /// it returns [`Some(value)`].
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # type File = module_util::file::File<i32, module_util::file::Json>;
    /// let mut file = File::json();
    /// assert_eq!(file.finish(), None);
    ///
    /// let mut file = File::json();
    /// assert!(file.read("non_existent.json").is_err());
    /// assert_eq!(file.finish(), None);
    ///
    /// let mut file = File::json();
    /// assert!(file.read("exists.json").is_ok());
    /// assert!(matches!(file.finish(), Some(_)));
    /// ```
    ///
    /// [`read()`]: File::read
    /// [`Some(value)`]: Some
    pub fn finish(self) -> Option<T> {
        self.inner.finish()
    }
}

impl<T, F> File<T, F>
where
    T: Merge + DeserializeOwned,
    F: Format,
{
    /// Read the module at `path`.
    ///
    /// See the [type-level docs](File) for more information
    pub fn read<P>(&mut self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        self._read(path.as_ref())
    }

    fn _read(&mut self, path: &Path) -> Result<()> {
        self.inner.import(DisplayPath(path.to_path_buf()));

        while let Some(path) = self.inner.next() {
            let DisplayPath(path) = path;

            let realpath = path
                .canonicalize()
                .map_err(Error::custom)
                .map_err(|mut e| {
                    e.trace = self.inner.trace(DisplayPath(path));
                    e
                })?;

            if !self.evaluated.insert(realpath.clone()) {
                return Err(Error::cycle()).map_err(|mut e| {
                    e.trace = self.inner.trace(DisplayPath(realpath));
                    e
                });
            }

            let Module { value, imports } = match self.format.read(&realpath) {
                Ok(x) => x,
                Err(mut e) => {
                    e.trace = self.inner.trace(DisplayPath(realpath));
                    return Err(e);
                }
            };

            // SAFETY: Since `read()` succeeded, this path must point to a
            //         file. File paths always have a parent, the directory they
            //         reside in.
            let basename = realpath
                .parent()
                .expect("file path should always have an ancestor");

            let imports = imports
                .into_iter()
                .map(|x| basename.join(x))
                .map(DisplayPath)
                .collect();

            self.inner.eval(DisplayPath(realpath), imports, value)?;
        }

        Ok(())
    }
}

/// Read the module at `path` with `format`.
///
/// See: [`File`]
#[expect(clippy::missing_panics_doc)]
pub fn read<T, F>(path: impl AsRef<Path>, format: F) -> Result<T, Error>
where
    T: Merge + DeserializeOwned,
    F: Format,
{
    let mut file = File::new(format);
    file.read(path)?;

    // SAFETY: `file` must have read at least one module. If it hadn't, the
    //         above statement should have returned with an error.
    let value = file
        .finish()
        .expect("File should have read at least one module");

    Ok(value)
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

#[derive(Clone)]
struct DisplayPath(PathBuf);

impl fmt::Debug for DisplayPath {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for DisplayPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.display().fmt(f)
    }
}
