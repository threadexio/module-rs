#![allow(missing_docs)]

use module::Merge;
use module::merge::ErrorKind;
use module::types::Overridable;
use serde::Deserialize;
use std::path::{Path, PathBuf};

use module_util::file::json;

fn path(p: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join(p)
}

#[test]
fn test_file_simple() {
    #[derive(Deserialize, Merge)]
    struct Simple {
        key: Option<String>,
        items: Option<Vec<i32>>,
    }

    let x: Simple = json(path("json/simple1.json")).unwrap();
    assert_eq!(x.key.as_deref(), Some("424242"));
    assert_eq!(x.items.as_deref(), Some([1, 3, 6, 0].as_slice()));
}

#[test]
fn test_file_no_imports() {
    #[derive(Deserialize, Merge)]
    struct NoImports {
        value: Option<i32>,
    }

    let x: NoImports = json(path("json/no_imports.json")).unwrap();
    assert_eq!(x.value, Some(42));
}

#[test]
fn test_file_relative_imports() {
    #[derive(Deserialize, Merge)]
    struct RelativeImports {
        value: Option<Overridable<i32>>,
    }

    let x: RelativeImports = json(path("json/relative_imports.json")).unwrap();
    assert_eq!(x.value.as_deref().copied(), Some(46));
}

#[test]
fn test_file_cycle() {
    #[derive(Debug, Deserialize, Merge)]
    struct Cycle;

    let err = json::<Cycle>(path("json/cycle.json")).unwrap_err();
    assert_eq!(err.kind, ErrorKind::Cycle);
}

#[test]
fn test_file_cycle2() {
    #[derive(Debug, Deserialize, Merge)]
    struct Cycle;

    let err = json::<Cycle>(path("json/cycle2.json")).unwrap_err();
    assert_eq!(err.kind, ErrorKind::Cycle);
}
