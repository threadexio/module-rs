use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;

use module::Merge;
use module::types::{NoMerge, Overridable};
use serde::Deserialize;

#[derive(Debug, Deserialize, Merge)]
struct Section {
    #[serde(rename = "User")]
    user: Option<Overridable<String>>,

    #[serde(rename = "HostName")]
    hostname: Option<Overridable<String>>,

    #[serde(rename = "IdentityFile")]
    identity_file: Option<Overridable<PathBuf>>,

    #[serde(rename = "Port")]
    port: Option<Overridable<u16>>,

    // Any options not defined above end up here.
    #[serde(flatten)]
    extra: HashMap<String, NoMerge<toml::Value>>,
}

#[derive(Debug, Deserialize, Merge)]
struct Config {
    #[serde(default, rename = "Host")]
    hosts: HashMap<String, Section>,

    #[serde(default, flatten)]
    global: Section,
}

fn main() {
    let config: Config = match examples::read("config.toml") {
        Ok(x) => x,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };

    println!("{config}");
}

// The rest is just for pretty printing to the console.

impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(user) = self.user.as_deref() {
            writeln!(f, "User {user}")?;
        }

        if let Some(hostname) = self.hostname.as_deref() {
            writeln!(f, "HostName {hostname}")?;
        }

        if let Some(identity_file) = self.identity_file.as_deref() {
            writeln!(f, "IdentityFile {}", identity_file.display())?;
        }

        if let Some(port) = self.port.as_deref() {
            writeln!(f, "Port {port}")?;
        }

        for (k, v) in self.extra.iter() {
            writeln!(f, "{k} {}", &**v)?;
        }

        Ok(())
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.global.fmt(f)?;

        for (host, section) in self.hosts.iter() {
            writeln!(f)?;
            writeln!(f, "Host {host}")?;
            section.fmt(f)?;
        }

        Ok(())
    }
}
