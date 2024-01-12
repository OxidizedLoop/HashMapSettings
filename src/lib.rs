//! `HashMap` wrapper for layered Settings of distinct types.
//!
//! This crate allows a developer to store and access all program settings on a [`Account`] struct,
//! a wrapper around a [`HashMap`] that can hold any type that implements [`Setting`].
//!```
//!# use hashmap_settings::Setting;
//!# //use serde::{Deserialize, Serialize};
//!# #[derive(Clone, Debug, PartialEq)] //, Deserialize, Serialize
//!# pub struct MyType{}
//! // add #[typetag::serde] if serde feature is activated
//!impl Setting for MyType{}
//! ```
//!  
//! An Account can also hold other [Accounts](Account#accounts), allowing the existence of layered settings.
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
//! ```
//! # #[allow(warnings)]
//! use hashmap_settings::{Account};
//! ```
//!
//! Basic use of an `Account` without layers:
//!
//! ```rust
//! /*
//! # use hashmap_settings::{Account};
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

use core::fmt::Debug;
use dyn_clone::DynClone;
use dyn_ord::DynEq;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    any::Any,
    collections::{hash_map, HashMap, HashSet},
    hash::Hash,
    option::Option,
};
/// module containing types used internally by the crate
pub mod types;
use types::errors::{DeepError, InvalidAccountError, StgError};

/// A [`HashMap`] with an associated name permitting layered settings.
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
///  - [`new`](Account::new): Create a new Account.
///
///  - [`new_valid`](Account::new_valid): Create a new Account that is guaranteed to be [valid](Account#valid).
///
///  - [`clone`][Clone::clone]: Clone an existing Account.
///
/// An `AccountBuilder` is planned to be created in the [future](https://github.com/OxidizedLoop/HashMapSettings/issues/20).
///
/// It's recommend that parent `Accounts` are made with [new_valid](Account::new_valid) but
/// [child Accounts](Accounts#accounts) are made with with [new](Account::new) to avoid repeated validity checks.
///
///
/// # [Name](Account#name)
///
///
/// An `Account's` name is used to identify an Account in multiple methods involving [child](Account#accounts) `Accounts`.
/// For this reason child `Accounts` need to be uniquely named to be [valid](Account#valid).
///
///  - [`name`](Account::name): Get an account's name
///
///  - [`rename`](Account::rename): Rename an `Account`
///
///  - [`deep_rename`](Account::deep_rename): Rename a [child](Account#accounts)  `Accounts`
///
///
/// # [Active](Account#active)
///
///
/// If a child `Account` is inactive it will be ignore by certain methods like [get()](Account::get)
/// when this are called on the parent `Account`.
///
///  - [`active`](Account::active): Get an account's activity state
///
///  - [`change_activity`](Account::change_activity): Change the activity
///
///  - [`deep_change_activity`](Account::deep_change_activity): Change the activity of one of the child `Accounts`
///
///
/// # [Settings](Account#settings)
///
///
/// A `HashMap` holding [Settings](Setting). Contains all the settings present in the
/// [child](Account#accounts) Accounts but can contain settings that aren't in them.
///
///  - [`get`](Account::get): Returns a reference to the value corresponding to the key
///
///  - [`deep_get`](Account::deep_get): Returns a reference to the value corresponding to the key on a child Account
///
///  - [`insert`](Account::insert): Inserts a key-value pair into the map.
///
///  - [`deep_insert`](Account::deep_insert):Inserts a key-value pair into the map of a child Account
///
///  - [`insert_box`](Account::insert): `insert` but `Box<dyn Setting>` instead of S
///
///  - [`deep_insert_box`](Account::deep_insert_box): `deep_insert` but `Box<dyn Setting>` instead of S
///
///  - [`keys`](Account::keys): An iterator visiting all keys in arbitrary order
///
///  - [`contains_key`](Account::contains_key): Returns `true` if the `Account` contains a value for the specified key
///
///  - [`capacity`](Account::capacity): Returns the number of elements the map can hold without reallocating.
///
///
/// # [Accounts](Account#accounts)
///
///
/// A `Vec` of Accounts. The Account that holds the `Vec` is the parent Account and the Accounts that are being held
/// are the child Accounts.
///
/// The consider the bottom layer of the `Vec` Account at index 0, and the top layer the on at len()-1.
///
/// When the `Vec` is changed, the parent account will update its settings, such that when
/// we use [get](Account.get) on the parent Account we obtain the value from the top layer
/// containing the setting or return `None` if no layer contained it.
///
///  - [`accounts`](Account::accounts): Get an Account's child `Accounts`
///
///  - [`len`](Account::len): Returns the number of elements in the `Vec`.
///
///  - [`is_empty`](Account::is_empty): Returns `true` if the `Vec` contains no elements.
///
///  - [`push`](Account::push): Appends an `Account` to the back of the `Vec`.
///
///  - [`pop`](Account::pop): Removes the last element from a vector and returns it, or [`None`] if it is empty.
///
///  - [`pop_keep`](Account::pop_keep): `pop` but keeps settings in the main account
///     even if they are not present in other child accounts
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
/// They accept an extra `Vec` of `&K` that are the list of child `Accounts`
/// you have to pass though to get to the child `Account` the function will be called.
/// For each value in the `Vec` the value to its right is its parent. Meaning that the right most value
/// is the a direct child of the `Account` we call the function on, and the left most is the the `Account`
/// we will interact with.
///
/// Deep functions can return [`DeepError`]'s
///
/// The main function is [deep](Account::deep) to get a reference to a child `Account`,
/// [deep_mut](Account::deep_mut) exists but it can make an Account [invalid](Account#valid)
/// so its recommend to use the `deep` version of methods instead
///  
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
#[must_use]
pub struct Account<
    N: Setting + Clone + Debug + Eq + Hash + Default,
    K: Clone + Debug + Eq + Hash + 'static,
    V: Clone + Debug + PartialEq + 'static,
