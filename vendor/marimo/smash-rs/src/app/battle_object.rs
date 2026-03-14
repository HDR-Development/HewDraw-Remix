use crate::*;

use std::ops::{Deref, DerefMut};

#[repr(C)]
#[vtable_impl(BattleObject)]
// #[derive(TypeAssert)]
pub(crate) struct BattleObjectVTable {
    destructor: extern "C" fn(this: &mut BattleObject),
}

#[repr(C)]
pub struct BattleObject {
    vtable: &'static BattleObjectVTable,
    pub battle_object_id: u32,
    pub kind: i32,
    pub entry_id: i32,
    pub agent_kind: phx::Hash40,
    pub module_accessor: *mut app::BattleObjectModuleAccessor,
    pub previous: *mut BattleObject,
    pub next: *mut BattleObject,
    pub unknown_byte1: u8,
    pub unknown_byte2: u8,
    pub status: u8,
    pub other_status: u8,
    pub mutex: cpp::Mutex,
    pub unknown_byte3: u8,
    pub unknown_byte4: u8,
    padding: [u8; 6],
}

#[repr(C)]
pub struct Fighter {
    parent: BattleObject,
    // ...
}

impl Deref for Fighter {
    type Target = BattleObject;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl DerefMut for Fighter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parent
    }
}

#[repr(C)]
pub struct Weapon {
    parent: BattleObject,
    // ...
}

#[repr(C)]
pub struct Enemy {
    parent: BattleObject,
    // ...
}

#[repr(C)]
pub struct Gimmick {
    parent: BattleObject,
    // ...
}

#[repr(C)]
pub struct Item {
    parent: BattleObject,
    // ...
}
