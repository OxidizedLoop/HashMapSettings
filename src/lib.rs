//! `HashMap` wrapper for layered Settings of distinct types.
//!
//! This crate allows a developer to store and access all program settings on a [`Account`](crate::account::Account) struct,
//! a wrapper around a [`HashMap`](std::collections::HashMap) that can hold any type that implements [`Setting`](crate::setting::Setting).
//!```
//!# use hashmap_settings::stg::Setting;
//!# //use serde::{Deserialize, Serialize};
//!# #[derive(Clone, Debug, PartialEq)] //, Deserialize, Serialize
//!# pub struct MyType{}
//! // add #[typetag::serde] if serde feature is activated
//!impl Setting for MyType{}
//! ```
//!  
//! An Account can also hold other [Accounts](crate::account::Account#accounts), allowing the existence of layered settings.
//!
//! This makes it possible to create complex systems where multiple places
//! (eg: Themes, Extensions, Global User Settings, Local User Settings)
//! are changing the same settings, and the value is taken from the top layer containing the setting
//! or the default layer if no other layer contained it.
//!
//! This crate gives the tools necessary for a developer to create layered settings.
//! This allows users of the application to not only have different settings for different environments,
//! but also have groups of settings that they can easily swap.
//!
//! ## How to use
//!
//! Add the following line to your Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! hashmap_settings = "0.4"
//! ```
//!
//! Add the following line to your .rs file:
//!
//! ```rust
//! # #[allow(warnings)]
//! use hashmap_settings::account::Account;
//! ```
//!
//! Basic use of an `Account` without layers:
//!
//! ```rust
//! /*
//! # use hashmap_settings::account::Account;
//! //creating a basic account
//! let mut account = Account::<(),&str>::default();
//!
//! //inserting values of distinct types
//! account.insert("Number of trees",5);
//! account.insert("Grass color","green".to_string());
//! account.insert("Today is good",true);
//!
//! //getting values from the account
//! let today_bool: bool = account.get(&"Today is good").unwrap();
//! let grass_color: String = account.get(&"Grass color").unwrap();
//! let trees: i32 = account.get(&"Number of trees").unwrap();
//!
//! //example of using the values
//! print!("It's {today_bool} that today is a wonderful day,
//!     the grass is {grass_color} and I can see {trees} trees in the distance");
//! */
//! ```

#![doc(test(attr(deny(warnings))))] //no warnings in tests
///module containing `Account`
pub mod account;
///module containing the Stg type
pub mod stg;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        account::Account,
        stg::{Setting, Stg, StgTrait,StgError},
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
