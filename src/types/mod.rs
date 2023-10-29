use serde::{Deserialize, Serialize};
use std::fmt::Debug;
pub mod example_types;
pub mod rust_types;

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
