use smash::app::BattleObject;
use smash::phx::Hash40;

/// Specified which kind of param to retrieve from ParamModule
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ParamType {
    /// References common params that exist across the entire game instance
    Common,
    /// References "shared" params that exist for all fighters but are different depending on fighter kind
    Shared,
    /// References agent specific params
    Agent,
}

extern "Rust" {
    #[link_name = "ParamModule__get_int"]
    fn ParamModule__get_int(object: *mut BattleObject, ty: ParamType, key: &str) -> i32;

    #[link_name = "ParamModule__get_hash"]
    fn ParamModule__get_hash(object: *mut BattleObject, ty: ParamType, key: &str) -> Hash40;

    #[link_name = "ParamModule__get_float"]
    fn ParamModule__get_float(object: *mut BattleObject, ty: ParamType, key: &str) -> f32;

    #[link_name = "ParamModule__is_flag"]
    fn ParamModule__is_flag(object: *mut BattleObject, ty: ParamType, key: &str) -> bool;

    #[link_name = "ParamModule__get_string"]
    fn ParamModule__get_string(object: *mut BattleObject, ty: ParamType, key: &str) -> String;
}

/// An additional module to be used with Smash's `BattleObject` class. This uses an ARCropolis API to load (new) files and store them as global param data
/// that objects can access at runtime. This allows for storing constants in data modifiable for easy balancing.
#[allow(non_snake_case)]
pub mod ParamModule {
    use super::*;

    pub fn get_int(object: *mut BattleObject, ty: ParamType, key: &str) -> i32 {
        unsafe {
            ParamModule__get_int(object, ty, key)
        }
    }
    pub fn get_hash(object: *mut BattleObject, ty: ParamType, key: &str) -> Hash40 {
        unsafe {
            ParamModule__get_hash(object, ty, key)
        }
    }
    pub fn get_float(object: *mut BattleObject, ty: ParamType, key: &str) -> f32 {
        unsafe {
            ParamModule__get_float(object, ty, key)
        }
    }
    pub fn is_flag(object: *mut BattleObject, ty: ParamType, key: &str) -> bool {
        unsafe {
            ParamModule__is_flag(object, ty, key)
        }
    }
    pub fn get_string(object: *mut BattleObject, ty: ParamType, key: &str) -> String {
        unsafe {
            ParamModule__get_string(object, ty, key)
        }
    }
}