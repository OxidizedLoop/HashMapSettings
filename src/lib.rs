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
use types::{constants::*, errors::*};

/// A [`HashMap`]`<`[`String`],[`Box<dyn Setting>`]`>` with an associated name. May contain a [`Vec`] of other `Accounts`.
///
/// The `HashMap` contains all the `Box<dyn Setting>`s inside of all sub accounts.
///
/// All sub accounts, need to be uniquely named.
///
/// len()-1 of the `Vec` is the cache if one is created with [`cache`](Account::cache).
///
/// ```
/// # // todo!() add examples
/// ```
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Account {
    name: String,
    active: bool,
    settings: HashMap<String, Box<dyn Setting>>,
    //should contains all settings inside of accounts and its the default for this Account
    accounts: Vec<Account>,
    //list of all sub accounts, uniquely named.
    //len()-1 is the cache if one is created.
    //last element of the vec contains the most important setting, the one that will be used by the program.
    //cache must contain all settings at all times if it exists
}
impl Account {
    pub fn new(
        name: &str,
        active: bool,
        settings: HashMap<String, Box<dyn Setting>>,
        accounts: Vec<Account>,
    ) -> Self {
        //doesn't check if Account is valid,consider using new_valid instead if it isn
        Account {
            name: name.to_string(),
            active,
            settings,
            accounts,
        }
    }
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
        //valid means that Account don't have the same name as Siblings account,
        //and that if cache exists is at the end of the vector;
        //this checks children accounts as well
        let accounts = self.accounts_names();
        let size = accounts.len();
        let mut hash_set = HashSet::with_capacity(size);
        for account in accounts {
            if !hash_set.insert(account) {
                return Some(InvalidAccountError::ExistingName);
            }
        }
        if hash_set.get(CACHE).is_some() && self.accounts[size - 1].name() == CACHE {
            // if .is_some is false it won't check the second part
            return Some(InvalidAccountError::WronglyPositionedCache);
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
    /// let account : Account = Account::new("New account", Default::default(), Default::default(), Default::default());
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
    /// Takes a `&str` and updates the name of the `Account`
    ///
    /// returns a [`CacheError`] if the new name or old name are [`Cache`](CACHE)
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::{Account};
    /// let mut account : Account = Account::new("Old Name", Default::default(), Default::default(), Default::default());
    ///
    /// account.rename("New Name");
    /// assert_eq!(account.name(), "New Name");
    /// ```
    ///
    /// ```
    /// use hashmap_settings::{Account,types::errors::CacheError};
    /// let mut account : Account = Account::new("Old Name", Default::default(), Default::default(), Default::default());
    ///
    /// assert_eq!(account.name(), "Old Name");
    ///
    /// assert!(account.rename("Cache") == Some(CacheError::Naming));
    /// assert_ne!(account.name(), "Cache");
    /// assert_eq!(account.name(), "Old Name");
    /// ```
    pub fn rename(&mut self, new_name: &str) -> Option<CacheError> {
        if self.name() == CACHE {
            return Some(CacheError::Renaming);
        }
        if new_name == CACHE {
            return Some(CacheError::Naming);
        }
        self.name = new_name.to_string();
        None
    }
    pub fn deep_rename(
        &mut self,
        account_names: &mut Vec<&str>, //for each value, the value to its right is its parent.
        //left is the account we rename, right is the first child of the Account we call
        new_name: &str,
    ) -> Option<DeepChangeError> {
        let account_to_find = if let Some(account_name) = account_names.pop() {
            account_name
        } else {
            return Some(DeepChangeError::EmptyVec); //error if the original call is empty, but this will create the base case in the recursive call
        };
        match account_to_find {
            n if n == CACHE => Some(DeepChangeError::Cache(CacheError::Inserting)),
            n => {
                if let Some(found_account) = self.get_mut_from_name(n) {
                    match found_account.deep_rename(account_names, new_name) {
                        //recursive call
                        Some(error) => match error {
                            DeepChangeError::EmptyVec => {
                                found_account.rename(new_name).map(DeepChangeError::Cache)
                            } //base case
                            _ => Some(error), //error, impossible/invalid function call
                        },
                        None => None,
                    }
                } else {
                    Some(DeepChangeError::NotFound)
                }
            }
        }
    }
    #[allow(clippy::borrowed_box)]
    pub fn deep_get(
        &self,
        account_names: &mut Vec<&str>, //for each value, the value to its right is its parent.
        //left is the account we rename, right is the first child of the Account we call
        setting_name: &str,
    ) -> Result<Option<&Box<dyn Setting>>, DeepChangeError> {
        let account_to_find = if let Some(account_name) = account_names.pop() {
            account_name
        } else {
            return Err(DeepChangeError::EmptyVec); //error if the original call is empty, but this will create the base case in the recursive call
        };
        match account_to_find {
            n if n == CACHE => Err(DeepChangeError::Cache(CacheError::Inserting)),
            n => {
                if let Some(found_account) = self.get_from_name(n) {
                    match found_account.deep_get(account_names, setting_name) {
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
        }
    }
    fn get_from_name(&self, name: &str) -> Option<&Account> {
        for account in 0..self.len() {
            if self.accounts[account].name() == name {
                return Some(&self.accounts[account]);
            }
        }
        None
    }
    pub fn accounts_names(&self) -> Vec<&str> {
        self.accounts.iter().map(|a| a.name()).collect()
    }
    pub fn contains_cache(&self) -> bool {
        let size = self.accounts.len();
        if size > 0 && self.accounts[size - 1].name() == CACHE {
            // if size > 0 is false it won't check the second part so there is no out of bounds error
            return true;
        }
        false
    }
    pub fn cache_position(&self) -> Option<usize> {
        let size = self.accounts.len();
        if size > 0 && self.accounts[size - 1].name() == CACHE {
            // if size > 0 is false it won't check the second part so there is no out of bounds error
            return Some(size - 1);
        }
        None
    }
    /// Creates a `Cache` of the sub `Accounts`.
    ///
    /// Does nothing if account name is [`CACHE`],
    /// will update the cache if one already exists.
    ///
    /// A `Cache` is a sub `Account` with a name created from the const [`CACHE`]
    /// and it's located at main `Account`.[len()](Account::len)-1
    ///
    /// Having a `Cache` makes calling functions like [get()](Account::get) much faster
    /// as only `Cache` is checked instead of all sub `Accounts` in the `Vec`.
    ///
    /// Verify if `Cache` exists with [contains_cache](Account::contains_cache)
    /// and get it's position with [cache_position](Account::cache_position)
    ///
    /// # Panics
    ///
    /// This effectively pushes a new `Account` into the `Vec` so it
    /// panics if the new capacity exceeds `isize::MAX` bytes.
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
    ///         Account::new("1", true, Default::default(), Default::default()),
    ///         Account::new("2", true, Default::default(), Default::default())
    ///     ],
    /// );
    /// account.cache();
    /// assert!(account ==
    ///     Account::new(
    ///         Default::default(),
    ///         Default::default(),
    ///         Default::default(),
    ///         vec![
    ///             Account::new("1", true, Default::default(), Default::default()),
    ///             Account::new("2", true, Default::default(), Default::default()),
    ///             Account::new("Cache", true, Default::default(), Default::default())
    ///         ],
    ///     )
    /// )
    /// ```
    pub fn cache(&mut self) {
        if self.name() != CACHE {
            if !self.contains_cache() {
                self.accounts.push(Account::new(
                    CACHE,
                    true,
                    Default::default(),
                    Default::default(),
                ));
            }
            let cache_position = self.cache_position().unwrap();
            self.accounts[cache_position]
                .settings
                .reserve(self.settings.capacity()); //this assumes that Account.settings contains all settings and isn't empty
            for setting in self.settings.keys() {
                //this assumes that Account.settings contains all settings and isn't empty
                for account in (0..cache_position).rev() {
                    if self.accounts[account].active() {
                        if let Some(value) = self.accounts[account].get(setting) {
                            let temp = value.clone(); //to prevent cannot borrow `self.sub_accounts` as mutable because it is also borrowed as immutable Error
                            self.accounts[cache_position].insert(setting, temp);
                        } else {
                            self.accounts[cache_position].insert(
                                setting,
                                self.settings.get(setting).unwrap().clone(), //safe unwrap because we got "setting" from .keys()
                            );
                        }
                    }
                }
            }
        }
    }
    pub fn delete_cache(&mut self) {
        if self.contains_cache() {
            self.accounts.pop();
        }
    }
    pub fn len_without_cache(&self) -> usize {
        if self.contains_cache() {
            self.accounts.len() - 1
        } else {
            self.accounts.len()
        }
    }
    pub fn deep_insert(
        &mut self,
        account_names: &mut Vec<&str>, //for each value, the value to its right is its parent.
        //left is where we insert the value, right is the first child of the Account we call
        setting_name: &str,
        setting_value: Box<dyn Setting>,
    ) -> Result<Option<Box<dyn Setting>>, DeepChangeError> {
        let account_to_find = if let Some(account_name) = account_names.pop() {
            account_name
        } else {
            return Err(DeepChangeError::EmptyVec); //error if the original call is empty, but this will create the base case in the recursive call
        };
        match account_to_find {
            n if n == CACHE => Err(DeepChangeError::Cache(CacheError::Inserting)),
            n => {
                if let Some(found_account) = self.get_mut_from_name(n) {
                    match found_account.deep_insert(
                        account_names,
                        setting_name,
                        setting_value.clone(),
                    ) {
                        //recursive call
                        Ok(insert_option) => {
                            found_account.insert(setting_name, setting_value); //after the base this will be called in all previous function calls,
                                                                               //inserting the value in the corresponding Account.settings and caches
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
        }
    }
    fn get_mut_from_name(&mut self, name: &str) -> Option<&mut Account> {
        for account in 0..self.len() {
            if self.accounts[account].name() == name {
                return Some(&mut self.accounts[account]);
            }
        }
        None
    }
    /// Appends an `Account` to the back of the `Vec` of sub `Accounts`.
    ///
    /// Won't return an error if the sub `Account` being pushed is invalid
    /// but will cause unintended behavior for future calls to the main `Account`.
    /// Use [push](Account::push) if the Account might be invalid.
    /// //todo!() put a link to what means for an Account to be invalid
    ///
    /// This sub `Account` settings will be added to the settings of the main `Account` that `push` was called on.
    ///
    /// The `Cache` will always be at the end of the collection, so if the main `Account`
    /// [contains_cache](Account::contains_cache) then the sub `Account` will be inserted
    /// before the `Cache`. The `Cache ` will be updated with the new settings unless [active](Account::active) of sub Account is false.
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
        //doesn't check if Account being pushed is valid
        //if a invalid account is pushed it can cause unintended behavior when other functions are called
        if let Some(cache_position) = self.cache_position() {
            if account.active() {
                self.accounts.insert(cache_position, account.clone());
                for setting in account.settings.keys() {
                    if !account.contains_key(setting) {
                        self._insert(setting, account.get(setting).unwrap().clone());
                        self.accounts[cache_position]
                            ._insert(setting, account.get(setting).unwrap().clone());
                    } else {
                        self.update_cache_of_setting(setting);
                    }
                }
            } else {
                for setting in account.settings.keys() {
                    if !account.contains_key(setting) {
                        self._insert(setting, account.get(setting).unwrap().clone());
                    }
                }
                self.accounts.insert(cache_position, account);
            }
        } else {
            for setting in account.settings.keys() {
                if !account.contains_key(setting) {
                    self._insert(setting, account.get(setting).unwrap().clone());
                }
            }
            self.accounts.push(account);
        }
    }
    fn update_cache_of_setting(&mut self, name: &str) {
        //Cache and setting needs to exist, not pub fn so needs to be checked by developer instead of adding if statements making code slower
        let size = self.len();
        for account in (0..size - 1).rev() {
            if let Some(value) = self.accounts[account].get(name) {
                let temp = value.clone();
                self.accounts[size - 1].insert(name, temp);
                break;
            }
        }
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
    /// Internally [`get()`](Account::get()) is called on all sub `Accounts` of the `Vec`
    /// starting at the end, followed by calling [`get()`](HashMap::get()) on the main `Account` `settings`.
    /// Will return `Some`([Box<dyn Setting>]) when found.
    ///
    /// If there is a significant number of sub accounts it is recommend to create a `Cache` with [`cache()`](Account::cache) to improve performance.
    /// Then there will be only one call of [`get()`](HashMap::get()) to `Cache` to obtain the desired [Box<dyn Setting>].
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// This method ends on a call to a [`HashMap`]'s [`get()`](HashMap::get()).
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
        if let Some(position) = self.cache_position() {
            return self.accounts[position].get(setting_name);
        }
        for account in (0..self.len()).rev() {
            if self.accounts[account].active {
                if let Some(value) = self.accounts[account].get(setting_name) {
                    return Some(value);
                }
            }
        }
        return self._get(setting_name);
    }
    pub fn insert(
        &mut self,
        setting_name: &str,
        setting_value: Box<dyn Setting>,
    ) -> Option<Box<dyn Setting>> {
        let mut return_value = None;
        if let Some(value) = self._insert(setting_name, setting_value.clone()) {
            return_value = Some(value);
        }
        if self.contains_cache() {
            self.update_cache_of_setting(setting_name);
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
    pub fn len(&self) -> usize {
        self.accounts.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Appends an `Account` to the back of the `Vec` of sub `Accounts`.
    ///
    /// Will return an error if the sub `Account` being pushed is invalid or would make the main `Account` invalid.
    /// Use [push_unchecked](Account::push_unchecked) for better performance if its guaranteed that `Account` is valid.
    /// //todo!() put a link to what means for an Account to be valid/invalid
    ///
    /// This sub `Account` settings will be added to the settings of the main `Account` that `push` was called on.
    ///
    /// The `Cache` will always be at the end of the collection, so if the main `Account`
    /// [contains_cache](Account::contains_cache) then the sub `Account` will be inserted
    /// before the `Cache`. The `Cache ` will be updated with the new settings unless [active](Account::active) of sub Account is false.
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
        if account.name() == CACHE {
            //check if account isn't named Cache
            return Some(InvalidAccountError::Cache(CacheError::Inserting));
        }
        if self.accounts_names().contains(&account.name()) {
            //check if account has the same name as a sibling account
            return Some(InvalidAccountError::ExistingName);
        }
        if let Some(error) = account.is_invalid() {
            //check if Account is internally valid
            return Some(error);
        }
        if let Some(cache_position) = self.cache_position() {
            if account.active() {
                self.accounts.insert(cache_position, account.clone());
                for setting in account.settings.keys() {
                    if !account.contains_key(setting) {
                        self._insert(setting, account.get(setting).unwrap().clone());
                        self.accounts[cache_position]
                            ._insert(setting, account.get(setting).unwrap().clone());
                    } else {
                        self.update_cache_of_setting(setting);
                    }
                }
            } else {
                for setting in account.settings.keys() {
                    if !account.contains_key(setting) {
                        self._insert(setting, account.get(setting).unwrap().clone());
                    }
                }
                self.accounts.insert(cache_position, account);
            }
        } else {
            for setting in account.settings.keys() {
                if !account.contains_key(setting) {
                    self._insert(setting, account.get(setting).unwrap().clone());
                }
            }
            self.accounts.push(account);
        }
        None
    }
    /// Removes the last element from a vector and returns it, or [`None`] if it
    /// is empty.
    ///
    /// Will not pop `Cache` if there is one, but will pop the next sub `Account`. `Cache` values will be updated.
    ///
    /// Use [pop_remove](Account::pop_remove) if you intend to remove settings from the main `Account` present only on the popped sub `Account`.
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
        if let Some(position) = self.cache_position() {
            if position == 0 {
                return None;
            }
            let popped_account = self.accounts.remove(position - 1);
            if popped_account.active() {
                for setting in popped_account.keys() {
                    self.update_cache_of_setting(setting)
                }
            }
            Some(popped_account)
        } else {
            self.accounts.pop()
        }
    }
    /// Removes the last element from a vector and returns it, or [`None`] if it empty.
    ///
    /// Will not pop `Cache` if there is one, but will pop the next child `Account`. `Cache` values will be updated.
    ///
    /// Will remove settings from the main `Account` present only on the popped child `Account`.
    /// Use [pop](Account::pop) if you want the main `Account` settings to remain unchanged.
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
    /// account.pop_remove();
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
    pub fn pop_remove(&mut self) -> std::option::Option<Account> {
        let popped_account = if let Some(position) = self.cache_position() {
            if position == 0 {
                return None;
            }
            let popped_account = self.accounts.remove(position - 1);
            if popped_account.active() {
                for setting in popped_account.keys() {
                    self.update_cache_of_setting(setting)
                }
            }
            popped_account
        } else if let Some(popped_account) = self.accounts.pop() {
            popped_account
        } else {
            return None;
        };
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
                return true;
            }
        }
        false
    }
    ///todo!()
    pub fn get_mut_account(&mut self, index: usize) -> Option<&mut Account> {
        self.accounts.get_mut(index)
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
    fn _get(&self, setting_name: &str) -> Option<&Box<dyn Setting>> {
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
    fn _insert(
        &mut self,
        setting_name: &str,
        setting_value: Box<dyn Setting>,
    ) -> Option<Box<dyn Setting>> {
        self.settings
            .insert(setting_name.to_string(), setting_value)
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
        account._insert("bool_setting", Box::new(bool_setting));
        account._insert("i32_setting", i32_setting.stg());
        let i32s: i32 = unstg(account._get("i32_setting").unwrap().clone());
        assert_eq!(i32s, 42);
        let stg: Box<dyn Setting> = account._get("bool_setting").unwrap().clone();
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
        account1._insert("answer to everything", 42.stg());
        account1._insert("true is true", true.stg());
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
