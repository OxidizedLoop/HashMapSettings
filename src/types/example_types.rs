use crate::*;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct BoolStg {
    pub value: bool,
}
#[allow(dead_code)]
impl BoolStg {
    pub fn new(value: bool) -> BoolStg {
        BoolStg { value }
    }
    pub fn get(&self) -> bool {
        self.value
    }
}
impl Settings for BoolStg {}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct I32Stg {
    pub value: i32,
}
#[allow(dead_code)]
impl I32Stg {
    pub fn new(value: i32) -> I32Stg {
        I32Stg { value }
    }
    pub fn get(&self) -> i32 {
        self.value
    }
}
impl Settings for I32Stg {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bool_stg_setting() {
        let bool_stg_setting = BoolStg::new(true);
        let stg_fun: Stg = stg(bool_stg_setting.clone());
        let stg_dot: Stg = bool_stg_setting.clone().stg();
        assert_eq!(stg_fun, stg_dot);
        let bool_stg_from: BoolStg = stg_fun.unstg();
        let bool_stg_into: BoolStg = stg_dot.safe_unstg().unwrap();
        assert_eq!(bool_stg_from, bool_stg_into);
        assert_eq!(bool_stg_setting, bool_stg_from);
        assert_eq!(bool_stg_setting, bool_stg_into);
        assert_eq!(bool_stg_setting.get(), bool_stg_into.get());
        assert!(bool_stg_into.get());
        assert!(bool_stg_from.get());
    }
}
