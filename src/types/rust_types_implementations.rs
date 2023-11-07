use crate::*;
/*
currently the types being added are most of the types that derive serde Deserialize per version 1.0.192
https://docs.rs/serde/latest/serde/de/trait.Deserialize.html#
the types were obtained by doing a copy paste of the page above followed by multiple find and replace commands.
exception being Cow<'a, T> where I additionally added "+ Setting"
types not implemented:
types in rust unstable feature: !
types in the serde rc feature: Ark, Rc, ArkWeak, and RcWeak.
types that had some sort of lifetime error: str, std::path::Path, [u8], serde::de::IgnoredAny
*/


impl Setting for std::net::IpAddr{}
impl Setting for std::net::SocketAddr{}
impl Setting for bool{}
impl Setting for char{}
impl Setting for f32{}
impl Setting for f64{}
impl Setting for i8{}
impl Setting for i16{}
impl Setting for i32{}
impl Setting for i64{}
impl Setting for i128{}
impl Setting for isize{}
/*
impl Setting for !{}//Available on crate feature unstable only.
*/
impl Setting for u8{}
impl Setting for u16{}
impl Setting for u32{}
impl Setting for u64{}
impl Setting for u128{}
impl Setting for (){}
impl Setting for usize{}
impl Setting for Box<str>{}//Available on crate features std or alloc only.{}
impl Setting for Box<std::ffi::CStr>{}//Available on crate features std or alloc only.{}
impl Setting for Box<std::ffi::OsStr>{}//Available on crate feature std and (Unix or Windows) only.{}
impl Setting for Box<std::path::Path>{}
impl Setting for std::ffi::CString{}//Available on crate features std or alloc only.{}
impl Setting for String{}//Available on crate features std or alloc only.{}
impl Setting for std::net::Ipv4Addr{}
impl Setting for std::net::Ipv6Addr{}
impl Setting for std::net::SocketAddrV4{}
impl Setting for std::net::SocketAddrV6{}
impl Setting for std::num::NonZeroI8{}
impl Setting for std::num::NonZeroI16{}
impl Setting for std::num::NonZeroI32{}
impl Setting for std::num::NonZeroI64{}
impl Setting for std::num::NonZeroI128{}
impl Setting for std::num::NonZeroIsize{}
impl Setting for std::num::NonZeroU8{}
impl Setting for std::num::NonZeroU16{}
impl Setting for std::num::NonZeroU32{}
impl Setting for std::num::NonZeroU64{}
impl Setting for std::num::NonZeroU128{}
impl Setting for std::num::NonZeroUsize{}
impl Setting for std::sync::atomic::AtomicBool{}//Available on crate feature // std and target_has_atomic="8"
impl Setting for std::sync::atomic::AtomicI8{}//Available on crate feature // std and target_has_atomic="8"
impl Setting for std::sync::atomic::AtomicI16{}//Available on crate feature // std and target_has_atomic="16"
impl Setting for std::sync::atomic::AtomicI32{}//Available on crate feature // std and target_has_atomic="32"
impl Setting for std::sync::atomic::AtomicI64{}//Available on crate feature // std and target_has_atomic="64"
impl Setting for std::sync::atomic::AtomicIsize{}//Available on crate feature // std and target_has_atomic="ptr"
impl Setting for std::sync::atomic::AtomicU8{}//Available on crate feature // std and target_has_atomic="8"
impl Setting for std::sync::atomic::AtomicU16{}//Available on crate feature // std and target_has_atomic="16"
impl Setting for std::sync::atomic::AtomicU32{}//Available on crate feature // std and target_has_atomic="32"
impl Setting for std::sync::atomic::AtomicU64{}//Available on crate feature // std and target_has_atomic="64"
impl Setting for std::sync::atomic::AtomicUsize{}//Available on crate feature // std and target_has_atomic="ptr"
impl Setting for std::time::Duration{}
impl Setting for std::ffi::OsString{}//Available on crate feature std and (Unix or Windows) only.{}
impl Setting for std::path::PathBuf{}
impl Setting for std::time::SystemTime{}
impl<'a, T> Setting for std::borrow::Cow<'a, T>
where
    T: ToOwned + ?Sized + Setting,// added +Setting
    T::Owned: Setting,{}//Available on crate features std or alloc only.{}
impl<Idx> Setting for std::ops::Range<Idx>
where
    Idx: Setting,{}
impl<Idx> Setting for std::ops::RangeFrom<Idx>
where
    Idx: Setting,{}
impl<Idx> Setting for std::ops::RangeInclusive<Idx>
where
    Idx: Setting,{}
impl<Idx> Setting for std::ops::RangeTo<Idx>
where
    Idx: Setting,{}
