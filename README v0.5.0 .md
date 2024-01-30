# HashMapSettings [![Version]][Crates.io] [![Rust Version]][Nightly] [![Documentation]][Docs.rs] [![Build Status]][Actions]

[Version]: https://img.shields.io/crates/v/hashmap_settings.svg
[Crates.io]: https://crates.io/crates/hashmap_settings
[Documentation]: https://img.shields.io/docsrs/hashmap_settings/latest
[Docs.rs]: https://docs.rs/hashmap_settings
[Build Status]: https://img.shields.io/github/actions/workflow/status/OxidizedLoop/HashMapSettings/rust.yml
[Actions]: https://github.com/OxidizedLoop/HashMapSettings/actions
[Rust Version]: https://img.shields.io/badge/rust-nightly-lightgray.svg
[Nightly]: https://github.com/rust-lang/rust/issues/65991

## `HashMap` wrapper for layered settings

This crate facilitates the management of settings, with the goal of allowing developers to turn previously needlessly set in stone
values into a setting a user can change, as well as making it easier for a developer to create multiple priority levels of settings,
allowing users to have greater control and customization including across devices.

This crate allows a developer to store and access all program settings on a `Account`,
a wrapper around a `HashMap`.

This crate is intended to be used with some sort of type abstraction so that settings of distinct types can be stored in
a single `Account`. This crate provides the `Stg` type abstraction for this.

An `Account` can also hold other Accounts, allowing the existence of layered settings,
that permit the creation complex systems that have the:

### Benefits

1. Having multiple places changing the same setting with the value being taken from the place that is deemed
to have the most importance.
(eg: Default, Themes, Extensions, Global Settings, OS Settings, Device Settings, Temporary Settings )

2. Organization of Settings. Given that an `Account` can hold accounts, and they can hold accounts of they own, its possible for
small groups of settings to be organized in an `Account`, making it more convenient to locate a setting, or display a group of settings.
Important to notice that this organization doesn't need to be (but could be) enforced in all held accounts equally.

3. `Account`s can be individually deactivated allowing for a developer (or a user)
to group settings in an `Account` and easily ignore them under certain conditions.

### Drawbacks

1. Each `Account` holds a copy of the settings present in it's child Accounts, so there is a memory cost, but its
[planned](https://github.com/OxidizedLoop/HashMapSettings/issues/28) for it to be changed to a reference to the value instead.

2. Having to internally do a `HashMap`'s .get() will most likely be slower than alternatives.

## How to use

This crate relies on the nightly feature [dyn trait upcasting](https://github.com/rust-lang/rust/issues/65991)
that was supposed to be stable in rust 1.76.0, unfortunately it has been [delayed](https://github.com/rust-lang/rust/pull/120233)
so currently the nightly compiler is required.

Add the following line to your Cargo.toml:

```toml
[dependencies]
hashmap_settings = "0.5"
```

Add the following line to your .rs file:

```rust
use hashmap_settings::prelude*;
```

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT LICENSE](LICENSE-MIT) at your option.
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in HashMapSettings by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
