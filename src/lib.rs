//! ## `HashMap` wrapper for layered settings
//!
//! This crate facilitates the management of settings, with the goal of allowing developers to turn previously needlessly set in stone
//! values into a setting a user can change, as well as making it easier for a developer to create multiple priority levels of settings,
//! allowing users to have greater control and customization including across devices.
//!
//! This crate allows a developer to store and access all program settings on a [`Account`](crate::account::Account),
//! a wrapper around a [`HashMap`](std::collections::HashMap).
//!
//! This crate is intended to be used with some sort of type abstraction so that settings of distinct types can be stored in
//! a single `Account`. This crate provides the [`Stg`](crate::stg::Stg) type abstraction for this.
//!
//! An `Account` can also hold other [Accounts](crate::account::Account#accounts), allowing the existence of layered settings,
//! that permit the creation complex systems that have the:
//!
//! ### Benefits
//!
//! 1. Having multiple places changing the same setting with the value being taken from the place that is deemed
//! to have the most importance.
//! (eg: Default, Themes, Extensions, Global Settings, OS Settings, Device Settings, Temporary Settings )
//!
//! 2. Organization of Settings. Given that an `Account` can hold accounts, and they can hold accounts of they own, its possible for
//! small groups of settings to be organized in an `Account`, making it more convenient to locate a setting, or display a group of settings.
//! Important to notice that this organization doesn't need to be (but could be) enforced in all held accounts equally.
//!
//! 3. `Account`s can be individually [deactivated](crate::account::Account#active) allowing for a developer (or a user)
//! to group settings in an `Account` and easily ignore them under certain conditions.
//!
//! ### Drawbacks
//!
//! 1. Each `Account` holds a copy of the settings present in it's child Accounts, so there is a memory cost, but its
//! [planned](https://github.com/OxidizedLoop/HashMapSettings/issues/28) for it to be changed to a reference to the value instead.
//!
//! 2. Having to internally do a [`HashMap`](std::collections::HashMap)'s .get() will most likely be slower than alternatives.
//!
//! ## How to use
//!
//! This crate relies on the nightly feature [dyn trait upcasting](https://github.com/rust-lang/rust/issues/65991)
//! that was supposed to be stable in rust 1.76.0, unfortunately it has been [delayed](https://github.com/rust-lang/rust/pull/120233)
//! so currently the nightly compiler is required.
//!
//! Add the following line to your Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! hashmap_settings = "0.5"
//! ```
//!
//! Add the following line to your .rs file:
//!
//! ```rust
//! # #[allow(warnings)]
//! use hashmap_settings::prelude::*;
//! ```

#![feature(trait_upcasting)]
#![doc(test(attr(deny(warnings))))] //no warnings in tests
/// module containing [`Account`]
pub mod account;
/// module containing the type abstraction [`Stg`]
pub mod stg;
/// HashMap_Settings Prelude
pub mod prelude {
    //! Prelude containing everything that will likely be used while using `Account`
    //!
    //! This includes everything in the crate except the trait [`Incrementable`](crate::account::Incrementable)
    #[doc(inline)]
    pub use crate::account::{Account, DeepError, Valid};
    #[doc(inline)]
    pub use crate::stg::{Setting, Stg, StgError, StgTrait};
}

// inline for docs
#[doc(inline)]
pub use self::{account::Account, stg::Stg};

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        account::Account,
        stg::{Setting, Stg, StgError, StgTrait},
    };

    #[test]
    fn doc_example() -> Result<(), StgError> {
        // # use hashmap_settings::Account;

        //creating a basic account
        let mut account = Account::<(), &str, Stg>::default();

        //inserting values of distinct types
        account.insert("Number of trees", 5.stg());
        account.insert("Grass color", "green".to_string().stg());
        account.insert("Today is good", true.stg());

        //getting values from the account
        let today_bool: bool = account.get(&"Today is good").unstg()?;
        let grass_color: String = account.get(&"Grass color").unstg_panic();
        let trees: i32 = account.get(&"Number of trees").unstg()?;

        //example of using the values
        print!(
            "It's {today_bool} that today is a wonderful day,
    the grass is {grass_color} and I can see {trees} trees in the distance"
        );
        Ok(())
    }
    #[test]
    fn account_test() {
        let bool_setting = true;
        let i32_setting = 42;
        let mut account = Account::<(), String, Stg>::default();
        account.insert("bool_setting".to_string(), bool_setting.stg());
        account.insert("i32_setting".to_string(), i32_setting.stg());
        let i32s: i32 = account
            .get(&"i32_setting".to_string())
            .unwrap()
            .clone()
            .unstg_panic();
        assert_eq!(i32s, 42);
        let stg: Stg = account.get(&"bool_setting".to_string()).unwrap().clone();
        assert!(stg.unstg_panic::<bool>());
    }
    #[test]
    fn partialeq_test() {
        assert!(true.stg() == true.stg());
    }
    #[test]
    const fn setting_example() {
        use crate::stg::Setting;

        #[cfg(feature = "serde")]
        use serde::{Deserialize, Serialize};

        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Debug, PartialEq)]
        pub struct MyType {}

        #[cfg_attr(feature = "serde", typetag::serde)]
        // add #[typetag::serde] if serde feature is activated
        impl Setting for MyType {}
    }
    #[test]
    fn account_new() {
        let mut account1 = Account::new(
            "name".to_string(),
            Default::default(),
            HashMap::default(),
            Vec::default(),
        );
        account1.insert("answer to everything", 42.stg());
        account1.insert("true is true", true.stg());
        let account2 = Account::new(
            "name".to_string(),
            Default::default(),
            [
                ("answer to everything", 42.stg()),
                ("true is true", true.stg()),
            ]
            .into(),
            Vec::default(),
        );
        assert!(account1 == account2);
    }
}