impl<K, V> Setting for std::collections::BTreeMap<K, V>
where
    K: Setting + Ord,
    V: Setting,{}//Available on crate features std or alloc only.{}
impl<K, V, S> Setting for HashMap<K, V, S>
where
    K: Setting + Eq + std::hash::Hash,
    V: Setting,
    S: std::hash::BuildHasher + Default,{}
impl<T0: Setting> Setting for (T0,){}
impl<T0: Setting, T1: Setting> Setting for (T0, T1){}
impl<T0: Setting, T1: Setting, T2: Setting> Setting for (T0, T1, T2){}
impl<T0: Setting, T1: Setting, T2: Setting, T3: Setting> Setting for (T0, T1, T2, T3){}
impl<T0: Setting, T1: Setting, T2: Setting, T3: Setting, T4: Setting> Setting for (T0, T1, T2, T3, T4){}
impl<T0: Setting, T1: Setting, T2: Setting, T3: Setting, T4: Setting, T5: Setting> Setting for (T0, T1, T2, T3, T4, T5){}
impl<T0: Setting, T1: Setting, T2: Setting, T3: Setting, T4: Setting, T5: Setting, T6: Setting> Setting for (T0, T1, T2, T3, T4, T5, T6){}
impl<T0: Setting, T1: Setting, T2: Setting, T3: Setting, T4: Setting, T5: Setting, T6: Setting, T7: Setting> Setting for (T0, T1, T2, T3, T4, T5, T6, T7){}
impl<T0: Setting, T1: Setting, T2: Setting, T3: Setting, T4: Setting, T5: Setting, T6: Setting, T7: Setting, T8: Setting> Setting for (T0, T1, T2, T3, T4, T5, T6, T7, T8){}
impl<T0: Setting, T1: Setting, T2: Setting, T3: Setting, T4: Setting, T5: Setting, T6: Setting, T7: Setting, T8: Setting, T9: Setting> Setting for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9){}
impl<T0: Setting, T1: Setting, T2: Setting, T3: Setting, T4: Setting, T5: Setting, T6: Setting, T7: Setting, T8: Setting, T9: Setting, T10: Setting> Setting for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10){}
impl<T0: Setting, T1: Setting, T2: Setting, T3: Setting, T4: Setting, T5: Setting, T6: Setting, T7: Setting, T8: Setting, T9: Setting, T10: Setting, T11: Setting> Setting for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11){}
impl<T0: Setting, T1: Setting, T2: Setting, T3: Setting, T4: Setting, T5: Setting, T6: Setting, T7: Setting, T8: Setting, T9: Setting, T10: Setting, T11: Setting, T12: Setting> Setting for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12){}
impl<T0: Setting, T1: Setting, T2: Setting, T3: Setting, T4: Setting, T5: Setting, T6: Setting, T7: Setting, T8: Setting, T9: Setting, T10: Setting, T11: Setting, T12: Setting, T13: Setting> Setting for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13){}
impl<T0: Setting, T1: Setting, T2: Setting, T3: Setting, T4: Setting, T5: Setting, T6: Setting, T7: Setting, T8: Setting, T9: Setting, T10: Setting, T11: Setting, T12: Setting, T13: Setting, T14: Setting> Setting for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14){}
impl<T0: Setting, T1: Setting, T2: Setting, T3: Setting, T4: Setting, T5: Setting, T6: Setting, T7: Setting, T8: Setting, T9: Setting, T10: Setting, T11: Setting, T12: Setting, T13: Setting, T14: Setting, T15: Setting> Setting for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15){}
impl<T> Setting for std::ops::Bound<T>
where
    T: Setting,{}
impl<T> Setting for Option<T>
where
    T: Setting,{}
impl<T> Setting for [T; 0]{}
impl<T> Setting for [T; 1]
where
    T: Setting,{}
impl<T> Setting for [T; 2]
where
    T: Setting,{}
impl<T> Setting for [T; 3]
where
    T: Setting,{}
impl<T> Setting for [T; 4]
where
    T: Setting,{}
impl<T> Setting for [T; 5]
where
    T: Setting,{}
impl<T> Setting for [T; 6]
where
    T: Setting,{}
impl<T> Setting for [T; 7]
where
    T: Setting,{}
impl<T> Setting for [T; 8]
where
    T: Setting,{}
impl<T> Setting for [T; 9]
where
    T: Setting,{}
impl<T> Setting for [T; 10]
where
    T: Setting,{}
impl<T> Setting for [T; 11]
where
    T: Setting,{}
impl<T> Setting for [T; 12]
where
    T: Setting,{}
impl<T> Setting for [T; 13]
where
    T: Setting,{}
impl<T> Setting for [T; 14]
where
    T: Setting,{}
impl<T> Setting for [T; 15]
where
    T: Setting,{}
