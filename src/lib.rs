//! HashMap wrapper for stackable Settings 
//! 
//! By using this crate you are able to store and access all your program settings by calling a single Account struct regardless of the type that those settings implement.
//! 
//! This crate gives the tools necessary for a developer to create layered settings. This allows users of the application to not only have different settings for different environments, but also have groups of settings that they can easily switch. 
#![warn(missing_docs)]
use serde::{Deserialize, Serialize};
use std::{
    collections::{hash_map, HashMap, HashSet},
    option::Option,
};
/// module containing types used internally by the crate
pub mod types;
use types::{constants::*, errors::*};

#[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Group {
    name: String,
    settings: HashMap<String, Stg>,
}
impl Group {
    pub fn new(name: &str, settings: HashMap<String, Stg>) -> Self {
        Self {
            name: name.to_string(),
            settings,
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
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
    pub fn settings(&self) -> &HashMap<String, Stg> {
        &self.settings
    }
    pub fn contains_setting(&self, setting_name: &str) -> bool {
        self.settings.contains_key(setting_name)
    }
    pub fn get(&self, setting_name: &str) -> Option<&Stg> {
        self.settings.get(setting_name)
    }
    pub fn insert(&mut self, setting_name: &str, setting_value: Stg) -> Option<Stg> {
        self.settings
            .insert(setting_name.to_string(), setting_value)
    }
    pub fn all_settings(&self) -> hash_map::Keys<'_, String, Stg> {
        self.settings.keys()
    }
    pub fn capacity(&self) -> usize {
        self.settings.capacity()
    }
}

#[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Account {
    settings: Group,
    //should contains all settings inside of accounts and its the default for this Account
    accounts: Vec<Account>,
    //list of all sub accounts, uniquely named.
    //len()-1 is the cache if one is created.
    //last element of the vec contains the most important setting, the one that will be used by the program.
    //cache must contain all settings at all times if it exists
}
impl Account {
    pub fn new(settings: Group, accounts: Vec<Account>) -> Self {
        //doesn't check if Account is valid,consider using new_valid instead if it isn
        Account { settings, accounts }
    }
    pub fn new_valid(settings: Group, accounts: Vec<Account>) -> Result<Self, InvalidAccountError> {
        let new_account = Account { settings, accounts };
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
    pub fn name(&self) -> &str {
        self.settings.name()
    }
    pub fn settings(&self) -> &Group {
        &self.settings
    }
    pub fn accounts(&self) -> &Vec<Account> {
        &self.accounts
    }
    pub fn rename(&mut self, new_name: &str) -> Option<CacheError> {
        if self.name() == CACHE {
            return Some(CacheError::Renaming);
        }
        if new_name == CACHE {
            return Some(CacheError::Naming);
        }
        self.settings.rename(new_name);
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
    pub fn deep_get(
        &self,
        account_names: &mut Vec<&str>, //for each value, the value to its right is its parent.
        //left is the account we rename, right is the first child of the Account we call
        setting_name: &str,
    ) -> Result<Option<&Stg>, DeepChangeError> {
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
    pub fn cache(&mut self) {
        //create a cache if one doesn't exist.
        //does nothing if account name is Cache
        //updates the cache.
        if self.name() != CACHE {
            if !self.contains_cache() {
                self.accounts.push(Account::new(
                    Group::new(CACHE, Default::default()),
                    Default::default(),
                ));
            }
            let cache_position = self.cache_position().unwrap();
            self.accounts[cache_position]
                .settings
                .settings
                .reserve(self.settings.capacity()); //this assumes that Account.settings contains all settings and isn't empty
            for setting in self.settings.all_settings() {
                //this assumes that Account.settings contains all settings and isn't empty
                for account in (0..cache_position).rev() {
                    if let Some(value) = self.accounts[account].get(setting) {
                        let temp = value.clone(); //to prevent cannot borrow `self.sub_accounts` as mutable because it is also borrowed as immutable Error
                        self.accounts[cache_position].insert(setting, temp);
                    } else {
                        self.accounts[cache_position].insert(
                            setting,
                            self.settings.settings.get(setting).unwrap().clone(), //safe unwrap because we got "setting" from .all_settings()
                        );
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
        setting_value: Stg,
    ) -> Result<Option<Stg>, DeepChangeError> {
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
    pub fn push_unchecked(&mut self, account: Account) {
        //doesn't check if Account being pushed is valid
        //if a invalid account is pushed it can cause unintended behavior when other functions are called
        if let Some(cache_position) = self.cache_position() {
            self.accounts.insert(cache_position, account.clone());
            for setting in account.all_settings() {
                self.update_cache_of_setting(setting)
            }
        } else {
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
    pub fn contains_setting(&self, setting_name: &str) -> bool {
        self.settings.contains_setting(setting_name)
    }
    pub fn get(&self, setting_name: &str) -> Option<&Stg> {
        if let Some(position) = self.cache_position() {
            return self.accounts[position].get(setting_name);
        }
        for account in (0..self.len()).rev() {
            if let Some(value) = self.accounts[account].get(setting_name) {
                return Some(value);
            }
        }
        return self.settings.get(setting_name);
    }
    pub fn insert(&mut self, setting_name: &str, setting_value: Stg) -> Option<Stg> {
        let mut return_value = None;
        if let Some(value) = self.settings.insert(setting_name, setting_value.clone()) {
            return_value = Some(value);
        }
        if self.contains_cache() {
            self.update_cache_of_setting(setting_name);
        }
        return_value
    }
    pub fn all_settings(&self) -> hash_map::Keys<'_, String, Stg> {
        self.settings.all_settings()
    }
    pub fn capacity(&self) -> usize {
        self.settings.capacity()
    }
    pub fn len(&self) -> usize {
        self.accounts.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn push(&mut self, account: Account) -> Option<InvalidAccountError> {
        if account.name() == CACHE {
            //check if account isn't named Cache
            return Some(InvalidAccountError::Cache(CacheError::Naming));
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
            self.accounts.insert(cache_position, account.clone());
            for setting in account.all_settings() {
                self.update_cache_of_setting(setting)
            }
        } else {
            self.accounts.push(account);
        }
        None
    }
    pub fn pop(&mut self) -> std::option::Option<Account> {
        if let Some(position) = self.cache_position() {
            if position == 0 {
                return None;
            }
            let r_value = self.accounts.remove(position - 1);
            for setting in r_value.all_settings() {
                self.update_cache_of_setting(setting)
            }
            Some(r_value)
        } else {
            self.accounts.pop()
        }
    }
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Account> {
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

pub trait Settings
where
    Self: Serialize + for<'a> Deserialize<'a>,
{
    fn stg(self) -> Stg
    where
        Self: Settings,
    {
        Stg {
            value: serde_json::to_string(&self).unwrap(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Stg {
    value: String,
}
impl Stg {
    pub fn new<T: Settings>(value: &T) -> Stg {
        Stg {
            value: serde_json::to_string(&value).unwrap(),
        }
    }
    pub fn get(&self) -> &str {
        &self.value
    }
    pub fn unstg<T: Settings>(self) -> T {
        serde_json::from_str(&self.value).unwrap() //unsafe, can panic
    }
    pub fn safe_unstg<T: Settings>(self) -> Result<T, serde_json::Error> {
        serde_json::from_str(&self.value)
    }
}
#[allow(dead_code)]
pub fn stg<T>(value: T) -> Stg
where
    T: Settings,
{
    Stg {
        value: serde_json::to_string(&value).unwrap(),
    }
}
#[allow(dead_code)]
pub fn unstg<T>(stg: Stg) -> T
where
    T: Settings,
{
    serde_json::from_str(stg.get()).unwrap() //unsafe can panic
}
#[allow(dead_code)]
pub fn safe_unstg<T>(stg: Stg) -> Result<T, serde_json::Error>
where
    T: Settings,
{
    serde_json::from_str(stg.get())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group_test() {
        let bool_setting = true;
        let i32_setting = 42;
        let mut account = Group::default();
        account.insert("bool_setting", Stg::new(&bool_setting));
        account.insert("i32_setting", i32_setting.stg());
        let i32s: i32 = account.get("i32_setting").unwrap().clone().unstg();
        assert_eq!(i32s, 42);
        let stg: Stg = account.get("bool_setting").unwrap().clone();
        assert!(stg.unstg::<bool>());
    }
    #[test]
    fn group_new() {
        let mut group1 = Group::new("name", Default::default());
        group1.insert("answer to everything", 42.stg());
        group1.insert("true is true", true.stg());
        let group2 = Group::new(
            "name",
            [
                ("answer to everything".to_string(), 42.stg()),
                ("true is true".to_string(), true.stg()),
            ]
            .into(),
        );
        assert!(group1 == group2);
    }
}
