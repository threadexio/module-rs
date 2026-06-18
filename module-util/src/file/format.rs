//! File formats.

use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::evaluator::Imports;

///////////////////////////////////////////////////////////////////////////////

/// A template module that all file formats should deserialize into.
#[derive(Debug, Clone, Deserialize)]
pub struct Module<T> {
    /// Imports of the module.
    #[serde(default)]
    pub imports: Imports<PathBuf>,

    /// Module body.
    #[serde(flatten)]
    pub body: T,
}

/// A file format.
///
/// This trait describes how to read and deserialize the contents of a file into
/// a module.
pub trait Format<T> {
    /// Error for [`Format::read`].
    type Error: std::error::Error + 'static;

    /// Read the file at `path` and deserialize the module in it.
    fn read(&mut self, path: &Path) -> Result<Module<T>, Self::Error>;
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "json")]
mod json {
    use super::*;

    use std::io;

    /// The [JSON](https://www.json.org/json-en.html) file format.
    #[derive(Debug, Default, Clone, Copy)]
    pub struct Json;

    impl<T> Format<T> for Json
    where
        T: for<'de> Deserialize<'de>,
    {
        type Error = io::Error;

        fn read(&mut self, path: &Path) -> Result<Module<T>, Self::Error> {
            let mut reader = fs::File::open(path).map(io::BufReader::new)?;
            serde_json::from_reader(&mut reader).map_err(Into::into)
        }
    }
}

#[cfg(feature = "json")]
pub use self::json::Json;

///////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "toml")]
mod toml {
    use super::*;

    use ::toml::de::Error;
    use serde::de::Error as _;

    /// The [TOML](https://toml.io/en/) file format.
    #[derive(Debug, Default, Clone, Copy)]
    pub struct Toml;

    impl<T> Format<T> for Toml
    where
        T: for<'de> Deserialize<'de>,
    {
        type Error = Error;

        fn read(&mut self, path: &Path) -> Result<Module<T>, Self::Error> {
            let bytes = fs::read(path).map_err(Error::custom)?;
            ::toml::from_slice(&bytes)
        }
    }
}

#[cfg(feature = "toml")]
pub use self::toml::Toml;
