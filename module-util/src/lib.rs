#![doc(
    html_logo_url = "https://raw.githubusercontent.com/threadexio/module-rs/master/assets/icon.png"
)]
#![doc = include_str!("../README.md")]
#![cfg_attr(module_nightly, feature(doc_cfg))]
#![no_std]
extern crate self as module_util;

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod evaluator;

#[cfg(feature = "file")]
pub mod file;

/// `Result<T, module::Error>`
pub type Result<T, E = module::Error> = core::result::Result<T, E>;
