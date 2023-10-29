use crate::types::*;

impl Settings for bool{}
impl Settings for i32{}
impl Settings for char{}

mod tests {
    #![allow(unused_imports)]
    use crate::types::*;
    #[test]
    fn bool_stg_conversion() {
        let bool = true;
        let stg_fun: Stg = stg(bool);
        let stg_dot: Stg = bool.stg();
        assert_eq!(stg_fun, stg_dot);
        let stg_fu1 = stg_fun.clone();
        let stg_do1 = stg_fun.clone();
        let bool_dot_unstg      : bool   = stg_fun.unstg();
        let bool_dot_safe_unstg : bool   = stg_dot.safe_unstg().unwrap();
        let bool_fun_unstg      : bool   = unstg(stg_fu1);
        let bool_fun_safe_unstg : bool   = safe_unstg(stg_do1).unwrap();
        assert_eq!(bool_dot_unstg, bool);
        assert_eq!(bool_fun_unstg, bool);
        assert_eq!(bool_dot_unstg, bool_dot_safe_unstg);
        assert_eq!(bool_fun_unstg, bool_fun_safe_unstg);
    }
}