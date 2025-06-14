//! Derive macros for the [`module`](https://github.com/threadexio/module-rs) crate.
#![forbid(unsafe_code)]

mod merge;

/// Derive the `Merge` trait.
///
/// This macro can be used only on `struct` items.
///
/// Generate a `Merge` implementation for the annotated type. The generated code
/// calls `.merge` and `.merge_ref` on each field.
///
/// # Field attributes
///
/// ## `rename`
///
/// * **Syntax:** `#[merge(rename = "foo")]`
///
/// Rename a field so it appears under a different name in the error context.
///
/// ## `skip`
///
/// * **Syntax:** `#[merge(skip)]`
///
/// Completely skip merging this field. This instructs the macro to not emit
/// code for merging the field. Skipped fields retain the value of `self`.
///
/// ## `with`
///
/// * **Syntax:** `#[merge(with = path::to::custom::merge)]`
///
/// Use `$module::merge` and `$module::merge_ref` to merge this field instead of
/// its own `Merge` implementation.
///
/// This can be used to make external types `Merge` without having to use
/// newtypes.
#[proc_macro_derive(Merge, attributes(merge))]
pub fn merge(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    self::merge::merge(item)
}
