use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app9blackball18add_suction_energyERNS_18ItemModuleAccessorEj"]
        pub(super) fn add_suction_energy(module_accessor: *mut app::ItemModuleAccessor, receiver_obj_id: u32);
    
        #[link_name = "_ZN3app9blackball29set_attack_offset_back_line_zERNS_18ItemModuleAccessorE"]
        pub(super) fn set_attack_offset_back_line_z(module_accessor: *mut app::ItemModuleAccessor);
    
        #[link_name = "_ZN3app9blackball18start_item_captureERNS_18ItemModuleAccessorE"]
        pub(super) fn start_item_capture(module_accessor: *mut app::ItemModuleAccessor);
    
        #[link_name = "_ZN3app9blackball20start_weapon_captureERNS_18ItemModuleAccessorEj"]
        pub(super) fn start_weapon_capture(module_accessor: *mut app::ItemModuleAccessor, battle_object_id: u32);
    
    }
}

pub fn add_suction_energy(module_accessor: &mut app::ItemModuleAccessor, receiver_obj_id: u32) {
    unsafe {
        impl_::add_suction_energy(module_accessor, receiver_obj_id)
    }
}

pub fn set_attack_offset_back_line_z(module_accessor: &mut app::ItemModuleAccessor) {
    unsafe {
        impl_::set_attack_offset_back_line_z(module_accessor)
    }
}

pub fn start_item_capture(module_accessor: &mut app::ItemModuleAccessor) {
    unsafe {
        impl_::start_item_capture(module_accessor)
    }
}

pub fn start_weapon_capture(module_accessor: &mut app::ItemModuleAccessor, battle_object_id: u32) {
    unsafe {
        impl_::start_weapon_capture(module_accessor, battle_object_id)
    }
}

