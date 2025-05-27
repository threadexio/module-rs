use std::fmt;
use std::path::{Path, PathBuf};

use module::Error;
use serde::Deserialize;
use serde::de::DeserializeOwned;

/// Imports of a [`Module`].
///
/// See: [`Module::imports`]
#[derive(Default, Clone, Deserialize)]
pub struct Imports(pub(crate) Vec<PathBuf>);

impl fmt::Debug for Imports {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<Vec<PathBuf>> for Imports {
    fn from(value: Vec<PathBuf>) -> Self {
        Self(value)
    }
}

impl<A> FromIterator<A> for Imports
where
    A: Into<PathBuf>,
{
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        Self(iter.into_iter().map(Into::into).collect())
    }
}

/// The top-level structure of a [`File`] module.
///
/// [`File`]: super::File
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Module<T> {
    /// Imports of the module.
    ///
    /// This field instructs [`File`] to additionally [`read()`] all modules
    /// specified here.
    ///
    /// [`File`]: super::File
    /// [`read()`]: super::File::read
    #[serde(default)]
    pub imports: Imports,

    /// Value of the module.
    #[serde(flatten)]
    pub value: T,
}

/// The format of a file.
///
/// The job of a [`Format`] is to read a file, parse it and convert it to a
/// [`Module`] so it can be merged.
///
/// [`File`]: super::File
pub trait Format {
    /// Read the module at `path`.
    ///
    /// See [trait-level docs](Format) for more information.
    fn read<T>(&mut self, path: &Path) -> Result<Module<T>, Error>
    where
        T: DeserializeOwned;
}
