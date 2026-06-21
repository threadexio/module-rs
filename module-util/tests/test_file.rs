#![allow(missing_docs, clippy::unwrap_used)]

use module::Merge;
use module::types::Overridable;
use serde::Deserialize;
use std::path::{Path, PathBuf};

use module_util::file::json;

fn path(p: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join(p)
}

#[test]
fn test_file_simple() {
    #[derive(Debug, Deserialize, Merge)]
    struct Simple {
        key: Option<String>,
        items: Option<Vec<i32>>,
    }

    let x: Simple = json([path("json/simple1.json")]).unwrap().unwrap();
    assert_eq!(x.key.as_deref(), Some("424242"));
    assert_eq!(x.items.as_deref(), Some([1, 3, 6, 0].as_slice()));
}

#[test]
fn test_file_no_imports() {
    #[derive(Debug, Deserialize, Merge)]
    struct NoImports {
        value: Option<i32>,
    }

    let x: NoImports = json([path("json/no_imports.json")]).unwrap().unwrap();
    assert_eq!(x.value, Some(42));
}

#[test]
fn test_file_relative_imports() {
    #[derive(Debug, Deserialize, Merge)]
    struct RelativeImports {
        value: Option<Overridable<i32>>,
    }

    let x: RelativeImports = json([path("json/relative_imports.json")]).unwrap().unwrap();
    assert_eq!(x.value.as_deref().copied(), Some(46));
}

#[test]
fn test_file_cycle() {
    #[derive(Debug, Deserialize, Merge, PartialEq, Eq)]
    struct Cycle;

    let _: Cycle = json([path("json/cycle.json")]).unwrap().unwrap();
}

#[test]
fn test_file_cycle2() {
    #[derive(Debug, Deserialize, Merge)]
    struct Cycle;

    let _: Cycle = json([path("json/cycle2.json")]).unwrap().unwrap();
}

#[test]
fn test_file_error_trace() {
    #[derive(Debug, Deserialize, Merge)]
    struct Module;

    let err = json::<Module, _>([path("json/nonexistent.json")]).unwrap_err();
    assert_eq!(Vec::from(err.trace), &[path("json/nonexistent.json")]);

    let err = json::<Module, _>([path("json/nonexistent1.json")]).unwrap_err();
    assert_eq!(
        Vec::from(err.trace),
        &[
            path("json/nonexistent1.json"),
            path("json/nonexistent2.json")
        ]
    );
}
