#![allow(missing_docs)]

use module::Merge;
use module_util::file::toml;
use serde::Deserialize;
use std::path::{Path, PathBuf};

fn path(p: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join(p)
}

#[test]
fn test_file_format_toml_simple() {
    #[derive(Deserialize, Merge)]
    struct Simple {
        key: Option<String>,
        items: Option<Vec<i32>>,
    }

    let x: Simple = toml(path("toml/simple1.toml")).unwrap();
    assert_eq!(x.key.as_deref(), Some("424242"));
    assert_eq!(x.items.as_deref(), Some([1, 3, 6, 0].as_slice()));
}