impl<T> Setting for [T; 16]
where
    T: Setting,{}
impl<T> Setting for [T; 17]
where
    T: Setting,{}
impl<T> Setting for [T; 18]
where
    T: Setting,{}
impl<T> Setting for [T; 19]
where
    T: Setting,{}
impl<T> Setting for [T; 20]
where
    T: Setting,{}
impl<T> Setting for [T; 21]
where
    T: Setting,{}
impl<T> Setting for [T; 22]
where
    T: Setting,{}
impl<T> Setting for [T; 23]
where
    T: Setting,{}
impl<T> Setting for [T; 24]
where
    T: Setting,{}
impl<T> Setting for [T; 25]
where
    T: Setting,{}
impl<T> Setting for [T; 26]
where
    T: Setting,{}
impl<T> Setting for [T; 27]
where
    T: Setting,{}
impl<T> Setting for [T; 28]
where
    T: Setting,{}
impl<T> Setting for [T; 29]
where
    T: Setting,{}
impl<T> Setting for [T; 30]
where
    T: Setting,{}
impl<T> Setting for [T; 31]
where
    T: Setting,{}
impl<T> Setting for [T; 32]
where
    T: Setting,{}
impl<T> Setting for std::collections::BinaryHeap<T>
where
    T: Setting + Ord,{}//Available on crate features std or alloc only.{}
impl<T> Setting for std::collections::BTreeSet<T>
where
    T: Setting + Eq + Ord,{}//Available on crate features std or alloc only.{}
impl<T> Setting for std::collections::LinkedList<T>
where
    T: Setting,{}//Available on crate features std or alloc only.{}
impl<T> Setting for std::collections::VecDeque<T>
where
    T: Setting,{}//Available on crate features std or alloc only.{}
/*
    impl<T> Setting for RcWeak<T>
where
    T: Setting + ?Sized,{}//Available on crate feature // rc and (crate features std or alloc)
impl<T> Setting for ArcWeak<T>
where
    T: Setting + ?Sized,{}//Available on crate feature // rc and (crate features std or alloc)
*/
impl<T> Setting for Vec<T>
where
    T: Setting,{}//Available on crate features std or alloc only.{}
impl<T> Setting for std::cell::Cell<T>
where
    T: Setting + Copy,{}
impl<T> Setting for std::num::Wrapping<T>
where
    T: Setting,{}
impl<T, E> Setting for Result<T, E>
where
    T: Setting,
    E: Setting,{}
impl<T, S> Setting for HashSet<T, S>
where
    T: Setting + Eq + std::hash::Hash,
    S: std::hash::BuildHasher + Default,{}
impl<T: Setting> Setting for Box<[T]>{}//Available on crate features std or alloc only.{}
impl<T: Setting> Setting for Box<T>{}//Available on crate features std or alloc only.{}
impl<T: Setting> Setting for std::cell::RefCell<T>{}
impl<T: Setting> Setting for std::cmp::Reverse<T>{}
impl<T: Setting> Setting for std::sync::Mutex<T>{}
impl<T: Setting> Setting for std::sync::RwLock<T>{}
/*
impl<T: ?Sized> Setting for std::rc::Rc<T>
where
    Box<T>: Setting,{}//Available on crate feature // rc and (crate features std or alloc)
impl<T: ?Sized> Setting for std::sync::Arc<T>
where
    Box<T>: Setting,{}//Available on crate feature // rc and (crate features std or alloc)
*/
impl<T: ?Sized> Setting for std::marker::PhantomData<T>{}
/*
impl<'de: 'a, 'a> Setting for &'a str{}
impl<'de: 'a, 'a> Setting for &'a std::path::Path{}
impl<'de: 'a, 'a> Setting for &'a [u8]{}
impl Setting for serde::de::IgnoredAny{}
*/
mod tests {
    #![allow(unused_imports)]
    use crate::*;
    #[test]
    fn bool_stg_conversion() {
        let bool = true;
        let stg_fun: Stg = stg(bool);
        let stg_dot: Stg = bool.stg();
        assert_eq!(stg_fun, stg_dot);
        let stg_fu1 = stg_fun.clone();
        let stg_do1 = stg_fun.clone();
        let bool_dot_unstg: bool = stg_fun.unstg();
        let bool_dot_safe_unstg: bool = stg_dot.safe_unstg().unwrap();
        let bool_fun_unstg: bool = unstg(stg_fu1);
        let bool_fun_safe_unstg: bool = safe_unstg(stg_do1).unwrap();
        assert_eq!(bool_dot_unstg, bool);
        assert_eq!(bool_fun_unstg, bool);
        assert_eq!(bool_dot_unstg, bool_dot_safe_unstg);
        assert_eq!(bool_fun_unstg, bool_fun_safe_unstg);
    }
}
