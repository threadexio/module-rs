//! Modular NixOS-style configuration crate.
//!
//! This crate provides most essentials to start building configuration files
//! that work like the modules from NixOS. Perhaps, you might find the [official
//! wiki](https://wiki.nixos.org/wiki/NixOS_modules) helpful.
//!
//! Essentially, this crate implements the mechanism by which values from
//! different modules are merged together. This is done via the [`Merge`] trait,
//! which is the core of this crate. The logic of how multiple values are merged
//! together is up to each implementation of [`Merge`]. For example, merging 2
//! [`Vec<T>`] is done by concatenating the 2 [`Vec`]s. Maps are merged by
//! recursively merging common keys and keeping all other unique keys.
//!
//! Implementations of [`Merge`] for many common structures are given by this
//! crate. It is helpful to think of each structure are providing a separate
//! property to the value. For instance, a value `type: Option<String>` means
//! that each module has the ability to define `type`, but in the end only one
//! module must do so. Following the same logic `type: Option<Overridable<String>>`
//! means that `type` may be defined by none, one or many modules with the
//! ability to be overridden by a priority defined by each module. In addition,
//! the definition `type: String` does not make sense. This is because each
//! module must set `type` but with no ability to override that value, resulting
//! in a collision error.
//!
//! This crate is **no_std** compatible by turning off the default feature `std`.
//!
//! [`Vec<T>`]: alloc::vec::Vec
//! [`Vec`]: alloc::vec::Vec

#![no_std]
#[macro_use]
extern crate alloc;
extern crate self as module;

#[cfg(feature = "std")]
extern crate std;

mod impls;

pub mod error;
pub use self::error::Error;

pub mod merge;
pub use self::merge::Merge;
pub use self::merge::merge;

#[cfg(feature = "derive")]
pub use module_derive::Merge;

pub mod types;
