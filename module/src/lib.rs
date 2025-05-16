//! <style>
//! .rustdoc-hidden { display: none; }
//! </style>
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/threadexio/module-rs/master/assets/icon.png"
)]
#![doc = include_str!("../README.md")]
#![cfg_attr(module_nightly, feature(doc_auto_cfg))]
#![no_std]
extern crate self as module;

#[macro_use]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod merge;

#[doc(inline)]
pub use self::merge::{Context, Error, Merge, merge};

#[cfg(feature = "derive")]
pub use module_derive::Merge;

pub mod types;

#[cfg(test)]
mod test;
