
///module including `Incrementable` implementations activated by the default feature "incrementable"
#[cfg(feature = "incrementable")]
pub mod incrementable_implementations;

use core::{fmt::Debug, mem::replace};
use std::{
    collections::{hash_map, HashMap, HashSet},
    hash::Hash,
    option::Option,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::stg::Setting;

/// A [`HashMap`] wrapper for layered settings.
///
/// The [`Stg`](crate::stg::Stg) type is a type abstraction that can be used to to have an `Account` with distinct types.
///
/// An `Account<N,K,S>` can also hold other [Accounts](Account#accounts). This allows for complex systems where
/// an app can have multiple layers of settings. The top most layer being the first one to be searched
/// for a specific setting, and in the case it isn't found the next layer will be search, this will be
/// done until the setting is found on the last layer that would be the default layer containing all the settings.
///
///
/// An `Account` contains the following fields:
///
///
///  - [name](Account#name): Name of type `N` ,
///
///  - [active](Account#active): [`bool`],
///
///  - [settings](Account#settings): A [`HashMap`]<`K`,`V`>,
///
///  - [accounts](Account#accounts): A [`Vec`]<`Account`>, of sub `Accounts`
///
///  - [valid](Account#valid): A [`Valid`] the `Account's` validity tracker
///
///
/// # New Account
///
///
/// A new Account can be created with:
///  - [`new`](Account::new): Create a new Account.
///
///  - [`new_unchecked`](Account::new_unchecked): Creates a new account without verifying its validity.
///
///  - [`clone`][Clone::clone]: Clone an existing Account.
///
/// An `AccountBuilder` is planned to be created in the [future](https://github.com/OxidizedLoop/HashMapSettings/issues/20).
///
///
/// # [Name](Account#name)
///
///
/// An `Account's` name is used to identify an Account in multiple methods involving [child](Account#accounts) `Accounts`.
///
/// For this reason child `Accounts` need to be uniquely named for the parent `Account` to be [valid](Account#valid).
///
///
///  - [`name`](Account::name): Get an account's name
///
///  - [`rename`](Account::rename): Rename an `Account`
///
///  - [`deep_rename`](Account::deep_rename): Rename a [child](Account#accounts) `Account`
///
///
/// # [Active](Account#active)
///
///
/// If a child `Account` is inactive it's settings will be ignore by the parent `Account`.
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
///  - [`hashmap`](Account::hashmap): Returns a reference to [`HashMap`].
///
///  - [`get`](Account::get): Returns a reference to the value corresponding to the key
///
///  - [`insert`](Account::insert): Inserts a key-value pair into the map.
///
///  - [`deep_insert`](Account::deep_insert): Inserts a key-value pair into the map of a child Account.
///
///  - [`remove`](Account::remove): Removes a key-value pair from the map.
///
///  - [`deep_remove`](Account::deep_remove): Removes a key-value pair from the map of a child Account.
///
///  - [`keys`](Account::keys): An iterator visiting all keys in arbitrary order
///
///  - [`contains_key`](Account::contains_key): Returns `true` if the `Account` contains a value for the specified key.
///
///  - [`capacity`](Account::capacity): Returns the number of elements the map can hold without reallocating.
///
///  - [`update_setting`](Account::update_setting): Updates a setting with the value its supposed to have.
///
///  - [`update_setting`](Account::update_setting_returns): Updates a setting with the value its supposed to have and.
/// 
///  - [`update_setting`](Account::update_vec): Updates a group of settings with the value they are supposed to have.
/// 
///  - [`update_setting`](Account::update_all_settings): Updates all settings currently present in the Account with the value they are supposed to have.
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
///  - [`accounts`](Account::accounts): Return a `Vec` of names of the child `Accounts`.
///
///  - [`len`](Account::len): Returns the number of elements in the `Vec`.
///
///  - [`is_empty`](Account::is_empty): Returns `true` if the `Vec` contains no elements.
///
///  - [`push`](Account::push): Appends an `Account` to the back of the `Vec`.
///
///  - [`deep_push`](Account::deep_push): Appends an `Account` to the back of the `Vec`of a child `Account`.
///
///  - [`pop`](Account::pop): Removes the last element from a vector and returns it, or [`None`] if it is empty.
///
///  - [`deep_pop`](Account::deep_pop): Removes the last element from a vector of a child `Account`
///
///
/// # [Valid](Account#valid)
///  
/// 
/// A valid `Account` is one where it's methods will always behave as intended.
///
/// There are certain methods that may make an Account invalid if improperly used,
/// and that would make other methods have unindent effects.
///
/// If a method can make an `Account` invalid it will be mentioned.
///
///
/// ## Validity Defined:
///
///
/// `Account` contains a valid field of type [`Valid`] that tracks if an `Account` is valid.
///
/// `Valid` contains 3 `bool` fields corresponding to the 3 ways an account can be invalid:
///
/// names: An `Account` is invalid if it's children `Accounts` have duplicated names.
///
/// settings: An `Account` is invalid if it doesn't contain all settings present in it's children `Accounts`.
///
/// accounts: An `Account` is invalid if it's children `Accounts` are themselves invalid.
///
/// If all the fields are true then the `Account` is valid.
///
/// An Account can be temporary made invalid  for either efficiency (using [push](Account::push) or [pop](Account::pop) repeatedly) 
/// or while using [deep_mut](Account::deep_mut) for something that isn't covered by the other [deep functions](Account#deep-functions).
/// But this should be fixed by 
///
/// -[valid](Account::valid): Returns a reference to the `Account` `valid` field.
///
/// -[update_valid](Account::update_valid): Updates `valid` to the values it's supposed to have.
///
/// -[change_valid](Account::change_valid): Changes the `valid` to a the provided `Valid`
///
/// -[fix_valid](Account::fix_valid): Makes an invalid Account valid
///
///
/// # [Deep Functions](Account#deep-functions)
///
///
/// Deep Functions are versions of functions to interact with a child `Account`
/// of the parent `Account` that the function is called.
///
/// They accept an extra `Vec` of `&N` that are the list of child `Accounts`
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
#[cfg_attr(feature = "serde", derive(Serialize))]
#[must_use]
pub struct Account<N, K, V> {
    name: N,
    active: bool,
    settings: HashMap<K, V>,
    accounts: Vec<Account<N, K, V>>,
    valid: Valid,
}

impl<N, K, V> Account<N, K, V> {
    /// Creates a new account without verifying its validity
    ///
    /// The is no [validity](Account#valid) check, so the account created can be an invalid account.
    /// Use [`new`](Account::new) to make sure that the account created is valid.
    ///
    /// It's recommend that the parent `Accounts` are made with [`new`](Account::new)
    /// but child `Accounts` are made with with `new_unchecked`.
    ///
    /// # Example
    /// ```
    /// use std::collections::HashMap;
    /// use hashmap_settings::account::*;
    /// let account = Account::new_unchecked(
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
    ///     Valid::new(true,true,true),
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
    /// assert!(account.valid() ==
    ///     &Valid::new(true,true,true)
    /// );
    ///
    /// ```
    pub fn new_unchecked(
        name: N,
        active: bool,
        settings: HashMap<K, V>,
        accounts: Vec<Self>,
        valid: Valid,
    ) -> Self {
        Self {
            name,
            active,
            settings,
            accounts,
            valid,
        }
    }
    /// Returns the name of the `Account`
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::account::Account;
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
    /// Return `true` if the `Account` is active
    ///
    /// When not active `Accounts` will be treated as if they were not there when called by some of the parent's `Account` methods.
    ///
    /// When creating an `Account` with [`Default`] active will be `true`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::account::Account;
    /// let mut account = Account::<String,(),()>::new(Default::default(), true, Default::default(), Default::default());
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
    /// use hashmap_settings::account::Account;
    /// let mut account = Account::<String,(),()>::new(Default::default(), false, Default::default(), Default::default());
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
    /// Return a reference to the `HashMap`
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::account::Account;
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
        &self.settings
    }
    /// An iterator visiting all keys in arbitrary order.
    /// The iterator element type is `&'a K`.
    ///
    /// This method is a direct call to [`HashMap`]'s [`keys()`](HashMap::keys()).
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::account::Account;
    /// use std::collections::HashMap;
    /// let account = Account::<String,String,i32>::new(
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
        self.settings.keys()
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
    /// use hashmap_settings::account::Account;
    /// use std::collections::HashMap;
    /// let account = Account::<String,(),()>::new(Default::default(), Default::default(), HashMap::with_capacity(100), Default::default());
    /// assert!(account.capacity() >= 100);
    /// ```
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.settings.capacity()
    }
    /// Return a reference to the `Vec` of child `Accounts`
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::account::Account;
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
    /// Return a `Vec` of names of the child `Accounts`
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::account::Account;
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
    /// Returns the number of elements in the `Vec` of child `Accounts`,
    /// also referred to as its 'length'.
    ///
    /// This method is a direct call to [`Vec`]'s [`len()`](Vec::len()).
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::account::Account;
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
    /// use hashmap_settings::account::{Account,Valid};
    /// let mut account = Account::<String,(),()>::default();
    /// assert!(account.is_empty());
    ///
    /// account.push(Account::<String,(),()>::default(), Valid::new_true());
    /// assert!(!account.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.accounts.is_empty()
    }
    /// Returns a mutable reference to a child `Account`
    ///  
    /// # Examples
    /// ```
    ///  //TODO(Example)
    /// ```
    #[must_use]
    pub fn get_mut_account(&mut self, index: usize) -> Option<&mut Self> {
        self.accounts.get_mut(index)
    }
    fn update_valid_children(&self) -> bool {
        for account in self.accounts() {
            if !account.valid.is_valid() {
                return false;
            }
        }
        true
    }
    /// Returns a reference to the `Account`'s [Valid].
    ///
    /// Check [valid](Account#valid)
    ///
    /// # Examples
    /// ```
    /// use hashmap_settings::account::{Account,Valid};
    /// let account = Account::<(),(),()>::default();
    /// assert_eq!(account.valid(),&Valid::new(true,true,true));
    /// ```
    pub const fn valid(&self) -> &Valid {
        &self.valid
    }
    /// Change `Account`'s [Valid] to the provided value.
    ///
    /// Returns `true` if new value is different than the previous one.
    ///
    /// This method (along with [update_valid](Account::update_valid)) is intended to be used with methods that
    /// can make an account [invalid](Account#valid) to correctly update they values for a future use of
    /// [fix_valid](Account::fix_valid).
    ///
    ///  `change_valid` can make an account [invalid](Account#valid) if improperly used.
    ///
    /// # Examples
    /// ```
    /// use hashmap_settings::account::{Account,Valid};
    /// let mut account = Account::<(),(),()>::default();
    /// assert_eq!(account.valid(),&Valid::new(true,true,true));
    /// assert_eq!(account.change_valid(Valid::new(true,true,true)),false);
    /// assert_eq!(account.change_valid(Valid::new(false,true,true)),true);
    /// assert_eq!(account.valid(),&Valid::new(false,true,true));
    /// ```
    pub fn change_valid(&mut self, new_valid: Valid) -> bool {
        new_valid != replace(&mut self.valid, new_valid)
    }
    /// Takes a `N` and updates the name of the `Account`.
    ///
    /// Returns the previous name that the Account had.
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::account::Account;
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
        core::mem::replace(&mut self.name, new_name)
    }
}
impl<N: Eq + Hash, K, V> Account<N, K, V> {
    fn update_valid_names(&self) -> bool {
        let accounts = self.accounts_names();
        let size = accounts.len();
        let mut hash_set = HashSet::with_capacity(size);
        for account in accounts {
            if !hash_set.insert(account) {
                return false;
            }
        }
        true
    }
}
impl<N: PartialEq, K, V> Account<N, K, V> {
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
    /// use hashmap_settings::account::Account;
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
    /// # Ok::<(), hashmap_settings::account::DeepError>(())
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
                        DeepError::NotFound => Err(error),        //error/bad function call
                    },
                    Ok(value) => Ok(value),
                },
            )
    }
    /// Returns a mutable reference to a child `Account`.
    ///
    /// Consider using [`deep`](Account::deep) with methods that don't need a `&mut self`,
    /// or the respective [deep_function](Account#deep-functions) for a specific method as
    /// `deep_mut` can make an account [invalid](Account#valid)
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
    /// use hashmap_settings::account::Account;
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
    /// # Ok::<(), hashmap_settings::account::DeepError>(())
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
                    DeepError::NotFound => Err(error), //error/bad function call
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
    fn mut_account_from_name(&mut self, name: &N) -> Option<&mut Self> {
        for account in 0..self.len() {
            if self.accounts[account].name() == name {
                return Some(&mut self.accounts[account]);
            }
        }
        None
    }
}
impl<N, K: Eq + Hash, V> Account<N, K, V> {
    /// Returns the value corresponding to the key.
    ///
    /// This method is a direct call to [`HashMap`]'s [`get()`](HashMap::get).
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::account::Account;
    /// let mut account: Account<(),&str,i32> = Default::default();
    /// account.insert("a small number", 42);
    /// assert_eq!(account.get(&"a small number"), Some(&42));
    /// assert_eq!(account.get(&"a big number"), None);
    /// ```
    #[must_use]
    #[allow(clippy::borrowed_box)]
    pub fn get(&self, setting_name: &K) -> Option<&V> {
        self.settings.get(setting_name)
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
    /// use hashmap_settings::account::Account;
    /// let mut account: Account<(),&str,i32> = Default::default();
    /// assert_eq!(account.insert("a small number", 1), None);
    /// assert_eq!(account.hashmap().is_empty(), false);
    ///
    /// account.insert("a small number", 2);
    /// assert_eq!(account.insert("a small number", 3), Some(2));
    /// assert!(account.hashmap()[&"a small number"] == 3);
    /// ```
    pub fn insert(&mut self, setting_name: K, setting_value: V) -> Option<V> {
        self.settings.insert(setting_name, setting_value)
    }
    /// Removes a setting from the map, returning the value at the key if the key was previously in the map.
    ///
    /// This method is a direct call to [`HashMap`]'s [`remove()`](HashMap::remove).
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::account::Account;
    /// let mut account: Account<(),&str,i32> = Default::default();
    /// assert_eq!(account.insert("a small number", 1), None);
    /// assert_eq!(account.remove(&"a small number"), Some(1));
    /// assert_eq!(account.remove(&"a small number"), None);
    /// ```
    pub fn remove(&mut self, setting_to_remove: &K) -> Option<V> {
        self.settings.remove(setting_to_remove)
    }
    /// Returns `true` if the `Account` contains a value for the specified key.
    ///
    /// The key may be any borrowed form of the map’s key type, but [`Hash`] and [`PartialEq`] on the borrowed form must match those for the key type.
    ///
    /// This method is a direct call to [`HashMap`]'s [`contains_key()`](HashMap::contains_key()) .
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::account::Account;
    /// let mut account: Account<(),&str,i32> = Default::default();
    /// account.insert("a small number", 42);
    /// assert_eq!(account.contains_key(&"a small number"), true);
    /// assert_eq!(account.contains_key(&"a big number"), false);
    /// ```
    #[must_use]
    pub fn contains_key(&self, setting_name: &K) -> bool {
        self.settings.contains_key(setting_name)
    }
    fn get_in_sub_accounts(&self, setting: &K) -> Option<&V> {
        for account in (0..self.len()).rev() {
            if self.accounts[account].active {
                if let Some(value) = self.accounts[account].settings.get(setting) {
                    return Some(value);
                }
            }
        }
        None
    }
}
impl<N, K: Eq + Hash, V: PartialEq> Account<N, K, V> {
    fn update_valid_settings(&self) -> bool {
        let mut hash_set = HashSet::new();
        for account in self.accounts() {
            if !account.valid.settings() {
                return false;
            }
            if account.active {
                for setting in account.keys() {
                    hash_set.insert(setting);
                }
            }
        }
        for setting in hash_set {
            if self.get_in_sub_accounts(setting) != self.get(setting) {
                return false;
            };
        }
        true
    }
}
impl<N, K: Clone + Eq + Hash, V: Clone> Account<N, K, V> {
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
    ///  //TODO(Example)
    /// ```
    pub fn update_setting(&mut self, setting: &K) {
        for account in (0..self.len()).rev() {
            if self.accounts[account].active {
                if let Some(value) = self.accounts[account].settings.get(setting) {
                    self.settings.insert(setting.to_owned(), value.clone());
                    return;
                }
            }
        }
        self.settings.remove(setting);
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
    ///  //TODO(Example)
    /// ```
    pub fn update_vec(&mut self, settings: &Vec<&K>) {
        'setting: for setting in settings {
            for account in (0..self.len()).rev() {
                if self.accounts[account].active {
                    if let Some(value) = self.accounts[account].settings.get(*setting) {
                        self.settings.insert((*setting).to_owned(), value.clone());
                        continue 'setting;
                    }
                }
            }
            self.settings.remove(*setting);
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
    ///  //TODO(Example)
    /// ```
    pub fn update_all_settings(&mut self) {
        let settings = self
            .settings
            .keys()
            .map(std::borrow::ToOwned::to_owned)
            .collect::<Vec<_>>();
        'setting: for setting in settings {
            for account in (0..self.len()).rev() {
                if self.accounts[account].active {
                    if let Some(value) = self.accounts[account].settings.get(&setting.clone()) {
                        self.settings.insert(setting.clone(), value.clone());
                        continue 'setting;
                    }
                }
            }
            self.settings.remove(&setting);
        }
    }
    fn fix_valid_settings(&mut self) {
        for account in &mut self.accounts {
            //fix child Accounts
            if !account.valid.settings {
                account.fix_valid_settings();
            }
        }
        let mut all_settings = HashSet::new();
        for account in self.accounts() {
            //get set off all settings
            if account.active {
                for setting in account.keys() {
                    all_settings.insert(setting.clone());
                }
            }
        }
        'setting: for setting in all_settings {
            //update settings on self account
            for account in (0..self.len()).rev() {
                if self.accounts[account].active {
                    if let Some(value) = self.accounts[account].settings.get(&setting) {
                        self.settings.insert(setting, value.clone());
                        continue 'setting;
                    }
                }
            }
        }
        self.valid.settings = true;
    }
}
impl<N: Eq + Hash, K: Eq + Hash, V: PartialEq> Account<N, K, V> {
    /// Updates `valid` to the values it's supposed to have.
    ///
    /// This method takes a [Valid], updating the `Account`'s [Valid] accordingly.
    ///
    /// This method (along with [change_valid](Account::change_valid)) is intended to be used with methods that
    /// can make an account [invalid](Account#valid) to correctly update they values for a future use of
    /// [fix_valid](Account::fix_valid).
    ///  
    /// # Examples
    /// ```
    ///  //TODO(Example)
    /// ```
    pub fn update_valid(&mut self, valid: Valid) {
        if valid.names {
            self.valid.names = self.update_valid_names();
        }
        if valid.children {
            self.valid.children = self.update_valid_children();
        }
        if valid.settings {
            self.valid.settings = self.update_valid_settings();
        }
    }
}
impl<N: Clone + Eq + Hash + Incrementable, K, V> Account<N, K, V> {
    fn fix_valid_names(&mut self) {
        //todo!(performance needs to be improved)
        let size = self.accounts.len();
        let mut hash_set = HashSet::with_capacity(size);
        let mut vec_names: Vec<(N, usize)> = vec![];
        for account in 0..size {
            if !hash_set.insert(self.accounts[account].name.clone()) {
                vec_names.push((self.accounts[account].name.clone(), account));
            }
        }
        for name in &mut vec_names {
            name.0.increment_mut();
            'looping: loop {
                if hash_set.insert(name.0.clone()) {
                    self.accounts[name.1].name = name.0.clone();
                    break 'looping;
                }
                name.0.increment_mut();
            }
        }
        self.valid.names = true;
    }
}
impl<N: PartialEq, K: Clone + Eq + Hash, V: Clone> Account<N, K, V> {
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
    /// use hashmap_settings::account::Account;
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
    /// use hashmap_settings::account::Account;
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
    /// # Ok::<(), hashmap_settings::account::DeepError>(())
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
            if account_names.is_empty() {
                //this and the unreachable()! have been added to prevent a .clone() on setting_value
                return Ok(found_account.insert(setting_name.to_owned(), setting_value));
            }
            match found_account.deep_insert(setting_name, setting_value, account_names) {
                //recursive call
                Ok(insert_option) => {
                    self.update_setting(setting_name);
                    //after the base this will be called in all previous function calls,
                    //updating the value in the corresponding Account.settings
                    Ok(insert_option) //returning the original value from the base case
                }
                Err(error) => match error {
                    DeepError::EmptyVec => {
                        unreachable!()
                    } //base case
                    DeepError::NotFound => Err(error), //error/bad function call
                },
            }
        } else {
            Err(DeepError::NotFound)
        }
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
    /// use hashmap_settings::account::Account;
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
    /// # Ok::<(), hashmap_settings::account::DeepError>(())
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
                    DeepError::NotFound => Err(error), //error/bad function call
                },
            }
        } else {
            Err(DeepError::NotFound)
        }
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
                    DeepError::NotFound => (Err(error), vec![]), //error/bad function call
                },
            }
        } else {
            (Err(DeepError::NotFound), vec![])
        }
    }
}
impl<N, K: Clone + Eq + Hash, V: Clone + PartialEq> Account<N, K, V> {
    /// Updates a setting with the value its supposed to have.
    ///
    /// Returns `None` if the setting isn't present in the Account or child Accounts.
    /// Returns `Some(true)` if the value of the setting was updated.
    /// Returns `Some(false)` if the value is in the Account but was not updated.
    ///
    /// if you don't need the return value use [update_setting](Account::update_setting) as it is faster
    ///
    /// If an Account is [valid](Account#valid) this method never returns Some(true)
    /// as this method is used to turn an invalid Account into a valid one.
    ///
    /// # Examples
    /// ```
    ///  //TODO(Example)
    /// ```
    #[must_use]
    pub fn update_setting_returns(&mut self, setting: &K) -> Option<bool> {
        for account in (0..self.len()).rev() {
            if self.accounts[account].active {
                if let Some(value) = self.accounts[account].settings.get(setting) {
                    return Some(
                        !self
                            .settings
                            .insert(setting.to_owned(), value.clone())
                            .map_or(false, |x| &x == value),
                    );
                }
            }
        }
        self.settings.remove(setting).map(|_| true)
    }
}
impl<N: Clone + Eq + Hash + Incrementable + PartialEq, K, V> Account<N, K, V> {
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
    /// use hashmap_settings::account::Account;
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
    /// assert_eq!(account.deep_rename(&"Cool Name".to_string(),&mut vec![&"3_2".to_string(),&"3".to_string()]), Ok("3_2".to_string()));
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
        new_name: &N,
        account_names: &mut Vec<&N>,
    ) -> Result<N, DeepError> {
        let Some(account_to_find) = account_names.pop() else {
            return Err(DeepError::EmptyVec); //error if the original call is empty, but this will create the base case in the recursive call
        };
        #[allow(clippy::option_if_let_else)]
        if let Some(found_account) = self.mut_account_from_name(account_to_find) {
            match found_account.deep_rename(new_name, account_names) {
                //recursive call
                Ok(insert_option) => {
                    Ok(insert_option) //returning the original value from the base case
                }
                Err(error) => match error {
                    DeepError::EmptyVec => {
                        let n = found_account.rename(new_name.clone());
                        self.fix_valid_names();
                        Ok(n)
                    } //base case
                    DeepError::NotFound => Err(error), //error/bad function call
                },
            }
        } else {
            Err(DeepError::NotFound)
        }
    }
}
impl<N: Eq + Hash, K: Clone + Eq + Hash, V: Clone + PartialEq> Account<N, K, V> {
    /// Removes the last element from the [`Vec`] of child `Account`s and returns it, or [`None`] if it is empty.
    ///
    /// Will update the settings from the parent `Account` present on the popped child `Account`.
    /// Consider using [pop_unchecked](Account::pop_unchecked) if you are removing multiple child `Accounts`.
    ///
    ///
    /// This method contains a call to [`Vec`]'s [`pop()`](Vec::pop()).
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::account::{Account,Valid};
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
    /// assert_eq!(account.pop(Valid::new_true()), Some(Account::new(3, Default::default(), Default::default(), Default::default())));
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
    pub fn pop(&mut self, valid: Valid) -> Option<Self> {
        let popped_account = self.accounts.pop()?;
        if !self.valid.names && valid.names {
            self.valid.names = self.update_valid_names();
        }
        if !self.valid.children && valid.children {
            self.valid.children = self.update_valid_children();
        }
        if !self.valid.settings && valid.settings && popped_account.active {
            self.update_vec(&popped_account.keys().collect());
            self.valid.settings = self.update_valid_settings();
        }
        Some(popped_account)
    }
    /// Removes the last element from the [`Vec`] of child `Account`s, from a child `Account,`and returns it, or [`None`] if it is empty.
    ///
    /// This will updated the [settings](Account#settings) of all necessary Accounts
    /// so that the parent Account remains [valid](Account#valid)
    ///
    /// Part of the [deep functions](Account#deep-functions) group that accept a `Vec` of &N to identify
    /// the child `Account` to run the function. [`pop`](Account::pop) in this case.
    ///
    /// # Errors
    ///
    /// Deep functions can return [`DeepError`]'s
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::account::{Account,Valid};
    /// let mut account = Account::<String,(),()>::new(
    ///     "Old Name".to_string(),
    ///     Default::default(),
    ///     Default::default(),
    ///     vec![
    ///         Account::new("1".to_string(), true, Default::default(), Default::default()),
    ///         Account::new("2".to_string(), true, Default::default(), Default::default()),
    ///         Account::new("3".to_string(), true, Default::default(), vec![
    ///             Account::new("3_1".to_string(), true, Default::default(), Default::default()),
    ///             Account::new("3_2".to_string(), true, Default::default(), vec![
    ///                     Account::new("3_2.1".to_string(), true, Default::default(), Default::default()),
    ///                 ]),
    ///             Account::new("3_3".to_string(), true, Default::default(), Default::default()),
    ///         ])
    ///     ],
    /// );
    ///
    /// assert_eq!(account.deep_pop(Valid::new_true(), &mut vec![&"3_2".to_string(),&"3".to_string()])
    ///     ,Ok(Some(Account::new("3_2.1".to_string(), true, Default::default(), Default::default())))
    /// );
    ///
    ///
    /// assert_eq!(account ,
    ///     Account::<String,(),()>::new(
    ///         "Old Name".to_string(),
    ///         Default::default(),
    ///         Default::default(),
    ///         vec![
    ///             Account::new("1".to_string(), true, Default::default(), Default::default()),
    ///             Account::new("2".to_string(), true, Default::default(), Default::default()),
    ///             Account::new("3".to_string(), true, Default::default(), vec![
    ///                 Account::new("3_1".to_string(), true, Default::default(), Default::default()),
    ///                 Account::new("3_2".to_string(), true, Default::default(), Default::default()),
    ///                 Account::new("3_3".to_string(), true, Default::default(), Default::default()),
    ///             ])
    ///         ],
    ///     )
    /// );
    ///
    ///
    /// # Ok::<(), hashmap_settings::account::DeepError>(())
    /// ```
    pub fn deep_pop(
        &mut self,
        valid: Valid,
        account_names: &mut Vec<&N>,
    ) -> Result<Option<Self>, DeepError> {
        let Some(account_to_find) = account_names.pop() else {
            return Err(DeepError::EmptyVec); //error if the original call is empty, but this will create the base case in the recursive call
        };
        #[allow(clippy::option_if_let_else)]
        if let Some(found_account) = self.mut_account_from_name(account_to_find) {
            match found_account.deep_pop(valid, account_names) {
                //recursive call
                Ok(popped_account) => {
                    Ok(match popped_account {
                        None => None,
                        Some(account) => {
                            if account.active {
                                self.update_vec(&account.keys().collect());
                            }
                            Some(account)
                        }
                    })
                    //returning the original value from the base case
                }
                Err(error) => match error {
                    DeepError::EmptyVec => Ok(found_account.pop(valid)), //base case
                    DeepError::NotFound => Err(error),                   //error/bad function call
                },
            }
        } else {
            Err(DeepError::NotFound)
        }
    }
}
impl<N: Clone + Eq + Hash + Incrementable, K: Clone + Eq + Hash, V: Clone + PartialEq>
    Account<N, K, V>
{
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
    /// use hashmap_settings::account::Account;
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
    /// assert_eq!(account, Account::<String,(),()>::new(
    ///     "New Account".to_string(),
    ///     Default::default(),
    ///     Default::default(),
    ///     vec![
    ///         Account::new("1".to_string(), true, Default::default(), Default::default()),
    ///         Account::new("2".to_string(), true, Default::default(), Default::default()),
    ///         Account::new("3".to_string(), true, Default::default(), Default::default())
    ///     ],
    /// ));
    pub fn new(name: N, active: bool, settings: HashMap<K, V>, accounts: Vec<Self>) -> Self {
        let mut new_account = Self {
            name,
            active,
            settings,
            accounts,
            valid: Valid::new_false(),
        };
        new_account.fix_valid(Valid::new_true());
        new_account
    }
    /// Makes an invalid `Account` valid
    ///
    /// This method fixes the `Account` according to the specified [`Valid`] `Account` to make it [valid](Account#valid)
    ///
    /// This method is called by [new](Account::new) when an Account is created.
    ///
    /// # Examples
    /// ```
    ///  //TODO(Example)
    /// ```
    pub fn fix_valid(&mut self, valid: Valid) {
        if self.valid.is_valid() && valid.is_valid() {
            return;
        }
        if !self.valid.children && valid.children {
            self.fix_valid_children();
        }
        if !self.valid.names && valid.names {
            self.fix_valid_names();
        }
        if !self.valid.settings && valid.settings {
            self.fix_valid_settings();
        }
    }

    fn fix_valid_children(&mut self) {
        for account in 0..self.len() {
            if !self.accounts[account].valid.is_valid() {
                self.accounts[account].fix_valid(Valid::default());
            }
        }
        self.valid.children = true;
    }
}
impl<
        N: Clone + Eq + Hash + Incrementable + PartialEq,
        K: Clone + Eq + Hash,
        V: Clone + PartialEq,
    > Account<N, K, V>
{
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
    /// use hashmap_settings::account::{Account,Valid};
    /// let mut account = Account::<i32,(),()>::new(
    ///     Default::default(),
    ///     Default::default(),
    ///     Default::default(),
    ///     vec![
    ///         Account::new(1, Default::default(), Default::default(), Default::default()),
    ///         Account::new(2, Default::default(), Default::default(), Default::default())
    ///     ],
    /// );
    /// account.push(Account::new(3, Default::default(), Default::default(), Default::default()), Valid::new_true());
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
    /// ```
    pub fn push(&mut self, account: Self, valid: Valid) {
        if self.valid.children && valid.children && !account.valid.is_valid() {
            self.fix_valid(Valid::new(false, false, true));
        }
        if self.valid.settings && valid.settings && !account.valid.is_valid() {
            self.fix_valid(Valid::new(false, true, false));
        }
        if account.active {
            for setting in account.settings.keys() {
                self.insert(setting.to_owned(), account.get(setting).unwrap().clone());
            }
        }
        if self.valid.names && valid.names && self.accounts_names().contains(&&account.name) {
            self.accounts.push(account);
            self.fix_valid(Valid::new(true, false, false));
        } else {
            self.accounts.push(account);
        }
    }
    /// Appends an `Account` to the back of the `Vec` of child `Accounts` of a child `Account`.
    ///
    /// This will updated the [settings](Account#settings) of all necessary Accounts
    /// so that the parent Account remains [valid](Account#valid)
    ///
    /// Part of the [deep functions](Account#deep-functions) group that accept a `Vec` of &N to identify
    /// the child `Account` to run the function. [`push`](Account::push) in this case.
    ///
    /// # Errors
    ///
    /// Deep functions can return [`DeepError`]'s
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::account::{Account,Valid};
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
    ///             Account::new("3_3".to_string(), true, Default::default(), Default::default()),
    ///         ])
    ///     ],
    /// );
    ///
    /// assert_eq!(account.deep_push(
    ///     Account::new("3_2.1".to_string(), true, Default::default(), Default::default()),
    ///     Valid::new_true(),
    ///     &mut vec![&"3_2".to_string(),&"3".to_string()])
    /// , None);
    ///
    ///
    /// assert_eq!(account ,
    ///     Account::<String,(),()>::new(
    ///         "Old Name".to_string(),
    ///         Default::default(),
    ///         Default::default(),
    ///         vec![
    ///             Account::new("1".to_string(), true, Default::default(), Default::default()),
    ///             Account::new("2".to_string(), true, Default::default(), Default::default()),
    ///             Account::new("3".to_string(), true, Default::default(), vec![
    ///                 Account::new("3_1".to_string(), true, Default::default(), Default::default()),
    ///                 Account::new("3_2".to_string(), true, Default::default(), vec![
    ///                     Account::new("3_2.1".to_string(), true, Default::default(), Default::default()),
    ///                 ]),
    ///                 Account::new("3_3".to_string(), true, Default::default(), Default::default()),
    ///             ])
    ///         ],
    ///     )
    /// );
    ///
    ///
    /// # Ok::<(), hashmap_settings::account::DeepError>(())
    /// ```
    pub fn deep_push(
        &mut self,
        account: Self,
        valid: Valid,
        account_names: &mut Vec<&N>,
    ) -> Option<DeepError> {
        self.deep_push_helper(account, valid, account_names).err()
    }
    fn deep_push_helper(
        &mut self,
        account: Self,
        valid: Valid,
        account_names: &mut Vec<&N>,
    ) -> Result<Vec<K>, DeepError> {
        let Some(account_to_find) = account_names.pop() else {
            return Err(DeepError::EmptyVec); //error if the original call is empty, but this will create the base case in the recursive call
        };
        #[allow(clippy::option_if_let_else)]
        if let Some(found_account) = self.mut_account_from_name(account_to_find) {
            if account_names.is_empty() {
                //this and the unreachable()! have been added due to https://github.com/rust-lang/rust/issues/21906
                let is_active = account.active;
                found_account.push(account, valid);
                return Ok(if is_active {
                    found_account.accounts[found_account.len() - 1]
                        .keys()
                        .map(std::borrow::ToOwned::to_owned)
                        .collect::<Vec<_>>()
                } else {
                    vec![]
                });
            }
            match found_account.deep_push_helper(account, valid, account_names) {
                //recursive call
                Ok(keys) => {
                    self.update_vec(&keys.iter().collect());
                    Ok(keys) //returning the original value from the base case
                }
                Err(error) => match error {
                    DeepError::EmptyVec => {
                        unreachable!() //Ok(found_account)
                    } //base case
                    DeepError::NotFound => Err(error), //error/bad function call
                },
            }
        } else {
            Err(DeepError::NotFound)
        }
    }
}

