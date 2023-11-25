//! HashMap wrapper for layered Settings
//!
//! This crate allows you to store and access all your program settings by calling a single Account struct regardless of the type that those settings implement.
//!
//! This crate gives the tools necessary for a developer to create layered settings. This allows users of the application to not only have different settings for different environments, but also have groups of settings that they can easily swap.
//!  ```
//! # // todo!() add examples
//! ```
#![warn(missing_docs)]
#![doc(test(attr(deny(warnings))))]
#![feature(trait_upcasting)]
use core::fmt::Debug;
use dyn_clone::DynClone;
use dyn_ord::DynEq;
use serde::{Deserialize, Serialize};
use std::{
    any::Any,
    collections::{hash_map, HashMap, HashSet},
    option::Option,
};
/// module containing types used internally by the crate
pub mod types;
use types::errors::*;

/// A [`HashMap`]<[`String`],[`Box`]<dyn [`Setting`]>> with an associated name.
/// 
/// An Account is a Wrapper around a [`HashMap`] that can hold any type that implements [`Setting`].
/// 
/// An Account can also hold other [Accounts](Account#accounts). This allows for complex systems where 
/// an app can have multiple layers of settings. The top most layer being the first one to be searched
/// for a specific setting, and in the case it isn't found the next layer will be search, this will be
/// done until the setting is found on the last layer that would be the default layer containing all the settings.
/// 
/// 
/// 
/// An `Account` contains the following fields:
/// 
///  - [name](Account#name): [`String`],
/// 
///  - [active](Account#active): [`bool`],
/// 
///  - [settings](Account#settings): [`HashMap`]<[`String`],[`Box`]<dyn [`Setting`]>>,
/// 
///  - [accounts](Account#accounts): [`Vec`]<`Account`>,
///
/// 
/// # New Account
///
/// Currently a new Account can be created with:
///  - [new](Account::new): Create a new Account.
/// 
///  - [new_valid](Account::new_valid): Create a new Account that is guaranteed to be [valid](Account#valid).
/// 
///  - [clone][Clone::clone]: Clone an existing Account.
/// 
/// An `AccountBuilder` is planned to be created in the [future](https://github.com/OxidizedLoop/HashMapSettings/issues/20).
/// 
/// It's recommend that parent `Accounts` are made with [new_valid](Account::new_valid) but child 
/// `Accounts` are made with with [new](Account::new) to avoid repeated validity checks.
/// 
/// 
/// # [Name](Account#name)
/// 
/// 
/// An `Account's` name is used to identify an Account in multiple methods involving [child](Account#accounts) `Accounts`. 
/// For this reason child `Accounts` need to be uniquely named to be [valid](Account#valid).
/// 
///  - [name](Account::name): Get an account's name 
/// 
///  - [rename](Account::rename): Rename an `Account`
/// 
///  - [deep_rename](Account::deep_rename): Rename a [child](Account#accounts)  `Accounts`
/// 
/// 
/// # [Active](Account#active)
/// 
/// 
/// If a child `Account` is inactive it will be ignore by certain methods like [get()](Account::get) 
/// when this are called on the parent `Account`.
/// 
///  - [active](Account::active): Get an account's activity state
/// 
///  - [change_activity](Account::change_activity): Change the activity 
/// 
///  - [deep_change_activity](Account::deep_change_activity): Change the activity of one of the child `Accounts` 
/// 
/// 
/// # [Settings](Account#settings)
/// 
/// 
/// A `HashMap` holding [Settings](Setting). Contains all the settings present in the
/// [child](Account#accounts) Accounts but can contain settings that aren't in them.
/// 
///  - [accounts()](Account::accounts) Get an account's child `Accounts` 
/// 
///  - [get](Account::get): Returns a reference to the value corresponding to the key
/// 
///  - [deep_get](Account::deep_get): Returns a reference to the value corresponding to the key on a child Account
/// 
///  - [insert](Account::insert): Inserts a key-value pair into the map.
/// 
///  - [deep_insert](Account::deep_insert):Inserts a key-value pair into the map of a child Account
/// 
///  - [keys](Account::keys): An iterator visiting all keys in arbitrary order 
/// 
///  - [contains_key](Account::contains_key): Returns `true` if the `Account` contains a value for the specified key
/// 
///  - [capacity](Account::capacity): Returns the number of elements the map can hold without reallocating.
/// 
/// 
/// # [Accounts](Account#accounts)
/// 
/// 
/// A `Vec` of Accounts. The Account that holds the `Vec` is the parent Account and the Accounts that are being held
/// are the child Accounts.
/// 
///  - [accounts()](Account::accounts): Get an Account's child `Accounts`
/// 
///  - [len](Account::len): Returns the number of elements in the `Vec`.
/// 
///  - [is_empty](Account::is_empty): Returns `true` if the `Vec` contains no elements.
/// 
///  - [push](Account::push): Appends an `Account` to the back of the `Vec`.
/// 
///  - [pop](Account::pop): Removes the last element from a vector and returns it, or [`None`] if it is empty.
/// 
///  - [pop_remove](Account::pop_remove): [pop](Account::pop) but also removes the settings from the main account. Cache
/// 
/// 
/// # [Valid](Account#valid)
///  
/// A valid `Account` is one where it's methods will always behave as intended. 
/// 
/// There are certain methods that may make an Account invalid if improperly used, 
/// and that would make other methods have unindent effects.
/// 
/// If a method can make an `Account` invalid it will be mentioned.
/// 
/// ## Validity Defined:
/// For an `Account` to be valid it needs to follow the following requirements:
/// 
///  - It's child `Accounts` are valid.
/// 
///  - It's child `Accounts` have unique names.
/// 
/// It's NOT yet implemented but it's intended that the following are also true:
/// 
///  - The `Account` contains all settings in the child `Accounts`.
/// 
/// 
/// # [Deep Functions](Account#deep-functions)
///
/// 
/// Deep Functions are versions of functions to interact with a child `Account` 
/// of the parent `Account` that the function is called.
///
/// They accept an extra `Vec` of `&str` that are the list of child `Accounts` 
/// you have to pass though to get to the child `Account` the function will be called.
///
/// 
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Account {
    name: String,
    active: bool,
    settings: HashMap<String, Box<dyn Setting>>,
    accounts: Vec<Account>,
}
impl Account {
    /// Creates a new account
    ///
    /// The is no [validity](Account#valid) check, so the account created can be an invalid account.
    /// Use [`new_valid`](Account::new_valid) to make sure that the account created is valid.
    ///
    /// It's recommend that the parent `Accounts` are made with [`new_valid`](Account::new_valid)
    /// but child `Accounts` are made with with `new` to avoid repeated validity checks.
    /// 
    /// # Example
    /// ```
    /// use std::collections::HashMap;
    /// use hashmap_settings::{Account,Setting};
    /// let account : Account = Account::new(
    ///     "New Account",
    ///     true,
    ///     HashMap::from([
    ///         ("int".to_string(),42.stg()),
    ///         ("bool".to_string(),true.stg())
    ///     ]),
    ///     vec![
    ///         Account::new("1", true, Default::default(), Default::default()),
    ///         Account::new("2", true, Default::default(), Default::default()),
    ///         Account::new("3", true, Default::default(), Default::default())
    ///     ],
    /// );
    ///
    /// assert_eq!(account.name(), "New Account");
    /// assert!(account.active());
    /// assert!(account.settings() ==
    ///     &HashMap::from([
    ///         ("int".to_string(),42.stg()),
    ///         ("bool".to_string(),true.stg())
    ///     ])
    /// );
    /// assert!(account.accounts() ==
    ///     &vec![
    ///         Account::new("1", true, Default::default(), Default::default()),
    ///         Account::new("2", true, Default::default(), Default::default()),
    ///         Account::new("3", true, Default::default(), Default::default())
    ///     ],
    /// );
    ///
    /// ```
    pub fn new(
        name: &str,
        active: bool,
        settings: HashMap<String, Box<dyn Setting>>,
        accounts: Vec<Account>,
    ) -> Self {
        Account {
            name: name.to_string(),
            active,
            settings,
            accounts,
        }
    }
    /// Creates a new [valid](Account#valid) account
    ///
    /// This lets you create an `Account` that is sure to be fully valid
    /// including it's child `Accounts` or an error is returned.
    ///
    /// It's recommend that parent `Accounts` are made with `new_valid` but child 
    /// `Accounts` are made with with [new](Account::new) to avoid repeated validity checks.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::Account;
    /// let account = Account::new_valid(
    ///     "New Account",
    ///     Default::default(),
    ///     Default::default(),
    ///     vec![
    ///         Account::new("1", true, Default::default(), Default::default()),
    ///         Account::new("2", true, Default::default(), Default::default()),
    ///         Account::new("3", true, Default::default(), Default::default())
    ///     ],
    /// );
    /// assert_eq!(account, Ok(Account::new(
    ///     "New Account",
    ///     Default::default(),
    ///     Default::default(),
    ///     vec![
    ///         Account::new("1", true, Default::default(), Default::default()),
    ///         Account::new("2", true, Default::default(), Default::default()),
    ///         Account::new("3", true, Default::default(), Default::default())
    ///     ],
    /// )));
    /// ```
    ///
    /// # Errors
    ///
    /// ```
    /// use hashmap_settings::types::errors::InvalidAccountError;
    /// use hashmap_settings::Account;
    /// let account = Account::new_valid(
    ///     "New Account",
    ///     Default::default(),
    ///     Default::default(),
    ///     vec![
    ///         Account::new("1", true, Default::default(), Default::default()),
    ///         Account::new("1", true, Default::default(), Default::default()),
    ///         Account::new("1", true, Default::default(), Default::default())
    ///     ],
    /// );
    /// assert_eq!(account, Err(InvalidAccountError::ExistingName));
    /// ```
    pub fn new_valid(
        name: &str,
        active: bool,
        settings: HashMap<String, Box<dyn Setting>>,
        accounts: Vec<Account>,
    ) -> Result<Self, InvalidAccountError> {
        let new_account = Account {
            name: name.to_string(),
            active,
            settings,
            accounts,
        };
        if let Some(error) = new_account.is_invalid() {
            Err(error)
        } else {
            Ok(new_account)
        }
    }
    fn is_invalid(&self) -> Option<InvalidAccountError> {
        let accounts = self.accounts_names();
        let size = accounts.len();
        let mut hash_set = HashSet::with_capacity(size);
        for account in accounts {
            if !hash_set.insert(account) {
                return Some(InvalidAccountError::ExistingName);
            }
        }
        drop(hash_set); // dropping map here as it isn't needed anymore and being a recursive function the memory usage would keep increasing.
                        //todo!() check if it's dropped automatically by the compiler.
        for account in self.accounts().iter() {
            if let Some(error) = account.is_invalid() {
                return Some(error);
            }
        }
        None
    }
    /// Returns the name of the `Account`
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::{Account};
    /// let account : Account = Account::new(
    ///     "New account",
    ///     Default::default(),
    ///     Default::default(),
    ///     Default::default()
    /// );
    ///
    /// assert_eq!(account.name(), "New account");
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Return a reference to the `HashMap`
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::{Account,Setting};
    /// use std::collections::HashMap;
    /// let account : Account = Account::new(
    ///     "New Account",
    ///     Default::default(),
    ///     HashMap::from([
    ///         ("int".to_string(),42.stg()),
    ///         ("bool".to_string(),true.stg())
    ///     ]),
    ///     Default::default(),
    /// );
    ///
    /// assert!(account.settings() ==
    ///     &HashMap::from([
    ///         ("int".to_string(),42.stg()),
    ///         ("bool".to_string(),true.stg())
    ///     ])
    /// );
    ///
    /// ```
    pub fn settings(&self) -> &HashMap<String, Box<dyn Setting>> {
        &self.settings
    }
    /// Return a reference to the `Vec` of child `Accounts`
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::{Account};
    /// let account : Account = Account::new(
    ///     "New Account",
    ///     Default::default(),
    ///     Default::default(),
    ///     vec![
    ///         Account::new("1", true, Default::default(), Default::default()),
    ///         Account::new("2", true, Default::default(), Default::default()),
    ///         Account::new("3", true, Default::default(), Default::default())
    ///     ],
    /// );
    ///
    /// assert!(account.accounts() ==
    ///     &vec![
    ///         Account::new("1", true, Default::default(), Default::default()),
    ///         Account::new("2", true, Default::default(), Default::default()),
    ///         Account::new("3", true, Default::default(), Default::default())
    ///     ],
    /// );
    ///
    /// ```
    pub fn accounts(&self) -> &Vec<Account> {
        &self.accounts
    }
    /// Return `true` if the `Account` is active
    ///
    /// When not active `Accounts` will be treated as if they were not there when called by some of the parent's `Account` methods.
    ///
    /// When creating an `Account` with [`Default`] active will be `true`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::{Account};
    /// let mut account : Account = Account::new("New Account", true, Default::default(), Default::default());
    ///
    /// assert!(account.active());
    /// account.change_activity(false);
    /// assert!(!account.active());
    ///
    /// ```
    pub fn active(&self) -> bool {
        self.active
    }
    /// Takes a `bool` and changes the value of active, returns `true` if changes were made.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::{Account};
    /// let mut account : Account = Account::new("New Account", false, Default::default(), Default::default());
    ///
    /// assert!(!account.active());
    /// assert_eq!(account.change_activity(true), true);
    /// assert!(account.active());
    /// assert_eq!(account.change_activity(true), false);
    /// assert!(account.active());
    ///
    /// ```
    pub fn change_activity(&mut self, new_active: bool) -> bool {
        if self.active() == new_active {
            false
        } else {
            self.active = new_active;
            true
        }
    }
    /// Takes a `bool` and changes the value of active of a child `Account`.
    /// 
    /// Part of the deep_functions group that accept a `Vec` of &str to identify
    /// the child `Account` to run the function. [`change_activity`](Account::change_activity) in this case.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::{Account};
    /// let mut account = Account::new(
    ///     "Old Name",
    ///     Default::default(),
    ///     Default::default(),
    ///     vec![
    ///         Account::new("1", true, Default::default(), Default::default()),
    ///         Account::new("2", true, Default::default(), Default::default()),
    ///         Account::new("3", true, Default::default(), vec![
    ///             Account::new("3_1", true, Default::default(), Default::default()),
    ///             Account::new("3_2", true, Default::default(), Default::default()),
    ///             Account::new("3_3", true, Default::default(), Default::default())
    ///         ])
    ///     ],
    /// );
    /// 
    /// assert_eq!(account.deep_change_activity(false,&mut vec!["3_2","3"]), Ok(true));
    /// assert_eq!(account, Account::new(
    ///     "Old Name",
    ///     Default::default(),
    ///     Default::default(),
    ///     vec![
    ///         Account::new("1", true, Default::default(), Default::default()),
    ///         Account::new("2", true, Default::default(), Default::default()),
    ///         Account::new("3", true, Default::default(), vec![
    ///             Account::new("3_1", true, Default::default(), Default::default()),
    ///             Account::new("3_2", false, Default::default(), Default::default()),
    ///             Account::new("3_3", true, Default::default(), Default::default())
    ///         ])
    ///     ],
    /// ));
    /// ```
    pub fn deep_change_activity(
        &mut self,
        new_active: bool,
        account_names: &mut Vec<&str>, //for each value, the value to its right is its parent.
        //left is the account we rename, right is the first child of the Account we call
    ) ->Result<bool,DeepChangeError> {
        let account_to_find = if let Some(account_name) = account_names.pop() {
            account_name
        } else {
            return Err(DeepChangeError::EmptyVec); //error if the original call is empty, but this will create the base case in the recursive call
        };
        if let Some(found_account) = self.mut_account_from_name(account_to_find) {
            match found_account.deep_change_activity(new_active, account_names) {
                //recursive call
                Err(error) => match error {
                    DeepChangeError::EmptyVec => {
                        Ok(found_account.change_activity(new_active))
                    } //base case
                    _ => Err(error), //error, impossible/invalid function call
                },
                Ok(value) => Ok(value),
            }
        } else {
            Err(DeepChangeError::NotFound)
        }
    }
    /// Takes a `&str` and updates the name of the `Account`.
    ///
    /// Returns the previous name that the Account had.
    /// 
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::{Account};
    /// let mut account = Account::new(
    ///     "Old Name",
    ///     Default::default(),
    ///     Default::default(),
    ///     Default::default()
    /// );
    /// assert_eq!(account.name(), "Old Name");
    /// assert_eq!(account.rename("New Name"), "Old Name".to_string());
    /// assert_eq!(account.name(), "New Name");
    /// ```
    pub fn rename(&mut self, new_name: &str) -> String {
        let r_value = self.name.clone();
        self.name = new_name.to_string();
        r_value
        
    }
    /// Takes a `&str` and updates the name of a child `Account`.
    /// 
    /// Part of the deep_functions group that accept a `Vec` of &str to identify
    /// the child `Account` to run the function. [`rename`](Account::rename) in this case.
    ///
    /// This can make a Account [invalid](Account#valid) if the child Account 
    /// got renamed to the same name as one of it's siblings.
    /// 
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::{Account};
    /// let mut account = Account::new(
    ///     "Old Name",
    ///     Default::default(),
    ///     Default::default(),
    ///     vec![
    ///         Account::new("1", true, Default::default(), Default::default()),
    ///         Account::new("2", true, Default::default(), Default::default()),
    ///         Account::new("3", true, Default::default(), vec![
    ///             Account::new("3_1", true, Default::default(), Default::default()),
    ///             Account::new("3_2", true, Default::default(), Default::default()),
    ///             Account::new("3_3", true, Default::default(), Default::default())
    ///         ])
    ///     ],
    /// );
    /// 
    /// assert_eq!(account.deep_rename("Cool Name",&mut vec!["3_2","3"]), Ok("3_2".to_string()));
    /// assert_eq!(account, Account::new(
    ///     "Old Name",
    ///     Default::default(),
    ///     Default::default(),
    ///     vec![
    ///         Account::new("1", true, Default::default(), Default::default()),
    ///         Account::new("2", true, Default::default(), Default::default()),
    ///         Account::new("3", true, Default::default(), vec![
    ///             Account::new("3_1", true, Default::default(), Default::default()),
    ///             Account::new("Cool Name", true, Default::default(), Default::default()),
    ///             Account::new("3_3", true, Default::default(), Default::default())
    ///         ])
    ///     ],
    /// ));
    /// ```
    pub fn deep_rename(
        &mut self,
        new_name: &str,
        account_names: &mut Vec<&str>, //for each value, the value to its right is its parent.
        //left is the account we rename, right is the first child of the Account we call
    ) ->Result<String,DeepChangeError> {
        let account_to_find = if let Some(account_name) = account_names.pop() {
            account_name
        } else {
            return Err(DeepChangeError::EmptyVec); //error if the original call is empty, but this will create the base case in the recursive call
        };
        if let Some(found_account) = self.mut_account_from_name(account_to_find) {
            match found_account.deep_rename(new_name, account_names) {
                //recursive call
                Err(error) => match error {
                    DeepChangeError::EmptyVec => {
                        Ok(found_account.rename(new_name))
                    } //base case
                    _ => Err(error), //error, impossible/invalid function call
                },
                Ok(value) => Ok(value),
            }
        } else {
            Err(DeepChangeError::NotFound)
        }
    }
    /// Returns a reference to the value corresponding to the key in a child `Account`.
    /// 
    /// Part of the deep_functions group that accept a `Vec` of &str to identify
    /// the child `Account` to run the function. [`get`](Account::get) in this case.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use hashmap_settings::{Account,Setting};
    /// let account = Account::new(
    ///     "Old Name",
    ///     Default::default(),
    ///     Default::default(),
    ///     vec![
    ///         Account::new("1", true, Default::default(), Default::default()),
    ///         Account::new("2", true, Default::default(), Default::default()),
    ///         Account::new("3", true, Default::default(), vec![
    ///             Account::new("3_1", true, Default::default(), Default::default()),
    ///             Account::new(
    ///                 "3_2", 
    ///                 true, 
    ///                 HashMap::from([
    ///                     ("int".to_string(),42.stg()),
    ///                     ("bool".to_string(),true.stg()),
    ///                     ("char".to_string(),'c'.stg()),
    ///                 ]), 
    ///                 Default::default()),
    ///             Account::new("3_3", true, Default::default(), Default::default()),
    ///         ])
    ///     ],
    /// );
    /// 
    /// assert_eq!(account.deep_get("int",&mut vec!["3_2","3"]), Ok(Some(&42.stg())));
    /// ```
    #[allow(clippy::borrowed_box)]
    pub fn deep_get(
        &self,
        setting_name: &str,
        account_names: &mut Vec<&str>, //for each value, the value to its right is its parent.
        //left is the account we rename, right is the first child of the Account we call
    ) -> Result<Option<&Box<dyn Setting>>, DeepChangeError> {
        let account_to_find = if let Some(account_name) = account_names.pop() {
            account_name
        } else {
            return Err(DeepChangeError::EmptyVec); //error if the original call is empty, but this will create the base case in the recursive call
        };
        if let Some(found_account) = self.account_from_name(account_to_find) {
            match found_account.deep_get(setting_name, account_names) {
                //recursive call
                Err(error) => match error {
                    DeepChangeError::EmptyVec => Ok(found_account.get(setting_name)), //base case
                    _ => Err(error), //error, impossible/invalid function call
                },
                Ok(value) => Ok(value),
            }
        } else {
            Err(DeepChangeError::NotFound)
        }
    }
    fn account_from_name(&self, name: &str) -> Option<&Account> {
        for account in 0..self.len() {
            if self.accounts[account].name() == name {
                return Some(&self.accounts[account]);
            }
        }
        None
    }
    /// Return a `Vec` of names of the child `Accounts`
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::{Account};
    /// let account : Account = Account::new(
    ///     "New Account",
    ///     Default::default(),
    ///     Default::default(),
    ///     vec![
    ///         Account::new("1", true, Default::default(), Default::default()),
    ///         Account::new("2", true, Default::default(), Default::default()),
    ///         Account::new("3", true, Default::default(), Default::default())
    ///     ],
    /// );
    ///
    /// assert!(account.accounts_names() == vec!["1","2","3"]);
    ///
    /// ```
    pub fn accounts_names(&self) -> Vec<&str> {
        self.accounts.iter().map(|a| a.name()).collect()
    }
    /// Inserts a key-value pair into the map of a child `Account`.
    /// 
    /// This will updated the [settings](Account#settings) of all necessary Accounts 
    /// so that the parent Account remains [valid](Account#valid)
    ///
    /// Part of the deep_functions group that accept a `Vec` of &str to identify
    /// the child `Account` to run the function. [`insert`](Account::insert) in this case.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use hashmap_settings::{Account,Setting};
    /// let mut account = Account::new(
    ///     "Old Name",
    ///     Default::default(),
    ///     Default::default(),
    ///     vec![
    ///         Account::new("1", true, Default::default(), Default::default()),
    ///         Account::new("2", true, Default::default(), Default::default()),
    ///         Account::new("3", true, Default::default(), vec![
    ///             Account::new("3_1", true, Default::default(), Default::default()),
    ///             Account::new(
    ///                 "3_2", 
    ///                 true, 
    ///                 HashMap::from([
    ///                     ("int".to_string(),42.stg()),
    ///                     ("bool".to_string(),true.stg()),
    ///                     ("char".to_string(),'c'.stg()),
    ///                 ]), 
    ///                 Default::default()),
    ///             Account::new("3_3", true, Default::default(), Default::default()),
    ///         ])
    ///     ],
    /// );
    /// 
    /// assert_eq!(account.deep_insert("int", 777.stg() ,&mut vec!["3_2","3"]), Ok(Some(42.stg())));
    /// assert_eq!(account.deep_get("int",&mut vec!["3_2","3"]), Ok(Some(&777.stg())));
    /// ```
    pub fn deep_insert(
        &mut self,
        setting_name: &str,
        setting_value: Box<dyn Setting>,
        account_names: &mut Vec<&str>, //for each value, the value to its right is its parent.
        //left is where we insert the value, right is the first child of the Account we call
    ) -> Result<Option<Box<dyn Setting>>, DeepChangeError> {
        let account_to_find = if let Some(account_name) = account_names.pop() {
            account_name
        } else {
            return Err(DeepChangeError::EmptyVec); //error if the original call is empty, but this will create the base case in the recursive call
        };
        if let Some(found_account) = self.mut_account_from_name(account_to_find) {
            match found_account.deep_insert(
                setting_name,
                setting_value.clone(),
                account_names,
            ) {
                //recursive call
                Ok(insert_option) => {
                    self.update_setting(setting_name);
                    //after the base this will be called in all previous function calls,
                    //updating the value in the corresponding Account.settings
                    Ok(insert_option) //returning the original value from the base case
                }
                Err(error) => match error {
                    DeepChangeError::EmptyVec => {
                        Ok(found_account.insert(setting_name, setting_value))
                    } //base case
                    _ => Err(error), //error, impossible/invalid function call
                },
            }
        } else {
            Err(DeepChangeError::NotFound)
        }
    }
    /// Updates a setting with the value its supposed to have.
    /// 
    /// 
    /// 
    /// Returns `None` if the setting isn't present in the Account or child Accounts.
    /// Returns true if the value of the setting was updated.
    /// 
    /// If an Account is [valid](Account#valid) this method never returns Some(true) 
    /// as this method is used to turn an invalid Account into a valid one.
    /// 
    /// # Examples
    /// ```
    ///  //todo!() add example
    /// ```
    pub fn update_setting(&mut self, setting: &str) -> Option<bool> {
        for account in (0..self.len()).rev() {
            if self.accounts[account].active() {
                if let Some(value) = self.accounts[account].get(setting) {
                    let temp = value.clone(); //to prevent cannot borrow `self.accounts` as mutable because it is also borrowed as immutable Error
                    return Some(!(self.insert(setting, temp.clone()) == Some(temp)))
                }//todo!()improve this so cloning twice isn't necessary
            }
        }
        if self.settings.contains_key(setting) {
            Some(false)
        } else {
            None
        }
    }
    fn mut_account_from_name(&mut self, name: &str) -> Option<&mut Account> {
        for account in 0..self.len() {
            if self.accounts[account].name() == name {
                return Some(&mut self.accounts[account]);
            }
        }
        None
    }
    /// Appends an `Account` to the back of the `Vec` of child `Accounts`.
    ///
    /// This child `Account` settings will be added to the settings of the main `Account` that `push` was called on.
    ///
    /// The Account will be updated with the new settings unless the inserted child `Account` is [inactive](Account::active).
    ///
    /// Won't return an error if the child `Account` being pushed is [invalid](Account#valid)
    /// but will cause unintended behavior for future calls to the main `Account`.
    /// Use [push](Account::push) if the Account might be [invalid](Account#valid).
    /// 
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::{Account};
    /// let mut account : Account = Account::new(
    ///     Default::default(),
    ///     Default::default(),
    ///     Default::default(),
    ///     vec![
    ///         Account::new("1", Default::default(), Default::default(), Default::default()),
    ///         Account::new("2", Default::default(), Default::default(), Default::default())
    ///     ],
    /// );
    /// account.push_unchecked(Account::new("3", Default::default(), Default::default(), Default::default()));
    /// assert!(account ==
    ///     Account::new(
    ///         Default::default(),
    ///         Default::default(),
    ///         Default::default(),
    ///         vec![
    ///             Account::new("1", Default::default(), Default::default(), Default::default()),
    ///             Account::new("2", Default::default(), Default::default(), Default::default()),
    ///             Account::new("3", Default::default(), Default::default(), Default::default())
    ///         ],
    ///     )
    /// )
    /// ```
    pub fn push_unchecked(&mut self, account: Account) {
        if account.active {
            for setting in account.settings.keys() {
                self.insert(setting, account.get(setting).unwrap().clone());
            }   
        }
        self.accounts.push(account);
    }
    /// Returns `true` if the `Account` contains a value for the specified key.
    ///
    /// The key may be any borrowed form of the map’s key type, but [`Hash`] and [`Eq`] on the borrowed form must match those for the key type.
    ///
    /// This method is a direct call to [`HashMap`]'s [`contains_key()`](HashMap::contains_key()) .
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::{Account,Setting};
    /// let mut account : Account = Default::default();
    /// account.insert("a small number", 42.stg());
    /// assert_eq!(account.contains_key("a small number"), true);
    /// assert_eq!(account.contains_key("a big number"), false);
    /// ```
    pub fn contains_key(&self, setting_name: &str) -> bool {
        self.settings.contains_key(setting_name)
    }
    /// Returns a reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// This method is a direct call to [`HashMap`]'s [`get()`](HashMap::get()).
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::{Account,Setting};
    /// let mut account : Account = Default::default();
    /// account.insert("a small number", 42.stg());
    /// assert_eq!(account.get("a small number"), Some(&42.stg()));
    /// assert_eq!(account.get("a big number"), None);
    /// ```
    #[allow(clippy::borrowed_box)]
    pub fn get(&self, setting_name: &str) -> Option<&Box<dyn Setting>> {
        self.settings.get(setting_name)
    }
    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, None is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned. The key is not updated, though; this matters for
    /// types that can be `==` without being identical. See the [module-level
    /// documentation] for more.
    ///
    /// [module-level documentation]: std::collections#insert-and-complex-keys
    ///
    /// This method is a direct call to [`HashMap`]'s [`insert()`](HashMap::insert()).
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::{Account,Setting};
    /// let mut account : Account = Default::default();
    /// assert_eq!(account.insert("a small number", 1.stg()), None);
    /// assert_eq!(account.settings().is_empty(), false);
    ///
    /// account.insert("a small number", 2.stg());
    /// assert_eq!(account.insert("a small number", 3.stg()), Some(2.stg()));
    /// assert!(account.settings()[&"a small number".to_string()] == 3.stg());
    /// ```
    pub fn insert(
        &mut self,
        setting_name: &str,
        setting_value: Box<dyn Setting>,
    ) -> Option<Box<dyn Setting>> {
        let mut return_value = None;
        if let Some(value) = self.settings.insert(setting_name.to_string(), setting_value.clone()) {
            return_value = Some(value);
        }
        return_value
    }
    /// An iterator visiting all keys in arbitrary order.
    /// The iterator element type is `&'a String`.
    ///
    /// This method is a direct call to [`HashMap`]'s [`keys()`](HashMap::keys()).
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::{Account,Setting};
    /// use std::collections::HashMap;
    /// let account: Account = Account::new(
    ///     Default::default(),
    ///     Default::default(),
    ///     HashMap::from([
    ///         ("int".to_string(),42.stg()),
    ///         ("bool".to_string(),true.stg()),
    ///         ("char".to_string(),'c'.stg()),
    ///     ]),
    ///     Default::default(),
    /// );
    ///
    /// for key in account.keys() {
    ///     println!("{key}");
    /// }
    /// ```
    ///
    /// # Performance
    ///
    /// In the current implementation, iterating over keys takes O(capacity) time
    /// instead of O(len) because it internally visits empty buckets too.
    pub fn keys(&self) -> hash_map::Keys<'_, String, Box<dyn Setting>> {
        self.settings.keys()
    }
    /// Returns the number of elements the map can hold without reallocating.
    ///
    /// This number is a lower bound; the `HashMap<String, Box<dyn Setting>>` might be able to hold
    /// more, but is guaranteed to be able to hold at least this many.
    ///
    /// This method is a direct call to [`HashMap`]'s [`keys()`](HashMap::keys()).
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::{Account};
    /// use std::collections::HashMap;
    /// let account : Account = Account::new(Default::default(), Default::default(), HashMap::with_capacity(100), Default::default());
    /// assert!(account.capacity() >= 100);
    /// ```
    pub fn capacity(&self) -> usize {
        self.settings.capacity()
    }
    /// Returns the number of elements in the `Vec` of child `Accounts`,
    /// also referred to as its 'length'.
    ///
    /// This method is a direct call to [`Vec`]'s [`len()`](Vec::len()).
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::Account;
    /// let account = Account::new(
    ///         Default::default(),
    ///         Default::default(),
    ///         Default::default(),
    ///         vec![
    ///             Account::new("1", Default::default(), Default::default(), Default::default()),
    ///             Account::new("2", Default::default(), Default::default(), Default::default()),
    ///             Account::new("3", Default::default(), Default::default(), Default::default())
    ///         ],
    ///     );
    /// assert_eq!(account.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.accounts.len()
    }
    /// Returns `true` if the `Vec` of child `Accounts` contains no elements.
    ///
    /// This method is a direct call to [`Vec`]'s [`is_empty()`](Vec::is_empty()).
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::{Account};
    /// let mut account = Account::default();
    /// assert!(account.is_empty());
    ///
    /// account.push(Account::default());
    /// assert!(!account.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.accounts.is_empty()
    }
    /// Appends an `Account` to the back of the `Vec` of child `Accounts`.
    ///
    /// This child `Account` settings will be added to the settings of the main `Account` that `push` was called on.
    ///
    /// The Account will be updated with the new settings unless the inserted child `Account` is [inactive](Account::active).
    /// 
    /// Will return an error if the child `Account` being pushed is [invalid](Account#valid) or would make the main `Account` invalid.
    /// Use [push_unchecked](Account::push_unchecked) for better performance if its guaranteed that `Account` is valid.
    /// 
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::{Account,types::errors::InvalidAccountError};
    /// let mut account : Account = Account::new(
    ///     Default::default(),
    ///     Default::default(),
    ///     Default::default(),
    ///     vec![
    ///         Account::new("1", Default::default(), Default::default(), Default::default()),
    ///         Account::new("2", Default::default(), Default::default(), Default::default())
    ///     ],
    /// );
    /// account.push(Account::new("3", Default::default(), Default::default(), Default::default()));
    /// assert!(account ==
    ///     Account::new(
    ///         Default::default(),
    ///         Default::default(),
    ///         Default::default(),
    ///         vec![
    ///             Account::new("1", Default::default(), Default::default(), Default::default()),
    ///             Account::new("2", Default::default(), Default::default(), Default::default()),
    ///             Account::new("3", Default::default(), Default::default(), Default::default())
    ///         ],
    ///     )
    /// );
    /// assert!(account.push(Account::new("3", Default::default(), Default::default(), Default::default()))
    ///     == Some(InvalidAccountError::ExistingName));
    /// ```
    pub fn push(&mut self, account: Account) -> Option<InvalidAccountError> {
        if self.accounts_names().contains(&account.name()) {
            //check if account has the same name as a sibling account
            return Some(InvalidAccountError::ExistingName);
        }
        if let Some(error) = account.is_invalid() {
            //check if Account is internally valid
            return Some(error);
        }
        if account.active {
            for setting in account.settings.keys() {
                self.insert(setting, account.get(setting).unwrap().clone());
            }   
        }
        self.accounts.push(account);
        None
    }
    /// Removes the last element from a vector and returns it, or [`None`] if it is empty.
    /// 
    /// Use [pop_remove](Account::pop_remove) if you intend to remove settings from 
    /// the main `Account` present only on the popped child `Account`.
    ///
    /// This method is a direct call to [`Vec`]'s [`pop()`](Vec::pop()).
    /// 
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::{Account};
    /// let mut account : Account = Account::new(
    ///     Default::default(),
    ///     Default::default(),
    ///     Default::default(),
    ///     vec![
    ///         Account::new("1", Default::default(), Default::default(), Default::default()),
    ///         Account::new("2", Default::default(), Default::default(), Default::default()),
    ///         Account::new("3", Default::default(), Default::default(), Default::default())
    ///     ],
    /// );
    /// account.pop_keep();
    /// assert!(account ==
    ///     Account::new(
    ///         Default::default(),
    ///         Default::default(),
    ///         Default::default(),
    ///         vec![
    ///             Account::new("1", Default::default(), Default::default(), Default::default()),
    ///             Account::new("2", Default::default(), Default::default(), Default::default())
    ///         ],
    ///     )
    /// )
    /// ```
    pub fn pop_keep(&mut self) -> std::option::Option<Account> {
        self.accounts.pop()
    }
    /// Removes the last element from a vector and returns it, or [`None`] if it empty.
    ///
    /// Will remove settings from the parent `Account` present only on the popped child `Account`.
    /// Use [pop_keep](Account::pop) if you want the parent `Account` settings to remain unchanged.
    ///
    /// 
    /// This method contains a call to [`Vec`]'s [`pop()`](Vec::pop()).
    /// 
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::{Account};
    /// let mut account : Account = Account::new(
    ///     Default::default(),
    ///     Default::default(),
    ///     Default::default(),
    ///     vec![
    ///         Account::new("1", Default::default(), Default::default(), Default::default()),
    ///         Account::new("2", Default::default(), Default::default(), Default::default()),
    ///         Account::new("3", Default::default(), Default::default(), Default::default())
    ///     ],
    /// );
    /// account.pop();
    /// assert!(account ==
    ///     Account::new(
    ///         Default::default(),
    ///         Default::default(),
    ///         Default::default(),
    ///         vec![
    ///             Account::new("1", Default::default(), Default::default(), Default::default()),
    ///             Account::new("2", Default::default(), Default::default(), Default::default())
    ///         ],
    ///     )
    /// )
    /// ```
    pub fn pop(&mut self) -> std::option::Option<Account> {
        let popped_account = self.accounts.pop()?;
        for setting in popped_account.keys() {
            if !self.vec_contains_key(setting) {
                self.settings.remove(setting);
            }
        }
        Some(popped_account)
    }
    fn vec_contains_key(&self, setting: &str) -> bool {
        for account in self.accounts() {
            if account.contains_key(setting) {
                return true
            }
        }
        false
    }
    ///todo!()
    pub fn get_mut_account(&mut self, index: usize) -> Option<&mut Account> {
        self.accounts.get_mut(index)
    }
    /*
        unused functions
        pub fn all_names(&self) -> Vec<&str> { //what would be the use
            let mut r_value = vec![self.name()];
            self.accounts
                .iter()
                .map(|a| a.all_names())
                .for_each(|a| r_value.extend(a));
            r_value
        }
        fn accounts_mut(&mut self) -> &mut Vec<Account> {
            &mut self.accounts
        }
    */
}
impl Default for Account {
    fn default() -> Self {
        Self {
            name: Default::default(),
            settings: Default::default(),
            accounts: Default::default(),
            active: true,
        }
    }
}

