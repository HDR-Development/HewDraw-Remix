use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app9smashball13chase_fighterERNS_18ItemModuleAccessorE"]
        pub(super) fn chase_fighter(module_accessor: *mut app::ItemModuleAccessor);

        #[link_name = "_ZN3app9smashball19escape_from_fighterERNS_18ItemModuleAccessorE"]
        pub(super) fn escape_from_fighter(module_accessor: *mut app::ItemModuleAccessor);

        #[link_name = "_ZN3app9smashball23force_clear_post_effectERNS_18ItemModuleAccessorE"]
        pub(super) fn force_clear_post_effect(module_accessor: *mut app::ItemModuleAccessor);

        #[link_name = "_ZN3app9smashball14get_auto_handiEj"]
        pub(super) fn get_auto_handi(arg1: u32) -> f32;

        #[link_name = "_ZN3app9smashball26get_equip_draw_ability_numENS_14FighterEntryIDE"]
        pub(super) fn get_equip_draw_ability_num(entry_id: app::FighterEntryID) -> u32;

        #[link_name = "_ZN3app9smashball16is_training_modeEv"]
        pub(super) fn is_training_mode() -> bool;

        #[link_name = "_ZN3app9smashball15set_post_effectERNS_18ItemModuleAccessorEjb"]
        pub(super) fn set_post_effect(module_accessor: *mut app::ItemModuleAccessor, arg2: u32, arg3: bool);
    }
}

pub fn chase_fighter(module_accessor: &mut app::ItemModuleAccessor) {
    unsafe {
        impl_::chase_fighter(module_accessor)
    }
}

pub fn escape_from_fighter(module_accessor: &mut app::ItemModuleAccessor) {
    unsafe {
        impl_::escape_from_fighter(module_accessor)
    }
}

pub fn force_clear_post_effect(module_accessor: &mut app::ItemModuleAccessor) {
    unsafe {
        impl_::force_clear_post_effect(module_accessor)
    }
}

pub fn get_auto_handi(arg1: u32) -> f32 {
    unsafe {
        impl_::get_auto_handi(arg1)
    }
}

pub fn get_equip_draw_ability_num(entry_id: app::FighterEntryID) -> u32 {
    unsafe {
        impl_::get_equip_draw_ability_num(entry_id)
    }
}

pub fn is_training_mode() -> bool {
    unsafe {
        impl_::is_training_mode()
    }
}

pub fn set_post_effect(module_accessor: &mut app::ItemModuleAccessor, arg2: u32, arg3: bool) {
    unsafe {
        impl_::set_post_effect(module_accessor, arg2, arg3)
    }
}
