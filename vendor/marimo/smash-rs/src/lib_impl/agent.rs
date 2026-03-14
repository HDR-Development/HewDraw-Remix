use std::mem::MaybeUninit;

use skyline::libc::c_void;

use crate::*;

extern "C" {
    #[link_name = "\u{1}_ZN3lib8L2CAgent15clear_lua_stackEv"]
    fn clear_lua_stack(agent: *mut L2CAgent);

    #[link_name = "\u{1}_ZN3lib8L2CAgent6insertERKNS_8L2CValueES3_"]
    fn insert(agent: *const L2CAgent, table: *const lib::L2CValue, value: *const lib::L2CValue);

    #[allow(improper_ctypes)]
    #[link_name = "\u{1}_ZN3lib8L2CAgent6ipairsERKNS_8L2CValueE"]
    fn ipairs(
        agent: *const L2CAgent,
        table: *const lib::L2CValue,
    ) -> cpp::Vector<(lib::L2CValue, lib::L2CValue)>;

    #[link_name = "\u{1}_ZN3lib8L2CAgentC2EP9lua_State"]
    fn ctor(agent: *mut L2CAgent, state: *mut lua_State);

    #[link_name = "\u{1}_ZN3lib8L2CAgent8math_absERKNS_8L2CValueE"]
    fn math_abs(value: *const lib::L2CValue) -> lib::L2CValueHack;

    #[link_name = "\u{1}_ZN3lib8L2CAgent9math_acosERKNS_8L2CValueE"]
    fn math_acos(value: *const lib::L2CValue) -> lib::L2CValueHack;

    #[link_name = "\u{1}_ZN3lib8L2CAgent9math_atanERKNS_8L2CValueES3_"]
    fn math_atan(y: *const lib::L2CValue, x: *const lib::L2CValue) -> lib::L2CValueHack;

    #[link_name = "\u{1}_ZN3lib8L2CAgent9math_ceilERKNS_8L2CValueE"]
    fn math_ceil(val: *const lib::L2CValue) -> lib::L2CValueHack;

    #[link_name = "\u{1}_ZN3lib8L2CAgent8math_cosERKNS_8L2CValueE"]
    fn math_cos(val: *const lib::L2CValue) -> lib::L2CValueHack;

    #[link_name = "\u{1}_ZN3lib8L2CAgent8math_degERKNS_8L2CValueE"]
    fn math_deg(val: *const lib::L2CValue) -> lib::L2CValueHack;

    #[link_name = "\u{1}_ZN3lib8L2CAgent10math_floorERKNS_8L2CValueE"]
    fn math_floor(val: *const lib::L2CValue) -> lib::L2CValueHack;

    #[link_name = "\u{1}_ZN3lib8L2CAgent12math_FLT_MAXE"]
    static math_FLT_MAX: lib::L2CValue;

    #[link_name = "\u{1}_ZN3lib8L2CAgent9math_fmodERKNS_8L2CValueES3_"]
    fn math_fmod(val: *const lib::L2CValue, divisor: *const lib::L2CValue) -> lib::L2CValueHack;

    #[link_name = "\u{1}_ZN3lib8L2CAgent9math_hugeE"]
    static math_huge: lib::L2CValue;

    #[link_name = "\u{1}_ZN3lib8L2CAgent8math_logERKNS_8L2CValueES3_"]
    fn math_log(val: *const lib::L2CValue, base: *const lib::L2CValue) -> lib::L2CValueHack;

    #[link_name = "\u{1}_ZN3lib8L2CAgent8math_maxERKNS_8L2CValueES3_"]
    fn math_max(val: *const lib::L2CValue, other: *const lib::L2CValue) -> lib::L2CValueHack;

    #[link_name = "\u{1}_ZN3lib8L2CAgent8math_minERKNS_8L2CValueES3_"]
    fn math_min(val: *const lib::L2CValue, other: *const lib::L2CValue) -> lib::L2CValueHack;

    #[link_name = "\u{1}_ZN3lib8L2CAgent7math_piE"]
    static math_pi: lib::L2CValue;

    #[link_name = "\u{1}_ZN3lib8L2CAgent8math_radERKNS_8L2CValueE"]
    fn math_rad(val: *const lib::L2CValue) -> lib::L2CValueHack;

    #[link_name = "\u{1}_ZN3lib8L2CAgent8math_sinERKNS_8L2CValueE"]
    fn math_sin(val: *const lib::L2CValue) -> lib::L2CValueHack;

    #[link_name = "\u{1}_ZN3lib8L2CAgent9math_sqrtERKNS_8L2CValueE"]
    fn math_sqrt(val: *const lib::L2CValue) -> lib::L2CValueHack;

    #[link_name = "\u{1}_ZN3lib8L2CAgent8math_tanERKNS_8L2CValueE"]
    fn math_tan(val: *const lib::L2CValue) -> lib::L2CValueHack;

    #[link_name = "\u{1}_ZN3lib8L2CAgent10math_tointERKNS_8L2CValueE"]
    fn math_toint(val: *const lib::L2CValue) -> lib::L2CValueHack;

    #[allow(improper_ctypes)]
    #[link_name = "\u{1}_ZN3lib8L2CAgent5pairsERKNS_8L2CValueE"]
    fn pairs(
        agent: *const L2CAgent,
        table: *const lib::L2CValue,
    ) -> cpp::Vector<(lib::L2CValue, lib::L2CValue)>;

    #[link_name = "\u{1}_ZN3lib8L2CAgent13pop_lua_stackEi"]
    fn pop_lua_stack(agent: *mut L2CAgent, index: i32) -> lib::L2CValueHack;

    #[link_name = "\u{1}_ZN3lib8L2CAgent14push_lua_stackERKNS_8L2CValueE"]
    fn push_lua_stack(agent: *mut L2CAgent, value: *const lib::L2CValue) -> lib::L2CValueHack;

    #[link_name = "\u{1}_ZN3lib8L2CAgent6removeERKNS_8L2CValueES3_"]
    fn remove(
        agent: *const L2CAgent,
        table: *const lib::L2CValue,
        index: *const lib::L2CValue,
    ) -> lib::L2CValueHack;

    #[link_name = "\u{1}_ZN3lib8L2CAgent12setmetatableENS_8L2CValueERKS1_"]
    fn setmetatable(
        agent: *const L2CAgent,
        table: lib::L2CValueHack,
        object: *const lib::L2CValue,
    ) -> lib::L2CValueHack;

    #[link_name = "\u{1}_ZN3lib8L2CAgent4sortERKNS_8L2CValueE"]
    fn sort(agent: *const L2CAgent, table: *const lib::L2CValue);

    #[link_name = "\u{1}_ZN3lib8L2CAgent4sortERKNS_8L2CValueES3_"]
    fn sort_by(agent: *const L2CAgent, table: *const lib::L2CValue, method: *const lib::L2CValue);

    #[link_name = "\u{1}_ZN3lib8L2CAgent20sv_set_function_hashEPvN3phx6Hash40E"]
    fn sv_set_function_hash(agent: *mut L2CAgent, function: LuaBindFn, name: phx::Hash40);

    #[link_name = "\u{1}_ZN3lib8L2CAgentD2Ev"]
    fn dtor(agent: *mut L2CAgent);
}

