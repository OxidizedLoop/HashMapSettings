# HashMapSettings [![Version]][Crates.io] [![Rust Version]][Rust 1.76] [![Documentation]][Docs.rs] [![Build Status]][Actions]

[Version]: https://img.shields.io/crates/v/hashmap_settings.svg
[Crates.io]: https://crates.io/crates/hashmap_settings
[Documentation]: https://img.shields.io/docsrs/hashmap_settings/latest
[Docs.rs]: https://docs.rs/hashmap_settings
[Build Status]: https://img.shields.io/github/actions/workflow/status/OxidizedLoop/HashMapSettings/rust.yml
[Actions]: https://github.com/OxidizedLoop/HashMapSettings/actions
[Rust Version]: https://img.shields.io/badge/rust-1.76+-lightgray.svg
[Rust 1.76]: https://blog.rust-lang.org/2024/02/08/Rust-1.76.0.html

## **A HashMap wrapper for layered Settings of distinct types**

This crate allows a developer to store and access all program settings on a `Account` struct, a wrapper around a `HashMap` that can hold any type that implements `Setting`.

An `Account` can also hold other Accounts, allowing the existence of layered settings.

This makes it possible to create complex systems where multiple places (eg: Themes, Extensions, Global User Settings, Local User Settings) are changing the same settings, and the value is taken from the top layer containing the setting or the default layer if no other layer contained it.

## How to use

Add the following line to your Cargo.toml:

```toml
[dependencies]
hashmap_settings = "0.4"
```

Add the following line to your .rs file:

```rust
use hashmap_settings::{Account,Setting,unstg,safe_unstg};
```

In the [future](https://github.com/OxidizedLoop/HashMapSettings/issues/1) you will be able to derive Setting, but for now you can implement it by adding the following lines:

```rust
#[typetag::serde] //if serde feature is activated
impl Setting for MyType {}
```

Basic use of an `Account`:

```rust
let mut account = Account::default(); //creating a basic account

//inserting values of distinct types
account.insert("Number of trees",5);
account.insert("Grass color","green".to_string());
account.insert("Today is good",true);

//getting values from the account (check issue #27)
let today_bool: bool = unstg(account.get("Today is good").unwrap().clone());
let grass_color: String = unstg(account.get("Grass color").unwrap().clone());
let trees: i32 = unstg(account.get("Number of trees").unwrap().clone());

//be careful as this would panic!:
//let grass: i32 = unstg(account.get("Grass Color").unwrap().clone());
//there is a safe_unstg() returning a Result that can be used to prevent mistakes.

//example of using the values 
print!("It's {today_bool} that today is a wonderful day, the grass is {grass_color} and I can see {trees} trees in the distance");
```

(At the moment getting values of an account isn't user friendly but it will be changed in the [future](https://github.com/OxidizedLoop/HashMapSettings/issues/27))

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT LICENSE](LICENSE-MIT) at your option.
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in HashMapSettings by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
