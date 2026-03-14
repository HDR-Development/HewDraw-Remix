use crate::*;

#[repr(C)]
#[vtable_impl(LuaModule)]
// #[derive(TypeAssert)]
// #[size = 0x168]
pub(crate) struct LuaModuleVTable {

}

#[repr(C)]
pub struct LuaModule {
    vtable: &'static LuaModuleVTable,
    owner: *mut app::BattleObjectModuleAccessor
}