impl<N: Default, K, V> Default for Account<N, K, V> {
    fn default() -> Self {
        Self {
            name: N::default(),
            active: true,
            settings: HashMap::default(),
            accounts: Vec::default(),
            valid: Valid::default(),
        }
    }
}
impl<N: Clone, K: Clone, V: Clone> Clone for Account<N, K, V> {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            active: self.active,
            settings: self.settings.clone(),
            accounts: self.accounts.clone(),
            valid: self.valid,
        }
    }
}
impl<N: Debug, K: Debug, V: Debug> Debug for Account<N, K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Account")
            .field("name", &self.name)
            .field("active", &self.active)
            .field("settings", &self.settings)
            .field("accounts", &self.accounts)
            .field("valid", &self.valid)
            .finish()
    }
}
impl<N: PartialEq, K: Eq + Hash, V: PartialEq> PartialEq for Account<N, K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.active == other.active
            && self.settings == other.settings
            && self.accounts == other.accounts
            && self.valid == other.valid
    }
}
#[cfg(feature = "serde")]
impl<'de, N, K, V> Deserialize<'de> for Account<N, K, V> {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        todo!()
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(feature = "serde", typetag::serialize)]
impl<
        N: Setting + Clone + Debug + PartialEq + Serialize + for<'a> Deserialize<'a>,
        K: Setting + Clone + Debug + Eq + Hash + Serialize + for<'a> Deserialize<'a>,
        V: Setting + Clone + Debug + PartialEq + Serialize + for<'a> Deserialize<'a>,
    > Setting for Account<N, K, V>
{
    fn typetag_deserialize(&self) {
        //todo!(figure what this is supposed to do as its not mut, and return "()")
    }
}

