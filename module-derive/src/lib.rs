//! Derive macros for the [`module`](https://github.com/threadexio/module-rs) crate.

mod merge;

/// Derive the `Merge` trait.
#[proc_macro_derive(Merge, attributes(merge))]
pub fn merge(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    self::merge::merge(item)
}
