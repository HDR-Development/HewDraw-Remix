use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3lib9SingletonIN3app11ItemManagerEE9instance_E"]
        pub(super) static INSTANCE: *mut app::ItemManager;

        #[link_name = "_ZN3app8lua_bind47ItemManager__find_active_item_from_area_id_implEPNS_11ItemManagerEj"]
        pub(super) fn find_active_item_from_area_id(manager: *const app::ItemManager, arg2: u32) -> *const skyline::libc::c_void;

        #[link_name = "_ZN3app8lua_bind42ItemManager__find_active_item_from_id_implEPNS_11ItemManagerEj"]
        pub(super) fn find_active_item_from_id(manager: *const app::ItemManager, battle_object_id: u32) -> *const skyline::libc::c_void;

        #[link_name = "_ZN3app8lua_bind33ItemManager__get_active_item_implEPNS_11ItemManagerEm"]
        pub(super) fn get_active_item(manager: *const app::ItemManager, arg2: u64) -> *const skyline::libc::c_void;

        #[link_name = "_ZN3app8lua_bind44ItemManager__get_num_of_active_item_all_implEPNS_11ItemManagerE"]
        pub(super) fn get_num_of_active_item_all(manager: *const app::ItemManager) -> usize;

        #[link_name = "_ZN3app8lua_bind41ItemManager__get_num_of_ownered_item_implEPNS_11ItemManagerEjNS_8ItemKindE"]
        pub(super) fn get_num_of_ownered_item(manager: *const app::ItemManager, battle_object_id: u32, item_kind: app::ItemKind) -> usize;

        #[link_name = "_ZN3app8lua_bind52ItemManager__is_change_fighter_restart_position_implEPNS_11ItemManagerE"]
        pub(super) fn is_change_fighter_restart_position(manager: *const app::ItemManager) -> bool;

        #[link_name = "_ZN3app8lua_bind37ItemManager__remove_item_from_id_implEPNS_11ItemManagerEj"]
        pub(super) fn remove_item_from_id(manager: *const app::ItemManager, battle_object_id: u32);
    }
}

#[repr(C)]
pub struct ItemManager {}

impl ItemManager {
    pub fn instance() -> Option<&'static Self> {
        unsafe {
            impl_::INSTANCE.as_ref()
        }
    }

    pub fn instance_mut() -> Option<&'static mut Self> {
        unsafe {
            impl_::INSTANCE.as_mut()
        }
    }

    pub fn find_active_item_from_area_id(&self, arg2: u32) -> *const skyline::libc::c_void {
        unsafe {
            impl_::find_active_item_from_area_id(self, arg2)
        }
    }

    pub fn find_active_item_from_id(&self, battle_object_id: u32) -> *const skyline::libc::c_void {
        unsafe {
            impl_::find_active_item_from_id(self, battle_object_id)
        }
    }

    pub fn get_active_item(&self, arg2: u64) -> *const skyline::libc::c_void {
        unsafe {
            impl_::get_active_item(self, arg2)
        }
    }

    pub fn get_num_of_active_item_all(&self) -> usize {
        unsafe {
            impl_::get_num_of_active_item_all(self)
        }
    }

    pub fn get_num_of_ownered_item(&self, battle_object_id: u32, item_kind: app::ItemKind) -> usize {
        unsafe {
            impl_::get_num_of_ownered_item(self, battle_object_id, item_kind)
        }
    }

    pub fn is_change_fighter_restart_position(&self) -> bool {
        unsafe {
            impl_::is_change_fighter_restart_position(self)
        }
    }

    pub fn remove_item_from_id(&self, battle_object_id: u32) {
        unsafe {
            impl_::remove_item_from_id(self, battle_object_id)
        }
    }
}