/// Required trait for any type that that will be used as a setting
#[typetag::serde(tag = "setting")]
pub trait Setting: Any + Debug + DynClone + DynEq {
    ///turns a type implementing [Setting] into a [Box<dyn Setting>]
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::{stg,Setting};
    /// let bool = true;
    /// let bool_stg: Box<dyn Setting> = bool.stg();
    /// assert!(bool_stg == stg(bool))
    /// ```
    fn stg(self) -> Box<dyn Setting>
    where
        Self: Setting + Sized
    {
        Box::new(self)
    }
}
dyn_clone::clone_trait_object!(Setting);
impl PartialEq for Box<dyn Setting> {
    fn eq(&self, other: &Self) -> bool {
        let x: Box<dyn DynEq> = self.clone();
        let y: Box<dyn DynEq> = other.clone();
        x == y
    }
}

///turns a type implementing [`Setting`] into a [`Box<dyn Setting>`]
///
/// # Examples
///
/// ```
/// use hashmap_settings::{stg,Setting};
/// let bool = true;
/// let bool_stg: Box<dyn Setting> = stg(bool);
/// assert!(bool_stg == bool.stg())
/// ```
pub fn stg<T: Setting>(value: T) -> Box<dyn Setting> {
    value.stg()
}
///turns a [`Box<dyn Setting>`] into a type implementing [`Setting`],can [`panic!`]
///
/// # Examples
///
/// ```
/// use hashmap_settings::{Setting,stg,unstg};
///
/// let bool_stg: Box<dyn Setting> = stg(true);
/// assert_eq!(unstg::<bool>(bool_stg), true);
/// //we need to use ::<bool> to specify that want to turn bool_stg into a bool
/// ```
/// ```
/// use hashmap_settings::{Setting,stg,unstg};
///
/// let bool_stg: Box<dyn Setting> = stg(true);
/// let bool :bool = unstg(bool_stg);
/// // here we don't as we specific the type annotation when we use :bool
/// assert_eq!(bool, true);
/// ```
/// We need to be careful using .unstg as if we try convert to the wrong type the program will panic.
/// Consider using [`safe_unstg`] as it returns a result type instead.
/// ```should_panic
/// use hashmap_settings::{Setting,stg,unstg};
///
/// let bool_stg: Box<dyn Setting> = stg(true);
/// let _number :i32 = unstg(bool_stg);
/// // this panics, as the Box<dyn Setting> holds a bool value but we are trying to convert it to a i32
///
/// ```
pub fn unstg<T: Setting>(stg: Box<dyn Setting>) -> T {
    let x: Box<dyn Any> = stg;
    *x.downcast().unwrap()
}
///turns a [`Box<dyn Setting>`] into a [`Result`] type implementing [`Setting`]
///
/// ```
/// # // todo!() add examples
/// ```
pub fn safe_unstg<T: Setting>(stg: Box<dyn Setting>) -> Result<Box<T>, Box<dyn Any>> {
    let x: Box<dyn Any> = stg;
    x.downcast()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn account_test() {
        let bool_setting = true;
        let i32_setting = 42;
        let mut account = Account::default();
        account.insert("bool_setting", Box::new(bool_setting));
        account.insert("i32_setting", i32_setting.stg());
        let i32s: i32 = unstg(account.get("i32_setting").unwrap().clone());
        assert_eq!(i32s, 42);
        let stg: Box<dyn Setting> = account.get("bool_setting").unwrap().clone();
        assert!(unstg::<bool>(stg));
    }
    #[test]
    fn partialeq_test() {
        assert!(true.stg() == true.stg());
    }
    #[test]
    fn account_new() {
        let mut account1 = Account::new(
            "name",
            Default::default(),
            Default::default(),
            Default::default(),
        );
        account1.insert("answer to everything", 42.stg());
        account1.insert("true is true", true.stg());
        let account2 = Account::new(
            "name",
            Default::default(),
            [
                ("answer to everything".to_string(), 42.stg()),
                ("true is true".to_string(), true.stg()),
            ]
            .into(),
            Default::default(),
        );
        assert!(account1 == account2);
    }
}
