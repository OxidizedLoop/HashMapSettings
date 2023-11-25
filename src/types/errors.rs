/// Errors involving Deep Functions
#[derive(Debug, PartialEq)]
pub enum DeepChangeError {
    /// Error of providing a name of a sub Account that doesn't exist
    NotFound,
    /// Error of providing a empty `Vec` to a deep function
    EmptyVec,
}
/// Errors involving Deep Functions
#[derive(Debug, PartialEq)]
pub enum InvalidAccountError {
    /// Error of trying to do a action that will lead to two `Accounts` having the same name.
    ExistingName,
    /// Error of trying to create an account where `Cache` isn't the last element of the `Vec`
    WronglyPositionedCache,
}
