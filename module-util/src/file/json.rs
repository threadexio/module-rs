use std::fs;
use std::io;
use std::path::Path;

use module::Error;
use serde::de::DeserializeOwned;

use super::{Format, Module};

/// A [`Format`] for [JSON] modules.
///
/// Uses [`serde_json`] under the hood.
///
/// [JSON]: https://www.json.org/json-en.html
#[derive(Debug, Default, Clone, Copy)]
pub struct Json;

impl Format for Json {
    fn read<T>(&mut self, path: &Path) -> Result<Module<T>, Error>
    where
        T: DeserializeOwned,
    {
        let reader = fs::File::options()
            .read(true)
            .open(path)
            .map(io::BufReader::new)
            .map_err(Error::custom)?;

        let module = serde_json::from_reader(reader).map_err(Error::custom)?;
        Ok(module)
    }
}
