pub enum CacheError {
    Renaming,
    Inserting,
    Naming,
}
pub enum DeepChangeError {
    Cache(CacheError),
    NotFound,
    EmptyVec,
}
pub enum InvalidAccountError {
    Cache(CacheError),
    ExistingName,
    WronglyPositionedCache,
}
