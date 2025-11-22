use std::path::{Path, PathBuf};

use module::Error;
use serde::Deserialize;
use serde::de::DeserializeOwned;

use crate::evaluator::Imports;

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
    pub imports: Imports<PathBuf>,

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