pub type LuaBindFn = extern "C" fn(*mut lib::L2CValue, *const lib::utility::Variadic);

#[repr(C)]
#[derive(TypeAssert)]
#[size = 0x48]
pub struct L2CAgent {
    #[offset = 0x00]
    vtable: *const *const c_void,
    #[offset = 0x08]
    pub lua_state: *mut lua_State,
    #[offset = 0x10]
    pub function_map: cpp::HashMap<phx::Hash40, LuaBindFn>,
    #[offset = 0x38]
    pub battle_object: *mut c_void, // to become BattleObject
    #[offset = 0x40]
    pub module_accessor: *mut c_void, // to become BattleObjectModuleAccessor
}

impl L2CAgent {
    pub fn new(state: *mut lua_State) -> Self {
        unsafe {
            let mut value = MaybeUninit::uninit();
            ctor(value.as_mut_ptr(), state);
            value.assume_init()
        }
    }

    pub fn clear_lua_stack(&mut self) {
        unsafe { clear_lua_stack(self) }
    }

    pub fn insert(&self, table: &lib::L2CValue, value: &lib::L2CValue) {
        unsafe { insert(self, table, value) }
    }

    pub fn ipairs(&self, table: &lib::L2CValue) -> Vec<(lib::L2CValue, lib::L2CValue)> {
        unsafe { ipairs(self, table).into_vec() }
    }

