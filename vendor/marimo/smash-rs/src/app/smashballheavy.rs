use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app14smashballheavy23force_clear_post_effectERNS_18ItemModuleAccessorE"]
        pub(super) fn force_clear_post_effect(module_accessor: *mut app::ItemModuleAccessor);

        #[link_name = "_ZN3app14smashballheavy14get_auto_handiEj"]
        pub(super) fn get_auto_handi(arg1: u32) -> f32;

        #[link_name = "_ZN3app14smashballheavy23get_fighter_entry_countEv"]
        pub(super) fn get_fighter_entry_count() -> u32;

        #[link_name = "_ZN3app14smashballheavy15set_post_effectERNS_18ItemModuleAccessorEjb"]
        pub(super) fn set_post_effect(module_accessor: *mut app::ItemModuleAccessor, arg2: u32, arg3: bool);
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

pub fn get_fighter_entry_count() -> u32 {
    unsafe {
        impl_::get_fighter_entry_count()
    }
}

pub fn set_post_effect(module_accessor: &mut app::ItemModuleAccessor, arg2: u32, arg3: bool) {
    unsafe {
        impl_::set_post_effect(module_accessor, arg2, arg3)
    }
}
