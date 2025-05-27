use std::fs;
use std::io;
use std::path::Path;

use module::Error;
use serde::de::DeserializeOwned;

use super::{Format, Module};

/// A [`Format`] for [YAML] modules.
///
/// Uses [`serde_yaml`] under the hood.
///
/// [YAML]: https://yaml.org/
#[derive(Debug, Default, Clone, Copy)]
pub struct Yaml;

impl Format for Yaml {
    fn read<T>(&mut self, path: &Path) -> Result<Module<T>, Error>
    where
        T: DeserializeOwned,
    {
        let reader = fs::File::options()
            .read(true)
            .open(path)
            .map(io::BufReader::new)
            .map_err(Error::custom)?;

        let module = serde_yaml::from_reader(reader).map_err(Error::custom)?;
        Ok(module)
    }
}
