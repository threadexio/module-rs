[crates-io]: https://crates.io/crates/module
[docs-rs]: https://docs.rs/module/latest/module
[gh-actions]: https://github.com/threadexio/module-rs/actions/workflows/ci.yaml

[license-badge]: https://img.shields.io/github/license/threadexio/module-rs?style=flat-square
[tests-badge]: https://img.shields.io/github/actions/workflow/status/threadexio/module-rs/ci.yaml?style=flat-square
[version-badge]: https://img.shields.io/crates/v/module?style=flat-square
[docs-badge]: https://img.shields.io/docsrs/module?style=flat-square

[examples]: https://github.com/threadexio/module-rs/tree/master/examples
[license]: https://github.com/threadexio/module-rs/blob/master/LICENSE

[`Context`]: https://docs.rs/module/latest/module/error/trait.Error.html
[`Error`]: https://docs.rs/module/latest/module/error/struct.Error.html
[`Merge`]: https://docs.rs/module/latest/module/trait.Merge.html
[`NoMerge`]: https://docs.rs/module/latest/module/types/overridable/struct.NoMerge.html
[`Overridable`]: https://docs.rs/module/latest/module/types/overridable/struct.Overridable.html
[`types`]: https://docs.rs/module/latest/module/types/index.html

[`serde::Deserialize`]: https://docs.rs/serde/latest/serde/trait.Deserialize.html

[`serde`]: https://docs.rs/serde/latest/serde
[`core`]: https://doc.rust-lang.org/stable/core
[`alloc`]: https://doc.rust-lang.org/stable/alloc
[`std`]: https://doc.rust-lang.org/stable/std

<div class="rustdoc-hidden">

<div align="center">
  <br>
  <img src="https://raw.githubusercontent.com/threadexio/module-rs/master/assets/icon.png" width="200em" alt="logo">
  <br>
  <br>
  <br>
  <img src="https://raw.githubusercontent.com/threadexio/module-rs/master/assets/title.svg" width="250em" alt="logo">
  <br>
  <br>

  <p>
    Modular NixOS-style configuration crate.
  </p>

  [![version-badge]][crates-io]
  [![tests-badge]][gh-actions]
  [![docs-badge]][docs-rs]
  [![license-badge]][crates-io]

  <br>
  <br>

</div>

</div>

A crate that provides:

* A standard interface for merging data.

* Strategies for merging that data.

The above can be used to build a module system akin to that of [NixOS](https://wiki.nixos.org/wiki/NixOS_modules).
With this crate you can easily build modular and composable configuration for
your application.

## An overview of module

### The Merge trait

The core functionality of `module` is provided by the [`Merge`] trait, it is the
lowest-level building block of this crate and provides the interface for merging
data. A value is called "mergeable" if it can be merged with the [`Merge`] trait.
A collection of such values is typically referred to as a "module".

### Strategies

Upon the [`Merge`] trait build wrappers like [`Overridable`] and [`NoMerge`]
which provide the "how" to merge data. Most common types from [`core`], [`alloc`]
and [`std`] have a sensible merge strategy. However, these wrappers can be used
to refine the merging of data even more. Such wrappers live under [`types`].

### Errors & Context

When merging complex nested data, it is possible that an error occurs in a merge
operation deep inside the data. [`Error`] and [`Context`] exist to provide useful
errors and context about where the failure happened and why.

To create such complex structures, the crate provides the `Merge` derive macro
that merges each field of the struct and automatically adds the necessary context.

**NOTE:** The derive macro only supports structs. It is not obvious how to merge
enums and a handwritten implementation will probably be better for readability in
this case.

### Evaluators

Just the [`Merge`] trait is not enough however to build a fully functioning
module system from configuration files in some format. Users of the crate must
build an "evaluator".

The evaluator's job is to construct such modules and merge them together.
The source of these modules can be files from disk, environment variables and
generally any medium that can be used to pass information to the app.

Instead of trying to support all these use-cases, this crate requires that users
write their own evaluator.

One fully functional evaluator that reads modules from [TOML](https://toml.io/en/)
files can be found in the [examples].

## Examples

Full examples do not fit here. See [examples] for reimplementations of popular
programs using this crate.

## Features

* `std`: Implement [`Merge`] for many [`std`] types. This is a **default** feature.
Disabling it will also make the entire crate `no_std`. ([`alloc`] is still required)

* `derive`: Enable the [`Merge`] derive macro. This is disabled by default to
avoid introducing the heavy dependencies of proc-macros when not needed.

* `serde`: Implement [`serde::Deserialize`] for types under [`types`],
allowing them to be used seamlessly with [`serde`].

<div class="rustdoc-hidden">

## License

All code in this repository is licensed under the Apache 2.0 license, a copy of
which can be found [here][license].

</div>