#[cfg(not(feature = "serde"))]
impl<
        N: Setting + Clone + Debug + PartialEq,
        K: Setting + Clone + Debug + Eq + Hash,
        V: Setting + Clone + Debug + PartialEq,
    > Setting for Account<N, K, V>
{
}

/// `Account`'s validity tracker
///
/// [`Account`] contains a valid field of type `Valid` that tracks if an [`Account`] is [valid](Account#valid).
///
/// `Valid` contains 3 `bool` fields corresponding to the 3 ways an account can be invalid:
///
/// names: An `Account` is invalid if it's children `Accounts` have duplicated names.
///
/// settings: An `Account` is invalid if it doesn't contain all settings present in it's children `Accounts`.
///
/// accounts: An `Account` is invalid if it's children `Accounts` are themselves invalid.
///
///
/// `Valid` is also used in certain methods in `Account` that interact with it's `valid` field.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[must_use]
pub struct Valid {
    names: bool,
    settings: bool,
    children: bool,
}
impl Valid {
    /// Creates a new `Valid`
    ///
    /// # Examples
    /// ```
    /// use hashmap_settings::account::Valid;
    ///
    /// let _valid = Valid::new(true,true,true);
    /// ```
    pub const fn new(names: bool, settings: bool, children: bool) -> Self {
        Self {
            names,
            settings,
            children,
        }
    }
    /// Creates a new `Valid` where all fields are `true`.
    ///
    /// # Examples
    /// ```
    /// use hashmap_settings::account::Valid;
    ///
    /// let valid = Valid::new_true();
    /// assert_eq!(valid,Valid::new(true,true,true));
    /// ```
    pub const fn new_true() -> Self {
        Self {
            names: true,
            settings: true,
            children: true,
        }
    }
    /// Creates a new `Valid` where all fields are `false`.
    ///
    /// # Examples
    /// ```
    /// use hashmap_settings::account::Valid;
    ///
    /// let valid = Valid::new_false();
    /// assert_eq!(valid,Valid::new(false,false,false));
    /// ```
    pub const fn new_false() -> Self {
        Self {
            names: false,
            settings: false,
            children: false,
        }
    }
    /// Returns `true` if all fields are `true`.
    ///
    /// # Examples
    /// ```
    /// use hashmap_settings::account::Valid;
    ///
    /// let valid = Valid::new(true,true,true);
    /// assert!(valid.is_valid());
    /// let invalid = Valid::new(false,true,true);
    /// assert!(!invalid.is_valid());
    /// ```
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.children && self.settings && self.names
    }
    /// Returns the value of the field `names`
    ///
    /// # Examples
    /// ```
    /// use hashmap_settings::account::Valid;
    ///
    /// let valid1 = Valid::new(true,true,true);
    /// assert!(valid1.names());
    /// let valid2 = Valid::new(false,true,true);
    /// assert!(!valid2.names());
    /// ```
    #[must_use]
    pub const fn names(&self) -> bool {
        self.names
    }
    /// Returns the value of the field `settings`
    ///
    /// # Examples
    /// ```
    /// use hashmap_settings::account::Valid;
    ///
    /// let valid1 = Valid::new(true,true,true);
    /// assert!(valid1.settings());
    /// let valid2 = Valid::new(true,false,true);
    /// assert!(!valid2.settings());
    /// ```
    #[must_use]
    pub const fn settings(&self) -> bool {
        self.settings
    }
    /// Returns the value of the field `children`
    ///
    /// # Examples
    /// ```
    /// use hashmap_settings::account::Valid;
    ///
    /// let valid1 = Valid::new(true,true,true);
    /// assert!(valid1.children());
    /// let valid2 = Valid::new(true,true,false);
    /// assert!(!valid2.children());
    /// ```
    #[must_use]
    pub const fn children(&self) -> bool {
        self.children
    }
}
impl Default for Valid {
    fn default() -> Self {
        Self {
            names: true,
            settings: true,
            children: true,
        }
    }
}

