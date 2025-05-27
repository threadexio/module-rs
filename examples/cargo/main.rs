use std::collections::HashMap;
use std::env::args_os;
use std::fmt;
use std::net::SocketAddr;
use std::path::PathBuf;

use module::Merge;
use serde::Deserialize;

const DEFAULT_PRIORITY: isize = 50;

// Change the default priority with a type alias.
type Overridable<T> = module::types::Overridable<T, DEFAULT_PRIORITY>;

#[derive(Deserialize)]
#[serde(untagged)]
enum Alias {
    Concat(String),
    Args(Vec<String>),
}

impl Merge for Alias {
    fn merge_ref(&mut self, _: Self) -> Result<(), module::Error> {
        Err(module::Error::collision())
    }
}

impl fmt::Debug for Alias {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Concat(x) => x.fmt(f),
            Self::Args(x) => x.fmt(f),
        }
    }
}

#[derive(Debug, Default, Deserialize, Merge)]
#[serde(rename_all = "kebab-case")]
struct Build {
    jobs: Option<Overridable<usize>>,
    rustc: Option<Overridable<PathBuf>>,
    rustc_wrapper: Option<Overridable<PathBuf>>,
    rustc_workspace_wrapper: Option<Overridable<PathBuf>>,
    rustdoc: Option<Overridable<PathBuf>>,
    target: Option<Overridable<PathBuf>>,
    target_dir: Option<Overridable<PathBuf>>,

    #[serde(default)]
    rustflags: Vec<String>,

    #[serde(default)]
    rustdocflags: Vec<String>,

    incremental: Option<Overridable<bool>>,
    dep_info_basedir: Option<Overridable<PathBuf>>,
}

#[derive(Debug, Default, Deserialize, Merge)]
#[serde(rename_all = "kebab-case")]
struct Doc {
    browser: Option<Overridable<PathBuf>>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum EnvVar {
    Relative { value: PathBuf, relative: bool },
    Force { value: PathBuf, force: bool },
    Raw(String),
}

impl Merge for EnvVar {
    fn merge_ref(&mut self, _: Self) -> Result<(), module::Error> {
        Err(module::Error::collision())
    }
}

impl fmt::Debug for EnvVar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Relative { value, relative } => f
                .debug_struct("EnvVar")
                .field("value", value)
                .field("relative", relative)
                .finish(),

            Self::Force { value, force } => f
                .debug_struct("EnvVar")
                .field("value", value)
                .field("force", force)
                .finish(),

            Self::Raw(value) => value.fmt(f),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum FutureIncompatReportFrequency {
    Always,
    Never,
}

impl Merge for FutureIncompatReportFrequency {
    fn merge_ref(&mut self, _: Self) -> Result<(), module::Error> {
        Err(module::Error::collision())
    }
}

#[derive(Debug, Default, Deserialize, Merge)]
#[serde(rename_all = "kebab-case")]
struct FutureIncompatReport {
    frequency: Option<Overridable<FutureIncompatReportFrequency>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Vcs {
    Git,
    Hg,
    Pijul,
    Fossil,
    None,
}

impl Merge for Vcs {
    fn merge_ref(&mut self, _: Self) -> Result<(), module::Error> {
        Err(module::Error::collision())
    }
}

#[derive(Debug, Default, Deserialize, Merge)]
#[serde(rename_all = "kebab-case")]
struct CargoNew {
    vcs: Option<Overridable<Vcs>>,
}

#[derive(Deserialize)]
enum SslVersion {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "tlsv1.0", alias = "tlsv1")]
    Tlsv1_0,
    #[serde(rename = "tlsv1.1")]
    Tlsv1_1,
    #[serde(rename = "tlsv1.2")]
    Tlsv1_2,
    #[serde(rename = "tlsv1.3")]
    Tlsv1_3,
}

impl Merge for SslVersion {
    fn merge_ref(&mut self, _: Self) -> Result<(), module::Error> {
        Err(module::Error::collision())
    }
}

impl fmt::Debug for SslVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default => f.write_str("default"),
            Self::Tlsv1_0 => f.write_str("TLSv1.0"),
            Self::Tlsv1_1 => f.write_str("TLSv1.1"),
            Self::Tlsv1_2 => f.write_str("TLSv1.2"),
            Self::Tlsv1_3 => f.write_str("TLSv1.3"),
        }
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum SslVersionTable {
    MinMax { min: SslVersion, max: SslVersion },
    Strict(SslVersion),
}

impl Merge for SslVersionTable {
    fn merge_ref(&mut self, _: Self) -> Result<(), module::Error> {
        Err(module::Error::collision())
    }
}

impl fmt::Debug for SslVersionTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MinMax { min, max } => f
                .debug_struct("SslVersionTable")
                .field("min", min)
                .field("max", max)
                .finish(),

            Self::Strict(version) => version.fmt(f),
        }
    }
}

#[derive(Debug, Default, Deserialize, Merge)]
#[serde(rename_all = "kebab-case")]
struct Http {
    debug: Option<Overridable<bool>>,
    proxy: Option<Overridable<SocketAddr>>,
    ssl_version: Option<Overridable<SslVersionTable>>,
    low_speed_limit: Option<Overridable<usize>>,
    multiplexing: Option<Overridable<bool>>,
    user_agent: Option<Overridable<String>>,
}

// You get the point...

#[derive(Debug, Default, Deserialize, Merge)]
#[serde(default, rename_all = "kebab-case")]
struct Config {
    alias: HashMap<String, Overridable<Alias>>,
    build: Build,
    credential_alias: HashMap<String, Overridable<Alias>>,
    doc: Doc,
    env: HashMap<String, Overridable<EnvVar>>,
    future_incompat_report: FutureIncompatReport,
    cargo_new: CargoNew,
    http: Http,
}

fn main() {
    let mut args = args_os().skip(1);

    let config_path = args.next().unwrap_or_else(|| "config.toml".into());

    let config: Config = match module_util::file::toml(&config_path) {
        Ok(x) => x,
        Err(e) => {
            eprintln!(
                "error: failed to read config `{}`: {e}",
                config_path.to_string_lossy()
            );
            return;
        }
    };

    println!("{config:#?}");
}
