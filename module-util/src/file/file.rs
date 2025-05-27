use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use module::{Context, Error, Merge};
use serde::de::DeserializeOwned;

use super::{Format, Module};

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
    evaluated: HashSet<PathBuf>,
    value: Option<T>,
    format: F,
}

impl<T, F> File<T, F> {
    /// Create a new [`File`] that reads files according to `format`.
    pub fn new(format: F) -> Self {
        Self {
            evaluated: HashSet::new(),
            value: None,
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
        self.value
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
    pub fn read<P>(&mut self, path: P) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let path = fs::canonicalize(path).map_err(Error::custom)?;
        self._read(&path).with_module(|| DisplayPath(path))
    }

    fn _read(&mut self, path: &Path) -> Result<(), Error> {
        if self.evaluated.contains(path) {
            return Err(Error::cycle());
        }

        let Module { imports, value } = self.format.read(path)?;

        match self.value {
            Some(ref mut x) => x.merge_ref(value)?,
            None => self.value = Some(value),
        }

        let basename = path
            .parent()
            .expect("file path should always have an ancestor")
            .to_path_buf();

        self.evaluated.insert(path.to_path_buf());

        imports
            .0
            .into_iter()
            .map(|x| basename.join(x))
            .try_for_each(|p| self.read(p))
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
    F: Default,
{
    fn default() -> Self {
        Self::new(F::default())
    }
}

struct DisplayPath(PathBuf);

impl fmt::Display for DisplayPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.display().fmt(f)
    }
}
