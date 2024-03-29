//! type abstraction [`Stg`] and other related elements.
//!
//! [`Stg`] type abstraction  
//!
//! [`Setting`] is the trait that needs to be implemented for types for them to be turned into `Stg`
//!
//! [`StgError`] Error on conversion from `Stg` or T<&Stg> into S: Setting
//!
//! [`StgTrait`] Trait implement
//!
//!
//! # Example use of `Stg` in an [`Account`](crate::account::Account):
//!
//! ```rust
//! use hashmap_settings::{account::Account,stg::{Setting,Stg,StgTrait,StgError}};
//! //creating an account
//! let mut account = Account::<(),&str,Stg>::default();
//!
//! //inserting values of distinct types
//! account.insert("Number of trees",5.stg());
//! account.insert("Grass color","green".to_string().stg());
//! account.insert("Today is good",true.stg());
//!
//! //getting values from the account
//! let today_bool: bool = account.get(&"Today is good").unstg()?;
//! let grass_color: String = account.get(&"Grass color").unstg()?;
//! let trees: i32 = account.get(&"Number of trees").unstg()?;
//!
//! //example of using the values
//! print!("It's {today_bool} that today is a wonderful day,
//!     the grass is {grass_color} and I can see {trees} trees in the distance");
//!
//! Ok::<(),StgError>(())
//! ```

///module containing implementations of `Setting` for rust types
pub mod setting_implementations;

use core::fmt::Debug;
use std::any::Any;

use dyn_clone::DynClone;
use dyn_ord::DynEq;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Required trait for conversion to abstract type [Stg]
///
/// For a Type to be able to implement Setting it needs to implement the traits
/// [Clone], [Debug], [PartialEq] (as well as [Deserialize](https://docs.rs/serde/latest/serde/trait.Deserialize.html) and [Serialize](https://docs.rs/serde/latest/serde/trait.Serialize.html) if the "serde" feature is activated )
///
/// In the [future](https://github.com/OxidizedLoop/HashMapSettings/issues/1) you will be able to derive Setting,
/// but for now you can do it by adding the following lines:
/// ```
/// # use hashmap_settings::stg::Setting;
/// # #[cfg(feature = "serde")]
/// # use serde::{Deserialize, Serialize};
/// #
/// # #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/// # #[derive(Clone, Debug, PartialEq)]
/// # pub struct MyType{}
/// #
/// # #[cfg_attr(feature = "serde", typetag::serde)]
/// // add #[typetag::serde] if serde feature is activated
/// impl Setting for MyType{}
/// ```
#[cfg_attr(feature = "serde", typetag::serde(tag = "setting"))]
pub trait Setting: Any + Debug + DynClone + DynEq {
    /// turns a type implementing [Setting] into a [Stg]
    ///
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::stg::{Setting,Stg};
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
    #[allow(clippy::unconditional_recursion)] //todo!(git issue https://github.com/rust-lang/rust-clippy/pull/12177 should resolve this)
    fn eq(&self, other: &Self) -> bool {
        let x: Box<dyn DynEq> = self.clone();
        let y: Box<dyn DynEq> = other.clone();
        x == y
    }
}