/// Trait for types that can be incremented.
///
/// This method is necessary for types that will be used as a name for [`Account`] as it is used
/// to change an `Account's` name in case of duplication.
///
/// For types that have a limited set of names such as i32, if there are more `Accounts` than the max number
/// of names (2^32 for i32) it will cause an infinite loop in certain `Account`'s methods.
///
///
/// Implementations for `Incrementable` are under the feature "incrementable" that is
/// activated by default.
///
/// The feature should be turned off in the case a different implementation than the provided is desired.
///
///
/// # Examples
///
/// ```
/// use hashmap_settings::account::Incrementable;
///
/// let mut number = 13_i32;
/// number.increment_mut();
/// assert_eq!(number, 14);
/// ```
pub trait Incrementable {
    /// return the incremented value
    ///
    /// # Examples
    ///  ```
    /// use hashmap_settings::account::Incrementable;
    ///
    /// let number = 13_i32;
    /// assert_eq!(number.increment(), 14);
    /// assert_eq!(number, 13);
    /// ```
    #[must_use]
    fn increment(&self) -> Self;
    /// increments self
    ///
    /// # Examples
    ///  ```
    /// use hashmap_settings::account::Incrementable;
    ///
    /// let mut number = 13_i32;
    /// number.increment_mut();
    /// assert_eq!(number, 14);
    /// ```
    fn increment_mut(&mut self);
}

/// Errors involving [Deep Functions](Account#deep-functions)
#[derive(Debug, PartialEq, Eq)]
pub enum DeepError {
    /// Error of providing a name of a [child](Account#accounts) Account that doesn't exist
    NotFound,
    /// Error of providing a empty `Vec` to a deep function
    EmptyVec,
}