> {
    name: N,
    active: bool,
    hashmap: HashMap<K, V>,
    accounts: Vec<Account<N, K, V>>,
}
impl<
        N: Setting + Clone + Debug + Eq + Hash + Default,
        K: Clone + Debug + Eq + Hash + 'static,
        V: Clone + Debug + PartialEq + 'static,
    > Account<N, K, V>
{
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
    /// use hashmap_settings::{Account};
    /// let account = Account::new(
    ///     "New Account".to_string(),
    ///     true,
    ///     HashMap::from([
    ///         ("answer".to_string(),42),
    ///         ("zero".to_string(),0),
    ///         ("big_number".to_string(),10000),
    ///     ]),
    ///     vec![
    ///         Account::new("1".to_string(), true, Default::default(), Default::default()),
    ///         Account::new("2".to_string(), true, Default::default(), Default::default()),
    ///         Account::new("3".to_string(), true, Default::default(), Default::default())
    ///     ],
    /// );
    ///
    /// assert_eq!(account.name(), "New Account");
    /// assert!(account.active());
    /// assert!(account.hashmap() ==
    ///     &HashMap::from([
    ///         ("answer".to_string(),42),
    ///         ("zero".to_string(),0),
    ///         ("big_number".to_string(),10000)
    ///     ])
    /// );
    /// assert!(account.accounts() ==
    ///     &vec![
    ///         Account::new("1".to_string(), true, Default::default(), Default::default()),
    ///         Account::new("2".to_string(), true, Default::default(), Default::default()),
    ///         Account::new("3".to_string(), true, Default::default(), Default::default())
    ///     ],
    /// );
    ///
    /// ```
    pub fn new(name: N, active: bool, settings: HashMap<K, V>, accounts: Vec<Self>) -> Self {
        Self {
            name,
            active,
            hashmap: settings,
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
    /// let account = Account::<String,(),()>::new_valid(
    ///     "New Account".to_string(),
    ///     Default::default(),
    ///     Default::default(),
    ///     vec![
    ///         Account::new("1".to_string(), true, Default::default(), Default::default()),
    ///         Account::new("2".to_string(), true, Default::default(), Default::default()),
    ///         Account::new("3".to_string(), true, Default::default(), Default::default())
    ///     ],
    /// );
    /// assert_eq!(account, Ok(Account::<String,(),()>::new(
    ///     "New Account".to_string(),
    ///     Default::default(),
    ///     Default::default(),
    ///     vec![
    ///         Account::new("1".to_string(), true, Default::default(), Default::default()),
    ///         Account::new("2".to_string(), true, Default::default(), Default::default()),
    ///         Account::new("3".to_string(), true, Default::default(), Default::default())
    ///     ],
    /// )));
    /// ```
    ///
    /// # Errors
    ///
    /// ```
    /// use hashmap_settings::types::errors::InvalidAccountError;
    /// use hashmap_settings::Account;
    /// let account = Account::<String,(),()>::new_valid(
    ///     "New Account".to_string(),
    ///     Default::default(),
    ///     Default::default(),
    ///     vec![
    ///         Account::new("1".to_string(), true, Default::default(), Default::default()),
    ///         Account::new("1".to_string(), true, Default::default(), Default::default()),
    ///         Account::new("1".to_string(), true, Default::default(), Default::default())
    ///     ],
    /// );
    /// assert_eq!(account, Err(InvalidAccountError::ExistingName));
    /// ```
    pub fn new_valid(
        name: N,
        active: bool,
        settings: HashMap<K, V>,
        accounts: Vec<Self>,
    ) -> Result<Self, InvalidAccountError> {
        let new_account = Self {
            name,
            active,
            hashmap: settings,
            accounts,
        };
        new_account
            .is_invalid()
            .map_or_else(|| Ok(new_account), Err)
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
        for account in self.accounts() {
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
    /// let account = Account::<String,(),()>::new(
    ///     "New account".to_string(),
    ///     Default::default(),
    ///     Default::default(),
    ///     Default::default()
    /// );
    ///
    /// assert_eq!(account.name(), "New account");
    /// ```
    #[must_use]
    pub const fn name(&self) -> &N {
        &self.name
    }
    /// Return a reference to the `HashMap`
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::{Account};
    /// use std::collections::HashMap;
    /// let account = Account::<String,String,i32>::new(
    ///     "New Account".to_string(),
    ///     Default::default(),
    ///     HashMap::from([
    ///         ("answer".to_string(),42),
    ///         ("zero".to_string(),0),
    ///         ("big_number".to_string(),10000),
    ///     ]),
    ///     Default::default(),
    /// );
    ///
    /// assert!(account.hashmap() ==
    ///     &HashMap::from([
    ///         ("answer".to_string(),42),
    ///         ("zero".to_string(),0),
    ///         ("big_number".to_string(),10000),
    ///     ])
    /// );
    ///
    /// ```
    #[must_use]
    pub const fn hashmap(&self) -> &HashMap<K, V> {
        &self.hashmap
    }
    /// Return a reference to the `Vec` of child `Accounts`
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::{Account};
    /// let account = Account::<i32,(),()>::new(
    ///     0,
    ///     Default::default(),
    ///     Default::default(),
    ///     vec![
    ///         Account::new(1, true, Default::default(), Default::default()),
    ///         Account::new(2, true, Default::default(), Default::default()),
    ///         Account::new(3, true, Default::default(), Default::default())
    ///     ],
    /// );
    ///
    /// assert!(account.accounts() ==
    ///     &vec![
    ///         Account::new(1, true, Default::default(), Default::default()),
    ///         Account::new(2, true, Default::default(), Default::default()),
    ///         Account::new(3, true, Default::default(), Default::default())
    ///     ],
    /// );
    ///
    /// ```
    #[must_use]
    pub const fn accounts(&self) -> &Vec<Self> {
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
    /// let mut account = Account::<(),(),()>::new(Default::default(), true, Default::default(), Default::default());
    ///
    /// assert!(account.active());
    /// account.change_activity(false);
    /// assert!(!account.active());
    ///
    /// ```
    #[must_use]
    pub const fn active(&self) -> bool {
        self.active
    }
    /// Takes a `bool` and changes the value of active, returns `true` if changes were made.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::{Account};
    /// let mut account = Account::<(),(),()>::new(Default::default(), false, Default::default(), Default::default());
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
    /// Part of the [deep functions](Account#deep-functions) group that accept a `Vec` of &N to identify
    /// the child `Account` to run the function. [`change_activity`](Account::change_activity) in this case.
    ///
    /// Also updates the settings, contained on the updated account, in all the affected accounts such that they
    /// contain the correct values.
    ///
    /// # Errors
    ///
    /// Deep functions can return [`DeepError`]'s
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::{Account};
    /// let mut account = Account::<String,(),()>::new(
    ///     "New Account".to_string(),
    ///     Default::default(),
    ///     Default::default(),
    ///     vec![
    ///         Account::new("1".to_string(), true, Default::default(), Default::default()),
    ///         Account::new("2".to_string(), true, Default::default(), Default::default()),
    ///         Account::new("3".to_string(), true, Default::default(), vec![
    ///             Account::new("3_1".to_string(), true, Default::default(), Default::default()),
    ///             Account::new("3_2".to_string(), true, Default::default(), Default::default()),
    ///             Account::new("3_3".to_string(), true, Default::default(), Default::default())
    ///         ])
    ///     ],
    /// );
    ///
    /// assert_eq!(account.deep_change_activity(false,&mut vec![&"3_2".to_string(),&"3".to_string()]), Ok(true));
    /// assert_eq!(account, Account::new(
    ///     "New Account".to_string(),
    ///     Default::default(),
    ///     Default::default(),
    ///     vec![
    ///         Account::new("1".to_string(), true, Default::default(), Default::default()),
    ///         Account::new("2".to_string(), true, Default::default(), Default::default()),
    ///         Account::new("3".to_string(), true, Default::default(), vec![
    ///             Account::new("3_1".to_string(), true, Default::default(), Default::default()),
    ///             Account::new("3_2".to_string(), false, Default::default(), Default::default()),
    ///             Account::new("3_3".to_string(), true, Default::default(), Default::default())
    ///         ])
    ///     ],
    /// ));
    /// ```
    pub fn deep_change_activity(
        &mut self,
        new_active: bool,
        account_names: &mut Vec<&N>,
    ) -> Result<bool, DeepError> {
        self.deep_change_activity_helper(new_active, account_names)
            .0
    }
    fn deep_change_activity_helper(
        &mut self,
        new_active: bool,
        account_names: &mut Vec<&N>,
    ) -> (Result<bool, DeepError>, Vec<K>) {
        let Some(account_to_find) = account_names.pop() else {
            return (Err(DeepError::EmptyVec), vec![]); //error if the original call is empty, but this will create the base case in the recursive call
        };
        #[allow(clippy::option_if_let_else)]
        if let Some(found_account) = self.mut_account_from_name(account_to_find) {
            match found_account.deep_change_activity_helper(new_active, account_names) {
                //recursive call
                (Ok(insert_option), settings) => {
                    self.update_vec(&settings.iter().collect());
                    //after the base this will be called in all previous function calls,
                    //updating the value in the corresponding Account.settings
                    (Ok(insert_option), settings) //returning the original value from the base case
                }
                (Err(error), _) => match error {
                    DeepError::EmptyVec => (
                        Ok(found_account.change_activity(new_active)),
                        found_account
                            .keys()
                            .map(std::borrow::ToOwned::to_owned)
                            .collect::<Vec<_>>(),
                    ), //base case
                    DeepError::NotFound => (Err(error), vec![]), //error, invalid function call
                },
            }
        } else {
            (Err(DeepError::NotFound), vec![])
        }
    }
    /// Takes a `&N` and updates the name of the `Account`.
    ///
    /// Returns the previous name that the Account had.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::{Account};
    /// let mut account = Account::<String,(),()>::new(
    ///     "Old Name".to_string(),
    ///     Default::default(),
    ///     Default::default(),
    ///     Default::default()
    /// );
    /// assert_eq!(account.name(), "Old Name");
    /// assert_eq!(account.rename("New Name".to_string()), "Old Name".to_string());
    /// assert_eq!(account.name(), "New Name");
    /// ```
    pub fn rename(&mut self, new_name: N) -> N {
        let r_value = self.name.clone(); //todo!(there should be a way to take the new value without cloning)
        self.name = new_name;
        r_value
    }
    /// Takes a `&N` and updates the name of a child `Account`.
    ///
    /// This can make a Account [invalid](Account#valid) if the child Account
    /// got renamed to the same name as one of it's siblings.
    ///
    /// Part of the [deep functions](Account#deep-functions) group that accept a `Vec` of &N to identify
    /// the child `Account` to run the function. [`rename`](Account::rename) in this case.
    ///
    /// # Errors
    ///
    /// Deep functions can return [`DeepError`]'s
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::{Account};
    /// let mut account = Account::<String,(),()>::new(
    ///     "Old Name".to_string(),
    ///     Default::default(),
    ///     Default::default(),
    ///     vec![
    ///         Account::new("1".to_string(), true, Default::default(), Default::default()),
    ///         Account::new("2".to_string(), true, Default::default(), Default::default()),
    ///         Account::new("3".to_string(), true, Default::default(), vec![
    ///             Account::new("3_1".to_string(), true, Default::default(), Default::default()),
    ///             Account::new("3_2".to_string(), true, Default::default(), Default::default()),
    ///             Account::new("3_3".to_string(), true, Default::default(), Default::default())
    ///         ])
    ///     ],
    /// );
    ///
    /// assert_eq!(account.deep_rename("Cool Name".to_string(),&mut vec![&"3_2".to_string(),&"3".to_string()]), Ok("3_2".to_string()));
    /// assert_eq!(account, Account::new(
    ///     "Old Name".to_string(),
    ///     Default::default(),
    ///     Default::default(),
    ///     vec![
    ///         Account::new("1".to_string(), true, Default::default(), Default::default()),
    ///         Account::new("2".to_string(), true, Default::default(), Default::default()),
    ///         Account::new("3".to_string(), true, Default::default(), vec![
    ///             Account::new("3_1".to_string(), true, Default::default(), Default::default()),
    ///             Account::new("Cool Name".to_string(), true, Default::default(), Default::default()),
    ///             Account::new("3_3".to_string(), true, Default::default(), Default::default())
    ///         ])
    ///     ],
    /// ));
    /// ```
    pub fn deep_rename(
        &mut self,
        new_name: N,
        account_names: &mut Vec<&N>,
    ) -> Result<N, DeepError> {
        match self.deep_mut(account_names) {
            Ok(found_account) => Ok(found_account.rename(new_name)),
            Err(error) => Err(error),
        }
    }
    /// Returns a reference to a child `Account`.
    ///
    /// `deep` can be used with other methods that don't need a `&mut self` (like
    /// [get](Account::get) or [len](Account::len)) to use those methods on child `Account`s
    ///
    /// Part of the [deep functions](Account#deep-functions) group that accept a `Vec` of &N to identify
    /// the child `Account` to run the function.
    ///
    /// # Errors
    ///
    /// Deep functions can return [`DeepError`]'s
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use hashmap_settings::{Account};
    /// let account = Account::<String,String,i32>::new(
    ///     "Parent Account".to_string(),
    ///     Default::default(),
    ///     Default::default(),
    ///     vec![
    ///         Account::new("1".to_string(), true, Default::default(), Default::default()),
    ///         Account::new("2".to_string(), true, Default::default(), Default::default()),
    ///         Account::new("3".to_string(), true, Default::default(), vec![
    ///             Account::new("3_1".to_string(), true, Default::default(), Default::default()),
    ///             Account::new(
    ///                 "3_2".to_string(),
    ///                 true,
    ///                 HashMap::from([
    ///                     ("answer".to_string(),42),
    ///                     ("zero".to_string(),0),
    ///                     ("big_number".to_string(),10000),
    ///                 ]),
    ///                 Default::default()),
    ///             Account::new("3_3".to_string(), true, Default::default(), Default::default()),
    ///         ])
    ///     ],
    /// );
    ///
    /// assert_eq!(account.deep(&mut vec![&"3_2".to_string(),&"3".to_string()])?.get(&"answer".to_string()), Some(&42));
    /// # Ok::<(), hashmap_settings::types::errors::DeepError>(())
    /// ```
    pub fn deep(&self, account_names: &mut Vec<&N>) -> Result<&Self, DeepError> {
        let Some(account_to_find) = account_names.pop() else {
            return Err(DeepError::EmptyVec); //error if the original call is empty, but this will create the base case in the recursive call
        };
        self.account_from_name(account_to_find)
            .map_or(
                Err(DeepError::NotFound),
                |found_account| match found_account.deep(account_names) {
                    //recursive call
                    Err(error) => match error {
                        DeepError::EmptyVec => Ok(found_account), //base case
                        DeepError::NotFound => Err(error),        //error, invalid function call
                    },
                    Ok(value) => Ok(value),
                },
            )
    }
    /// Returns a mutable reference to a child `Account`.
    ///
    /// Consider using [`deep`](Account::deep) with methods that don't need a `&mut self`,
    /// or the respective [deep_function](Account#deep-functions) for a specific method as
    /// `deep_mut` can make an account [invalid])(Account#valid)
    ///
    /// Part of the [deep functions](Account#deep-functions) group that accept a `Vec` of &N to identify
    /// the child `Account` to run the function.
    ///
    /// # Errors
    ///
    /// Deep functions can return [`DeepError`]'s
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use hashmap_settings::{Account};
    /// let mut account = Account::<String,String,i32>::new(
    ///     "Old Name".to_string(),
    ///     Default::default(),
    ///     Default::default(),
    ///     vec![
    ///         Account::new("1".to_string(), true, Default::default(), Default::default()),
    ///         Account::new("2".to_string(), true, Default::default(), Default::default()),
    ///         Account::new("3".to_string(), true, Default::default(), vec![
    ///             Account::new("3_1".to_string(), true, Default::default(), Default::default()),
    ///             Account::new(
    ///                 "3_2".to_string(),
    ///                 true,
    ///                 HashMap::from([
    ///                     ("answer".to_string(),42),
    ///                     ("zero".to_string(),0),
    ///                     ("big_number".to_string(),10000),
    ///                 ]),
    ///                 Default::default()),
    ///             Account::new("3_3".to_string(), true, Default::default(), Default::default()),
    ///         ])
    ///     ],
    /// );
    /// assert_eq!(account.deep_mut(&mut vec![&"3_2".to_string(),&"3".to_string()])?.insert("answer".to_string(), 777), Some(42));
    /// assert_eq!(account.deep(&mut vec![&"3_2".to_string(),&"3".to_string()])?.get(&"answer".to_string()), Some(&777));
    /// # Ok::<(), hashmap_settings::types::errors::DeepError>(())
    /// ```
    pub fn deep_mut(&mut self, account_names: &mut Vec<&N>) -> Result<&mut Self, DeepError> {
        let Some(account_to_find) = account_names.pop() else {
            return Err(DeepError::EmptyVec); //error if the original call is empty, but this will create the base case in the recursive call
        };
        if let Some(found_account) = self.mut_account_from_name(account_to_find) {
            if account_names.is_empty() {
                //this and the unreachable()! have been added due to https://github.com/rust-lang/rust/issues/21906
                return Ok(found_account);
            }
            match found_account.deep_mut(account_names) {
                //recursive call
                Ok(value) => {
                    Ok(value) //returning the original value from the base case
                }
                Err(error) => match error {
                    DeepError::EmptyVec => {
                        unreachable!() //Ok(found_account)
                    } //base case
                    DeepError::NotFound => Err(error), //error, invalid function call
                },
            }
        } else {
            Err(DeepError::NotFound)
        }
    }
    fn account_from_name(&self, name: &N) -> Option<&Self> {
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
    /// let account = Account::<String,(),()>::new(
    ///     "New Account".to_string(),
    ///     Default::default(),
    ///     Default::default(),
    ///     vec![
    ///         Account::new("1".to_string(), true, Default::default(), Default::default()),
    ///         Account::new("2".to_string(), true, Default::default(), Default::default()),
    ///         Account::new("3".to_string(), true, Default::default(), Default::default())
    ///     ],
    /// );
    ///
    /// assert!(account.accounts_names() == vec!["1","2","3"]);
    ///
    /// ```
    #[must_use]
    pub fn accounts_names(&self) -> Vec<&N> {
        self.accounts.iter().map(Self::name).collect()
    }
    /// Inserts a key-value pair into the map of a child `Account`.
    ///
    /// This will updated the [settings](Account#settings) of all necessary Accounts
    /// so that the parent Account remains [valid](Account#valid)
    ///
    /// Part of the [deep functions](Account#deep-functions) group that accept a `Vec` of &N to identify
    /// the child `Account` to run the function. [`insert`](Account::insert) in this case.
    ///
    /// # Errors
    ///
    /// Deep functions can return [`DeepError`]'s
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use hashmap_settings::{Account};
    /// let mut account = Account::<String,String,i32>::new(
    ///     "Old Name".to_string(),
    ///     Default::default(),
    ///     Default::default(),
    ///     vec![
    ///         Account::new("1".to_string(), true, Default::default(), Default::default()),
    ///         Account::new("2".to_string(), true, Default::default(), Default::default()),
    ///         Account::new("3".to_string(), true, Default::default(), vec![
    ///             Account::new("3_1".to_string(), true, Default::default(), Default::default()),
    ///             Account::new(
    ///                 "3_2".to_string(),
    ///                 true,
    ///                 HashMap::from([
    ///                     ("answer".to_string(),42),
    ///                     ("zero".to_string(),0),
    ///                     ("big_number".to_string(),10000),
    ///                 ]),
    ///                 Default::default()),
    ///             Account::new("3_3".to_string(), true, Default::default(), Default::default()),
    ///         ])
    ///     ],
    /// );
    ///
    /// assert_eq!(account.deep_insert(&"answer".to_string(), 777, &mut vec![&"3_2".to_string(),&"3".to_string()]), Ok(Some(42)));
    /// assert_eq!(account.deep(&mut vec![&"3_2".to_string(),&"3".to_string()])?.get(&"answer".to_string()), Some(&777));
    /// # Ok::<(), hashmap_settings::types::errors::DeepError>(())
    /// ```
    pub fn deep_insert(
        &mut self,
        setting_name: &K,
        setting_value: V,
        account_names: &mut Vec<&N>,
    ) -> Result<Option<V>, DeepError> {
        let Some(account_to_find) = account_names.pop() else {
            return Err(DeepError::EmptyVec); //error if the original call is empty, but this will create the base case in the recursive call
        };
        #[allow(clippy::option_if_let_else)]
        if let Some(found_account) = self.mut_account_from_name(account_to_find) {
            match found_account.deep_insert(setting_name, setting_value.clone(), account_names) {
                //recursive call
                Ok(insert_option) => {
                    self.update_setting(setting_name);
                    //after the base this will be called in all previous function calls,
                    //updating the value in the corresponding Account.settings
                    Ok(insert_option) //returning the original value from the base case
                }
                Err(error) => match error {
                    DeepError::EmptyVec => {
                        Ok(found_account.insert(setting_name.to_owned(), setting_value))
                    } //base case
                    DeepError::NotFound => Err(error), //error, invalid function call
                },
            }
        } else {
            Err(DeepError::NotFound)
        }
    }
    /// Updates a setting with the value its supposed to have.
    ///
    /// Returns `None` if the setting isn't present in the Account or child Accounts.
    /// Returns `Some(true)` if the value of the setting was updated.
    /// Returns `Some(false)` if the value is in the Account but was not updated.
    ///
    /// if you don't need the return value use [update_setting](update_setting) as it is faster
    ///
    /// If an Account is [valid](Account#valid) this method never returns Some(true)
    /// as this method is used to turn an invalid Account into a valid one.
    ///
    /// # Examples
    /// ```
    ///  //todo!() add example
    /// ```
    #[must_use]
    pub fn update_setting_returns(&mut self, setting: &K) -> Option<bool> {
        for account in (0..self.len()).rev() {
            if self.accounts[account].active {
                if let Some(value) = self.accounts[account].hashmap.get(setting) {
                    return Some(
                        !self
                            .hashmap
                            .insert(setting.to_owned(), value.clone())
                            .map_or(false, |x| &x == value),
                    );
                }
            }
        }
        self.hashmap.remove(setting).map(|_| true)
    }
    /// Updates a setting with the value its supposed to have.
    ///
    /// This function doesn't return anything, consider using [update_setting_returns](Account::update_setting_returns)
    /// if a return value is needed.
    ///
    /// Use [update_vec](Account::update_vec) if you want to update multiple settings.
    ///
    /// Use [update_all_settings](Account::update_all_settings) if you want to update all settings.
    ///
    /// If an Account is [valid](Account#valid) this wont do anything.
    ///
    /// # Examples
    /// ```
    ///  //todo!() add example
    /// ```
    pub fn update_setting(&mut self, setting: &K) {
        for account in (0..self.len()).rev() {
            if self.accounts[account].active {
                if let Some(value) = self.accounts[account].hashmap.get(setting) {
                    self.hashmap.insert(setting.to_owned(), value.clone());
                    return;
                }
            }
        }
        self.hashmap.remove(setting);
    }
    /// Updates a group of settings with the value they are supposed to have.
    ///
    /// If an Account is [valid](Account#valid) this wont do anything.
    ///
    /// Use [update_setting](Account::update_setting) if you want to update a single setting.
    ///
    /// Use [update_all_settings](Account::update_all_settings) if you want to update all settings.
    ///
    /// # Examples
    /// ```
    ///  //todo!() add example
    /// ```
    pub fn update_vec(&mut self, settings: &Vec<&K>) {
        'setting: for setting in settings {
            for account in (0..self.len()).rev() {
                if self.accounts[account].active {
                    if let Some(value) = self.accounts[account].hashmap.get(*setting) {
                        self.hashmap.insert((*setting).to_owned(), value.clone());
                        continue 'setting;
                    }
                }
            }
            self.hashmap.remove(*setting);
        }
    }
    /// Updates all settings in the Account with the value they are supposed to have.
    ///
    /// If an Account is [valid](Account#valid) this wont do anything.
    ///
    /// Use [update_setting](Account::update_setting) if you want to update a single setting.
    ///
    /// Use [update_vec](Account::update_vec) if you want to update multiple but not all settings.
    ///
    /// # Examples
    /// ```
    ///  //todo!() add example
    /// ```
    pub fn update_all_settings(&mut self) {
        let settings = self
            .hashmap
            .keys()
            .map(std::borrow::ToOwned::to_owned)
            .collect::<Vec<_>>();
        'setting: for setting in settings {
            for account in (0..self.len()).rev() {
                if self.accounts[account].active {
                    if let Some(value) = self.accounts[account].hashmap.get(&setting.clone()) {
                        self.hashmap.insert(setting.clone(), value.clone());
                        continue 'setting;
                    }
                }
            }
            self.hashmap.remove(&setting);
        }
    }
    fn mut_account_from_name(&mut self, name: &N) -> Option<&mut Self> {
        for account in 0..self.len() {
            if self.accounts[account].name() == name {
                return Some(&mut self.accounts[account]);
            }
        }
        None
    }
    /// Appends an `Account` to the back of the `Vec` of child `Accounts`.
    ///
    /// This child `Account` settings will be added to the settings of the parent `Account` that `push` was called on.
    ///
    /// The parent Account will be updated with the new settings unless the inserted child `Account` is [inactive](Account::active).
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
    /// let mut account = Account::<i32,(),()>::new(
    ///     Default::default(),
    ///     Default::default(),
    ///     Default::default(),
    ///     vec![
    ///         Account::new(1, Default::default(), Default::default(), Default::default()),
    ///         Account::new(2, Default::default(), Default::default(), Default::default())
    ///     ],
    /// );
    /// account.push_unchecked(Account::new(3, Default::default(), Default::default(), Default::default()));
    /// assert!(account ==
    ///     Account::new(
    ///         Default::default(),
    ///         Default::default(),
    ///         Default::default(),
    ///         vec![
    ///             Account::new(1, Default::default(), Default::default(), Default::default()),
    ///             Account::new(2, Default::default(), Default::default(), Default::default()),
    ///             Account::new(3, Default::default(), Default::default(), Default::default())
    ///         ],
    ///     )
    /// )
    /// ```
    pub fn push_unchecked(&mut self, account: Self) {
        if account.active {
            for setting in account.hashmap.keys() {
                self.insert(setting.to_owned(), account.get(setting).unwrap().clone());
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
    /// use hashmap_settings::{Account};
    /// let mut account: Account<(),&str,i32> = Default::default();
    /// account.insert("a small number", 42);
    /// assert_eq!(account.contains_key(&"a small number"), true);
    /// assert_eq!(account.contains_key(&"a big number"), false);
    /// ```
    #[must_use]
    pub fn contains_key(&self, setting_name: &K) -> bool {
        self.hashmap.contains_key(setting_name)
    }
    /// Returns the value corresponding to the key.
    ///
    /// This method is a direct call to [`HashMap`]'s [`get()`](HashMap::get).
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::{Account};
    /// let mut account: Account<(),&str,i32> = Default::default();
    /// account.insert("a small number", 42);
    /// assert_eq!(account.get(&"a small number"), Some(&42));
    /// assert_eq!(account.get(&"a big number"), None);
    /// ```
    #[must_use]
    #[allow(clippy::borrowed_box)]
    pub fn get(&self, setting_name: &K) -> Option<&V> {
        self.hashmap.get(setting_name)
    }
    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, `None` is returned.
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
    /// use hashmap_settings::{Account};
    /// let mut account: Account<(),&str,i32> = Default::default();
    /// assert_eq!(account.insert("a small number", 1), None);
    /// assert_eq!(account.hashmap().is_empty(), false);
    ///
    /// account.insert("a small number", 2);
    /// assert_eq!(account.insert("a small number", 3), Some(2));
    /// assert!(account.hashmap()[&"a small number"] == 3);
    /// ```
    pub fn insert(&mut self, setting_name: K, setting_value: V) -> Option<V> {
        self.hashmap.insert(setting_name, setting_value)
    }
    /// An iterator visiting all keys in arbitrary order.
    /// The iterator element type is `&'a K`.
    ///
    /// This method is a direct call to [`HashMap`]'s [`keys()`](HashMap::keys()).
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::{Account};
    /// use std::collections::HashMap;
    /// let account = Account::<(),String,i32>::new(
    ///     Default::default(),
    ///     Default::default(),
    ///     HashMap::from([
    ///         ("answer".to_string(),42),
    ///         ("zero".to_string(),0),
    ///         ("big_number".to_string(),10000),
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
    #[must_use]
    pub fn keys(&self) -> hash_map::Keys<'_, K, V> {
        self.hashmap.keys()
    }
    /// Removes a setting from the map, returning the value at the key if the key was previously in the map.
    ///
    /// This method is a direct call to [`HashMap`]'s [`remove()`](HashMap::remove).
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::{Account};
    /// let mut account: Account<(),&str,i32> = Default::default();
    /// assert_eq!(account.insert("a small number", 1), None);
    /// assert_eq!(account.remove(&"a small number"), Some(1));
    /// assert_eq!(account.remove(&"a small number"), None);
    /// ```
    pub fn remove(&mut self, setting_to_remove: &K) -> Option<V> {
        self.hashmap.remove(setting_to_remove)
    }
    /// Removes a setting from the map, returning the value at the key if the key was previously in the map.
    ///
    /// Part of the [deep functions](Account#deep-functions) group that accept a `Vec` of &N to identify
    /// the child `Account` to run the function. [`remove`](Account::remove) in this case.
    ///
    /// This method is a direct call to [`HashMap`]'s [`remove()`](HashMap::remove).
    ///
    /// # Errors
    ///
    /// Deep functions can return [`DeepError`]'s
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use hashmap_settings::{Account};
    /// let mut account = Account::<String,String,i32>::new(
    ///     "Old Name".to_string(),
    ///     Default::default(),
    ///     Default::default(),
    ///     vec![
    ///         Account::new("1".to_string(), true, Default::default(), Default::default()),
    ///         Account::new("2".to_string(), true, Default::default(), Default::default()),
    ///         Account::new("3".to_string(), true, Default::default(), vec![
    ///             Account::new("3_1".to_string(), true, Default::default(), Default::default()),
    ///             Account::new(
    ///                 "3_2".to_string(),
    ///                 true,
    ///                 HashMap::from([
    ///                     ("answer".to_string(),42),
    ///                     ("zero".to_string(),0),
    ///                     ("big_number".to_string(),10000),
    ///                 ]),
    ///                 Default::default()),
    ///             Account::new("3_3".to_string(), true, Default::default(), Default::default()),
    ///         ])
    ///     ],
    /// );
    ///
    /// assert_eq!(account.deep_remove(&"answer".to_string(),&mut vec![&"3_2".to_string(),&"3".to_string()]), Ok(Some(42)));
    /// assert_eq!(account.deep(&mut vec![&"3_2".to_string(),&"3".to_string()])?.get(&"int".to_string()), None);
    /// # Ok::<(), hashmap_settings::types::errors::DeepError>(())
    /// ```
    pub fn deep_remove(
        &mut self,
        setting_to_remove: &K,
        account_names: &mut Vec<&N>,
    ) -> Result<Option<V>, DeepError> {
        let Some(account_to_find) = account_names.pop() else {
            return Err(DeepError::EmptyVec); //error if the original call is empty, but this will create the base case in the recursive call
        };
        #[allow(clippy::option_if_let_else)]
        if let Some(found_account) = self.mut_account_from_name(account_to_find) {
            match found_account.deep_remove(setting_to_remove, account_names) {
                //recursive call
                Ok(insert_option) => {
                    self.update_setting(setting_to_remove);
                    //after the base this will be called in all previous function calls,
                    //updating the value in the corresponding Account.settings
                    Ok(insert_option) //returning the original value from the base case
                }
                Err(error) => match error {
                    DeepError::EmptyVec => Ok(found_account.remove(setting_to_remove)), //base case
                    DeepError::NotFound => Err(error), //error, invalid function call
                },
            }
        } else {
            Err(DeepError::NotFound)
        }
    }
    /// Returns the number of elements the map can hold without reallocating.
    ///
    /// This number is a lower bound; the `HashMap<K, V>` might be able to hold
    /// more, but is guaranteed to be able to hold at least this many.
    ///
    /// This method is a direct call to [`HashMap`]'s [`keys()`](HashMap::keys()).
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::{Account};
    /// use std::collections::HashMap;
    /// let account = Account::<(),(),()>::new(Default::default(), Default::default(), HashMap::with_capacity(100), Default::default());
    /// assert!(account.capacity() >= 100);
    /// ```
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.hashmap.capacity()
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
    /// let account = Account::<i32,(),()>::new(
    ///         Default::default(),
    ///         Default::default(),
    ///         Default::default(),
    ///         vec![
    ///             Account::new(1, Default::default(), Default::default(), Default::default()),
    ///             Account::new(2, Default::default(), Default::default(), Default::default()),
    ///             Account::new(3, Default::default(), Default::default(), Default::default())
    ///         ],
    ///     );
    /// assert_eq!(account.len(), 3);
    /// ```
    #[must_use]
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
    /// let mut account = Account::<(),(),()>::default();
    /// assert!(account.is_empty());
    ///
    /// account.push(Account::<(),(),()>::default());
    /// assert!(!account.is_empty());
    /// ```
    #[must_use]
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
    /// let mut account = Account::<i32,(),()>::new(
    ///     Default::default(),
    ///     Default::default(),
    ///     Default::default(),
    ///     vec![
    ///         Account::new(1, Default::default(), Default::default(), Default::default()),
    ///         Account::new(2, Default::default(), Default::default(), Default::default())
    ///     ],
    /// );
    /// account.push(Account::new(3, Default::default(), Default::default(), Default::default()));
    /// assert!(account ==
    ///     Account::new(
    ///         Default::default(),
    ///         Default::default(),
    ///         Default::default(),
    ///         vec![
    ///             Account::new(1, Default::default(), Default::default(), Default::default()),
    ///             Account::new(2, Default::default(), Default::default(), Default::default()),
    ///             Account::new(3, Default::default(), Default::default(), Default::default())
    ///         ],
    ///     )
    /// );
    /// assert!(account.push(Account::new(3, Default::default(), Default::default(), Default::default()))
    ///     == Some(InvalidAccountError::ExistingName));
    /// ```
    pub fn push(&mut self, account: Self) -> Option<InvalidAccountError> {
        if self.accounts_names().contains(&account.name()) {
            //check if account has the same name as a sibling account
            return Some(InvalidAccountError::ExistingName);
        }
        if let Some(error) = account.is_invalid() {
            //check if Account is internally valid
            return Some(error);
        }
        if account.active {
            for setting in account.hashmap.keys() {
                self.insert(setting.to_owned(), account.get(setting).unwrap().clone());
            }
        }
        self.accounts.push(account);
        None
    }
    /// Removes the last element from the [`Vec`] of child `Account`s and returns it, or [`None`] if it is empty.
    ///
    /// This method doesn't update the parent `Account` making it [invalid](Account#valid), so it's use
    /// is only recommend if multiple `Accounts` are being removed.
    /// 
    /// Use [pop](Account::pop) if you intend to update the settings from
    /// the main `Account` present on the popped child `Account`.
    /// 
    ///
    /// This method is a direct call to [`Vec`]'s [`pop()`](Vec::pop()).
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::{Account};
    /// let mut account = Account::<i32,(),()>::new(
    ///     Default::default(),
    ///     Default::default(),
    ///     Default::default(),
    ///     vec![
    ///         Account::new(1, Default::default(), Default::default(), Default::default()),
    ///         Account::new(2, Default::default(), Default::default(), Default::default()),
    ///         Account::new(3, Default::default(), Default::default(), Default::default())
    ///     ],
    /// );
    /// account.pop_keep();
    /// assert!(account ==
    ///     Account::<i32,(),()>::new(
    ///         Default::default(),
    ///         Default::default(),
    ///         Default::default(),
    ///         vec![
    ///             Account::new(1, Default::default(), Default::default(), Default::default()),
    ///             Account::new(2, Default::default(), Default::default(), Default::default())
    ///         ],
    ///     )
    /// )
    /// ```
    pub fn pop_keep(&mut self) -> std::option::Option<Self> {
        self.accounts.pop()
    }
    /// Removes the last element from the [`Vec`] of child `Account`s and returns it, or [`None`] if it is empty.
    ///
    /// Will update the settings from the parent `Account` present on the popped child `Account`.
    /// Consider using [pop_keep](Account::pop) if you are removing multiple child `Accounts`.
    ///
    ///
    /// This method contains a call to [`Vec`]'s [`pop()`](Vec::pop()).
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::{Account};
    /// let mut account = Account::<i32,(),()>::new(
    ///     Default::default(),
    ///     Default::default(),
    ///     Default::default(),
    ///     vec![
    ///         Account::new(1, Default::default(), Default::default(), Default::default()),
    ///         Account::new(2, Default::default(), Default::default(), Default::default()),
    ///         Account::new(3, Default::default(), Default::default(), Default::default())
    ///     ],
    /// );
    /// account.pop();
    /// assert!(account ==
    ///     Account::<i32,(),()>::new(
    ///         Default::default(),
    ///         Default::default(),
    ///         Default::default(),
    ///         vec![
    ///             Account::new(1, Default::default(), Default::default(), Default::default()),
    ///             Account::new(2, Default::default(), Default::default(), Default::default())
    ///         ],
    ///     )
    /// )
    /// ```
    pub fn pop(&mut self) -> std::option::Option<Self> {
        let popped_account = self.accounts.pop()?;
        for setting in popped_account.keys() {
            if !self.vec_contains_key(setting) {
                self.hashmap.remove(setting);
            }
        }
        Some(popped_account)
    }
    #[must_use]
    fn vec_contains_key(&self, setting: &K) -> bool {
        for account in self.accounts() {
            if account.contains_key(setting) {
                return true;
            }
        }
        false
    }
    ///todo!()
    #[must_use]
    pub fn get_mut_account(&mut self, index: usize) -> Option<&mut Self> {
        self.accounts.get_mut(index)
    }
    /*
        unused functions
        pub fn all_names(&self) -> Vec<&K> { //what would be the use
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
impl<
        N: Setting + Clone + Debug + Eq + Hash + Default,
        K: Clone + Debug + Eq + Hash + 'static,
        V: Clone + Debug + PartialEq + 'static,
    > Default for Account<N, K, V>
{
    fn default() -> Self {
        Self {
            name: N::default(),
            active: true,
            hashmap: HashMap::default(),
            accounts: Vec::default(),
        }
    }
}
cfg_if::cfg_if! {
    if #[cfg(serde)] {
        #[cfg_attr(feature = "serde", typetag::serialize)]
        impl<
                N: Setting + Clone + Debug + Eq + Hash + Default + Serialize + for<'a> Deserialize<'a>,
                K: Clone + Debug + Eq + Hash + 'static + Serialize + for<'a> Deserialize<'a>,
                V: Clone + Debug + PartialEq + 'static + Serialize + for<'a> Deserialize<'a>,
            > Setting for Account<N, K, V>
        {
            fn typetag_deserialize(&self) {
                //todo!(figure what this is supposed to do as its not mut, and return "()")
            }
        }
    }else{
        impl<
            N: Setting + Clone + Debug + Eq + Hash + Default,
            K: Clone + Debug + Eq + Hash + 'static,
            V: Clone + Debug + PartialEq + 'static,
        > Setting for Account<N, K, V>{}
    }
}

/// Required trait for conversion to abstract type [Stg]
///
/// For a Type to be able to implement Setting it needs to implement the traits
/// [Clone], [Debug], [PartialEq] (as well as [Deserialize] and [Serialize] if the "serde" feature is activated )
///
/// In the [future](https://github.com/OxidizedLoop/HashMapSettings/issues/1) you will be able to derive Setting,
/// but for now you can do it by adding the following lines:
/// ```
/// # use hashmap_settings::Setting;
/// # // use serde::{Deserialize, Serialize};
/// # #[derive(Clone, Debug, PartialEq)] //, Deserialize, Serialize
/// # pub struct MyType{}
/// // add #[typetag::serde] if serde feature is activated
///impl Setting for MyType{}
/// ```
#[cfg_attr(feature = "serde", typetag::serde(tag = "setting"))]
pub trait Setting: Any + Debug + DynClone + DynEq {
    /// turns a type implementing [Setting] into a [Stg]
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::{Setting,Stg};
    /// let bool = true;
    /// let bool_stg: Stg = bool.stg();
    /// assert!(bool_stg == bool.stg())
    /// ```
    fn stg(self) -> Stg
    where
        Self: Setting + Sized,
    {
        Stg {
            value: Box::new(self),
        }
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

/// Type abstraction for types implementing [`Setting`]
///
/// Types implementing `Setting` can be turned into a `Stg` with [.stg()](Setting::stg).
///
/// ```
/// todo!(example)
/// ```
///
/// They can be turned back to a specific type with [.unstg()](Stg::unstg) or [.unstg_panic()](Stg::unstg_panic)
///
///  ```
/// todo!(example)
/// ```
///
/// Additionally there is the [`StgTrait`] that can be implemented for types containing `Stg` to allow
/// `.unstg()` and `.unstg_panic()` to be called on them.
///
/// The main example would be [Option<&Stg>]
///
///  ```
/// todo!(example)
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
#[must_use]
pub struct Stg {
    value: Box<dyn Setting>,
}
impl Stg {
    /// turns a [`Stg`] into a `Result<S, Box<dyn Any>>`
    ///
    /// ´unstg´ is the main and safe way to used to get a concrete type `S` from `Stg`
    ///
    /// Consider using [`unstg_panic`] if it's guaranteed that we will convert to the right type.
    ///
    /// # Example
    ///
    /// ```
    /// use hashmap_settings::{Setting,Stg};
    ///
    /// let bool_stg: Stg = true.stg();
    /// assert_eq!(bool_stg.unstg::<bool>()?, true);
    /// //we need to use ::<bool> to specify that want to turn bool_stg into a bool
    /// # Ok::<(),Box<dyn core::any::Any>>(())
    /// ```
    ///
    /// ```
    /// use hashmap_settings::{Setting,Stg};
    ///
    /// let bool_stg: Stg = true.stg();
    /// let bool :bool = bool_stg.unstg()?;
    /// // here we don't as we specific the type annotation when we use :bool
    /// assert_eq!(bool, true);
    /// # Ok::<(),Box<dyn core::any::Any>>(())
    /// ```
    ///
    /// # Errors
    ///
    /// This function returns a Err(Box<dyn Any>) if we try to covert to the wrong type.
    ///
    /// ```
    /// use hashmap_settings::{Setting,Stg};
    ///
    /// let bool_stg: Stg = true.stg();
    /// let number = match bool_stg.unstg::<i32>(){
    ///     Ok(x)   => x, //unreachable!()
    ///     Err(x)  => {
    ///         print!("wrong conversion {:?}",x);
    ///         404
    ///     },
    /// };
    /// assert_eq!(number, 404)
    /// ```
    pub fn unstg<S: Setting>(self) -> Result<S, Box<dyn Any>> {
        let x: Box<dyn Any> = self.value;
        x.downcast().map(|t| *t)
    }
    /// turns a [`Stg`] into a concrete type `S`, can [`panic!`]
    ///
    /// This method is used to get a concrete type out of a `Stg`
    /// when it's know what `S` it contains.
    ///
    /// # Panics
    ///
    /// We need to be careful using `unstg_panic` as if we try convert to a type
    /// that isn't the one contained in `Stg` the program will panic.
    /// Consider using [`unstg`] as it returns a result type instead.
    ///
    /// ```should_panic
    /// use hashmap_settings::{Setting,Stg};
    ///
    /// let bool_stg: Stg = true.stg();
    /// let _number :i32 = bool_stg.unstg_panic();
    /// // this panics, as the Box<dyn Setting> holds a bool value but we are trying to convert it to a i32
    /// ```
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::{Setting,Stg};
    ///
    /// let bool_stg: Stg = true.stg();
    /// assert_eq!(bool_stg.unstg_panic::<bool>(), true);
    /// //we need to use ::<bool> to specify that want to turn bool_stg into a bool
    /// ```
    /// ```
    /// use hashmap_settings::{Setting,Stg};
    ///
    /// let bool_stg: Stg = true.stg();
    /// let bool :bool = bool_stg.unstg_panic();
    /// // here we don't as we specific the type annotation when we use :bool
    /// assert_eq!(bool, true);
    /// ```
    #[must_use]
    pub fn unstg_panic<S: Setting>(self) -> S {
        let x: Box<dyn Any> = self.value;
        *x.downcast().unwrap()
    }
}
#[cfg_attr(feature = "serde", typetag::serde)]
impl Setting for Stg {}
impl PartialEq for Stg {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value.clone()
    }
}
impl StgTrait for Option<&Stg> {
    fn unstg<S: Setting>(self) -> Result<S, StgError> {
        self.map_or(Err(StgError::None), |value| {
            match value.clone().unstg::<S>() {
                Ok(value) => Ok(value),
                Err(_error) => Err(StgError::WrongType), //todo! change WrongType to contain error Err(StgError::WrongType(error)),
            }
        })
    }
    fn unstg_panic<S: Setting>(self) -> S {
        self.unwrap().clone().unstg_panic()
    }
}

/// [`Stg`] container converter trait
///
/// This trait is implemented by types to facilitate the conversion from
/// `T`<`Stg`> to a concrete type `S`.
///
/// Main example, and the use case of this crate, would be `Option<&Stg>` as it is what gets
/// returned when calling `get` on an `HashMap`/`Account`
///
/// #Example
/// ```
/// # use hashmap_settings::{Account,Setting,Stg,StgTrait,types::errors::StgError};
///
/// //creating a Stg Account
/// let mut account = Account::<(), &str, Stg>::default();
///
/// //inserting values of distinct types
/// account.insert("Number of trees", 5.stg());
/// account.insert("Grass color", "green".to_string().stg());
/// account.insert("Today is good", true.stg());
///
/// //getting values from the account in 3 different ways
/// let today_bool: bool    = account.get(&"Today is good").unstg()?;
/// let grass_color: String = account.get(&"Grass color").unstg_panic();
/// let trees: i32          = account.get(&"Number of trees").unwrap().clone().unstg().unwrap();
/// //in the i32 example the last unwrap could be swapped for a "?" but it still would be a
/// //more complicated method chain than the other two alternatives.
///
/// //example of using the values
/// print!("It's {today_bool} that today is a wonderful day, the grass
///     is {grass_color} and I see {trees} trees in the distance");
/// # Ok::<(), StgError>(())
/// ```
///
pub trait StgTrait {
    /// Conversion to a Result<S, StgError>.
    ///
    /// Will return a [StgError] when the value isn't found, or when the value is found
    /// but isn't of the type that it is being converted to.
    ///
    /// # Errors
    ///
    /// This function can return [StgErrors](StgError).
    ///
    /// [None](StgError::None) when the value is not contained in the `T<Stg>`.
    /// [WrongType][StgError::WrongType] when the value is contained, but it was been tried
    /// to convert it to the wrong type
    ///
    /// # Examples
    ///
    /// ```
    /// # use hashmap_settings::{Account,Stg,StgTrait,Setting,types::errors::StgError};
    /// let mut account: Account<(),&str,Stg> = Default::default();
    /// account.insert("a small number", 42_i32.stg());
    /// assert_eq!(account.get(&"a small number").unstg::<i32>(), Ok(42));
    /// assert_eq!(account.get(&"a big number").unstg::<i32>(), Err(StgError::None));
    /// assert_eq!(account.get(&"a small number").unstg::<String>(), Err(StgError::WrongType));
    /// ```
    fn unstg<S: Setting>(self) -> Result<S, StgError>;
    /// Conversion to concrete type `S`, can panic.
    ///
    /// in the case the conversion can't be made, this method should panic.
    /// but this method should never be used if the conversion is not assured to be the correct one
    /// and [`unstg`](StgTrait::unstg) should be used instead.
    ///
    /// # Examples
    ///
    /// ```
    /// # use hashmap_settings::{Account,Stg,StgTrait,Setting};
    /// let mut account: Account<(),&str,Stg> = Default::default();
    /// account.insert("a small number", 42_i32.stg());
    /// assert_eq!(account.get(&"a small number").unstg_panic::<i32>(), 42);
    /// ```
    /// ```should_panic
    /// # use hashmap_settings::{Account,Stg,StgTrait,Setting};
    /// let mut account: Account<(),&str,Stg> = Default::default();
    /// account.insert("a small number", 42_i32.stg());
    /// assert_eq!(account.get(&"a small number").unstg_panic::<bool>(), true);//this panics
    /// ```
    #[must_use]
    fn unstg_panic<S: Setting>(self) -> S;
}

#[cfg(test)]
mod tests {
    use super::*;

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
