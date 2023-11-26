/// Errors involving [Deep Functions](Account#deep-functions)
#[derive(Debug, PartialEq, Eq)]
pub enum DeepError {
    /// Error of providing a name of a [child](Account#accounts) Account that doesn't exist
    NotFound,
    /// Error of providing a empty `Vec` to a deep function
    EmptyVec,
}
/// Errors involving Account [validity](Account#valid)
#[derive(Debug, PartialEq, Eq)]
pub enum InvalidAccountError {
    /// Error of trying to do a action that will lead to two [sibling](Account#accounts) `Accounts` having the same name.
    ExistingName,
}
