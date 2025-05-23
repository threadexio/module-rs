use std::path::{Path, PathBuf};

use serde::Deserialize;
use serde::de::DeserializeOwned;

use module::{Context, Error, Merge};

use std::collections::HashSet;
use std::fs;

#[derive(Debug, Merge, Deserialize)]
struct Module<T> {
    #[serde(default)]
    imports: Vec<PathBuf>,

    #[serde(flatten)]
    inner: T,
}

pub struct Eval<T> {
    evaluated: HashSet<PathBuf>,
    value: Option<T>,
}

impl<T> Eval<T> {
    pub fn new() -> Self {
        Self {
            evaluated: HashSet::new(),
            value: None,
        }
    }
}

impl<T> Default for Eval<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Eval<T>
where
    T: DeserializeOwned + Merge,
{
    fn read(&mut self, path: &Path) -> Result<Module<T>, Error> {
        let path = fs::canonicalize(path).map_err(Error::custom)?;

        if self.evaluated.contains(&path) {
            return Err(Error::cycle());
        }

        let contents = fs::read_to_string(&path).map_err(Error::custom)?;

        let basename = path
            .parent()
            .expect("paths that lead to files must have a parent");

        let mut module: Module<T> = toml::from_str(&contents).map_err(Error::custom)?;

        // Make each import relative to the path of the module which imported it.
        module.imports = module
            .imports
            .into_iter()
            .map(|x| basename.join(x))
            .collect();

        self.evaluated.insert(path);
        Ok(module)
    }

    fn _add(&mut self, path: &Path) -> Result<(), Error> {
        let Module {
            inner: other,
            imports,
        } = self.read(path)?;

        match self.value {
            Some(ref mut value) => value.merge_ref(other)?,
            None => self.value = Some(other),
        }

        for import in imports {
            self.add(&import)?;
        }

        Ok(())
    }

    pub fn add(&mut self, path: impl AsRef<Path>) -> Result<(), Error> {
        let path = path.as_ref();

        self._add(path).with_module(|| path.display().to_string())
    }

    pub fn finish(self) -> Option<T> {
        self.value
    }
}

pub fn read<T>(path: impl AsRef<Path>) -> Result<T, Error>
where
    T: DeserializeOwned + Merge,
{
    let mut eval = Eval::new();
    eval.add(path)?;
    Ok(eval.finish().unwrap())
}
