use std::fs;
use std::path::Path;

use module::Error;
use serde::de::DeserializeOwned;

use super::{Format, Module};

/// A [`Format`] for [TOML] modules.
///
/// Uses [`toml`] under the hood.
///
/// [TOML]: https://toml.io/en/
#[derive(Debug, Default, Clone, Copy)]
pub struct Toml;

impl Format for Toml {
    fn read<T>(&mut self, path: &Path) -> Result<Module<T>, Error>
    where
        T: DeserializeOwned,
    {
        let data = fs::read_to_string(path).map_err(Error::custom)?;
        let module = toml::from_str(&data).map_err(Error::custom)?;
        Ok(module)
    }
}