    pub fn math_abs(value: &lib::L2CValue) -> lib::L2CValue {
        unsafe { math_abs(value).into() }
    }

    pub fn math_acos(value: &lib::L2CValue) -> lib::L2CValue {
        unsafe { math_acos(value).into() }
    }

    pub fn math_atan(y: &lib::L2CValue, x: &lib::L2CValue) -> lib::L2CValue {
        unsafe { math_atan(y, x).into() }
    }

    pub fn math_ceil(value: &lib::L2CValue) -> lib::L2CValue {
        unsafe { math_ceil(value).into() }
    }

    pub fn math_cos(value: &lib::L2CValue) -> lib::L2CValue {
        unsafe { math_cos(value).into() }
    }

    pub fn math_deg(value: &lib::L2CValue) -> lib::L2CValue {
        unsafe { math_deg(value).into() }
    }

    pub fn math_floor(value: &lib::L2CValue) -> lib::L2CValue {
        unsafe { math_floor(value).into() }
    }

    #[allow(non_snake_case)]
    pub fn math_FLT_MAX() -> &'static lib::L2CValue {
        unsafe { &math_FLT_MAX }
    }

    pub fn math_fmod(value: &lib::L2CValue, divisor: &lib::L2CValue) -> lib::L2CValue {
        unsafe { math_fmod(value, divisor).into() }
    }

    pub fn math_huge() -> &'static lib::L2CValue {
        unsafe { &math_huge }
    }

    pub fn math_log(value: &lib::L2CValue, base: &lib::L2CValue) -> lib::L2CValue {
        unsafe { math_log(value, base).into() }
    }

    pub fn math_max(first: &lib::L2CValue, second: &lib::L2CValue) -> lib::L2CValue {
        unsafe { math_max(first, second).into() }
    }

    pub fn math_min(first: &lib::L2CValue, second: &lib::L2CValue) -> lib::L2CValue {
        unsafe { math_min(first, second).into() }
    }

    pub fn math_pi() -> &'static lib::L2CValue {
        unsafe { &math_pi }
    }

    pub fn math_rad(value: &lib::L2CValue) -> lib::L2CValue {
        unsafe { math_rad(value).into() }
    }

    pub fn math_sin(value: &lib::L2CValue) -> lib::L2CValue {
        unsafe { math_sin(value).into() }
    }

    pub fn math_sqrt(value: &lib::L2CValue) -> lib::L2CValue {
        unsafe { math_sqrt(value).into() }
    }

    pub fn math_tan(value: &lib::L2CValue) -> lib::L2CValue {
        unsafe { math_tan(value).into() }
    }

    pub fn math_toint(value: &lib::L2CValue) -> lib::L2CValue {
        unsafe { math_toint(value).into() }
    }

    pub fn pairs(&self, table: &lib::L2CValue) -> Vec<(lib::L2CValue, lib::L2CValue)> {
        unsafe { pairs(self, table).into_vec() }
    }

    pub fn pop_lua_stack(&mut self, index: i32) -> lib::L2CValue {
        unsafe { pop_lua_stack(self, index).into() }
    }

    pub fn push_lua_stack(&mut self, value: &lib::L2CValue) {
        unsafe {
            push_lua_stack(self, value);
        }
    }

    pub fn remove(&self, table: &lib::L2CValue, index: &lib::L2CValue) -> lib::L2CValue {
        unsafe { remove(self, table, index).into() }
    }

    pub fn setmetatable(&self, table: lib::L2CValue, object: &lib::L2CValue) -> lib::L2CValue {
        unsafe { setmetatable(self, table.into(), object).into() }
    }

    pub fn sort(&self, table: &lib::L2CValue) {
        unsafe { sort(self, table) }
    }

    pub fn sort_by(&self, table: &lib::L2CValue, method: &lib::L2CValue) {
        unsafe { sort_by(self, table, method) }
    }

    pub fn sv_set_function_hash(&mut self, func: LuaBindFn, name: phx::Hash40) {
        unsafe { sv_set_function_hash(self, func, name) }
    }

    pub fn sv_get_function_by_hash(&self, name: phx::Hash40) -> Option<LuaBindFn> {
        self.function_map.get(&name).map(|x| *x)
    }
}

impl Drop for L2CAgent {
    fn drop(&mut self) {
        unsafe {
            dtor(self);
        }
    }
}
