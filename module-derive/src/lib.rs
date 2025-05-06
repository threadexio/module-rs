//! Derive macros for the [`module`](https://github.com/threadexio/module-rs) crate.

use proc_macro::TokenStream;

mod merge;

/// Derive the `Merge` trait.
#[proc_macro_derive(Merge)]
pub fn merge(item: TokenStream) -> TokenStream {
    self::merge::merge(item)
}
