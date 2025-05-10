//! <style>
//! .rustdoc-hidden { display: none; }
//! </style>
#![doc = include_str!("../README.md")]
#![cfg_attr(module_nightly, feature(doc_auto_cfg))]
#![no_std]
extern crate self as module;

#[macro_use]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod impls;

pub mod error;
pub use self::error::{Context, Error};

pub mod merge;
pub use self::merge::{Merge, merge};

#[cfg(feature = "derive")]
pub use module_derive::Merge;

pub mod types;
