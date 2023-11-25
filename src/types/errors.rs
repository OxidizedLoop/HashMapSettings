/// Errors involving interaction with `Cache`
#[derive(Debug, PartialEq)]
pub enum CacheError {
    /// Error of trying to rename `Cache`
    Renaming,
    /// Error of trying to create a `Cache` without using [`cache()`](Account::cache)
    Creating,
    /// Error of trying to modify the contents of `Cache`
    Modify,
    /// Error of trying to name a Account `Cache`
    Naming,
}
/// Errors involving Deep Functions
#[derive(Debug, PartialEq)]
pub enum DeepChangeError {
    /// Errors involving interaction with `Cache`
    Cache(CacheError),
    /// Error of providing a name of a sub Account that doesn't exist
    NotFound,
    /// Error of providing a empty `Vec` to a deep function
    EmptyVec,
}
/// Errors involving Deep Functions
#[derive(Debug, PartialEq)]
pub enum InvalidAccountError {
    /// Errors involving interaction with `Cache`
    Cache(CacheError),
    /// Error of trying to do a action that will lead to two `Accounts` having the same name.
    ExistingName,
    /// Error of trying to create an account where `Cache` isn't the last element of the `Vec`
    WronglyPositionedCache,
}