/// type abstraction for types implementing [`Setting`]
///
/// Types implementing `Setting` can be turned into a `Stg` with [.stg()](Setting::stg).
///
/// ```
/// use hashmap_settings::stg::{Setting,Stg};
/// # #[allow(unused_variables)]
/// let bool_stg: Stg = true.stg();
/// ```
///
/// They can be turned back to a specific type with [.unstg()](Stg::unstg) or [.unstg_panic()](Stg::unstg_panic)
///
///  ```
/// # use hashmap_settings::stg::{Setting,Stg};
/// # let bool_stg: Stg = true.stg();
/// # #[allow(unused_variables)]
/// let bool: bool = bool_stg.unstg()?;
/// # Ok::<(),Box<dyn core::any::Any>>(())
/// ```
///
/// Additionally there is the [`StgTrait`] that can be implemented for types containing `Stg` to allow
/// `.unstg()` and `.unstg_panic()` to be called on them.
///
/// The main example would be [Option<&Stg>]
///
///  ```
/// use std::collections::HashMap;
/// use hashmap_settings::stg::{Setting,Stg,StgError,StgTrait};
/// let bool_stg: Stg = true.stg();
/// let mut hashmap = HashMap::new();
/// hashmap.insert("bool",bool_stg);
/// # #[allow(unused_variables)]
/// let bool: bool = hashmap.get("bool").unstg()?;
/// # Ok::<(),StgError>(())
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
    /// Consider using [`unstg_panic`](Stg::unstg_panic) if it's guaranteed that we will convert to the right type.
    ///
    /// # Example
    ///
    /// ```
    /// use hashmap_settings::stg::{Setting,Stg};
    ///
    /// let bool_stg: Stg = true.stg();
    /// assert_eq!(bool_stg.unstg::<bool>()?, true);
    /// //we need to use ::<bool> to specify that want to turn bool_stg into a bool
    /// # Ok::<(),Box<dyn core::any::Any>>(())
    /// ```
    ///
    /// ```
    /// use hashmap_settings::stg::{Setting,Stg};
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
    /// This function returns a `Err(Box<dyn Any>)` if we try to covert to the wrong type.
    ///
    /// ```
    /// use hashmap_settings::stg::{Setting,Stg};
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
    /// Consider using [`unstg`](Stg::unstg) as it returns a result type instead.
    ///
    /// ```should_panic
    /// use hashmap_settings::stg::{Setting,Stg};
    ///
    /// let bool_stg: Stg = true.stg();
    /// let _number :i32 = bool_stg.unstg_panic();
    /// // this panics, as the Box<dyn Setting> holds a bool value but we are trying to convert it to a i32
    /// ```
    /// # Examples
    ///
    /// ```
    /// use hashmap_settings::stg::{Setting,Stg};
    ///
    /// let bool_stg: Stg = true.stg();
    /// assert_eq!(bool_stg.unstg_panic::<bool>(), true);
    /// //we need to use ::<bool> to specify that want to turn bool_stg into a bool
    /// ```
    /// ```
    /// use hashmap_settings::stg::{Setting,Stg};
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
                Err(_error) => Err(StgError::WrongType),
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
/// # use hashmap_settings::{account::Account,stg::{Setting,Stg,StgTrait,StgError}};
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
#[allow(clippy::module_name_repetitions)]
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
    /// # use hashmap_settings::{account::Account,stg::{Setting,Stg,StgTrait,StgError}};
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
    /// # use hashmap_settings::{account::Account,stg::{Setting,Stg,StgTrait}};
    /// let mut account: Account<(),&str,Stg> = Default::default();
    /// account.insert("a small number", 42_i32.stg());
    /// assert_eq!(account.get(&"a small number").unstg_panic::<i32>(), 42);
    /// ```
    /// ```should_panic
    /// # use hashmap_settings::{account::Account,stg::{Setting,Stg,StgTrait}};
    /// let mut account: Account<(),&str,Stg> = Default::default();
    /// account.insert("a small number", 42_i32.stg());
    /// assert_eq!(account.get(&"a small number").unstg_panic::<bool>(), true);//this panics
    /// ```
    #[must_use]
    fn unstg_panic<S: Setting>(self) -> S;
}

/// Errors for [Stg] and [StgTrait] methods
#[derive(Debug, PartialEq, Eq)]
#[allow(clippy::module_name_repetitions)]
pub enum StgError {
    /// No value found, equivalent to None in Option()
    None,
    /// Error of trying to convert to the wrong type,
    WrongType, //todo!() change WrongType to contain the error Err(StgError::WrongType(Box<dyn core::any::Any>)),
}
