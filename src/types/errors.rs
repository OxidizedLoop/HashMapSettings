#[derive(PartialEq)]
pub enum CacheError {
    Renaming,
    Inserting,
    Naming,
}
#[derive(PartialEq)]
pub enum DeepChangeError {
    Cache(CacheError),
    NotFound,
    EmptyVec,
}
#[derive(PartialEq)]
pub enum InvalidAccountError {
    Cache(CacheError),
    ExistingName,
    WronglyPositionedCache,
}
