//! The [`File`] evaluator for working with modules from files.

#[allow(clippy::module_inception)]
mod file;
mod format;

pub use self::file::{File, read};
pub use self::format::{Format, Imports, Module};

macro_rules! formats {
    ($(
        $mod:ident::$name:ident $(if $cfg:meta)?,
    )*) => { $(
        $(#[cfg($cfg)])?
        mod $mod;
        $(#[cfg($cfg)])?
        pub use self::$mod::$name;

        impl<T> File<T, $name> {
            #[doc = concat!("Create a new [`File`] that reads [`", stringify!($name), "`] files.")]
            #[doc = ""]
            #[doc = concat!("See: [`", stringify!($name), "`].")]
            #[doc = ""]
            #[doc = concat!("Equivalent to: `File::new(", stringify!($name), "::default())`")]
            pub fn $mod() -> Self {
                Self::new($name::default())
            }
        }

        #[doc = concat!("Read the module at `path` with [`", stringify!($name), "`].")]
        #[doc = ""]
        #[doc = concat!("See: [`", stringify!($name), "`].")]
        pub fn $mod<T>(path: impl AsRef<std::path::Path>) -> Result<T, module::Error>
        where
            T: module::Merge + serde::de::DeserializeOwned,
        {
            read(path, $name::default())
        }
    )* };
}

formats! {
    json::Json if feature = "json",
    toml::Toml if feature = "toml",
    yaml::Yaml if feature = "yaml",
}
