use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app6driver14get_center_posEj"]
        pub(super) fn get_center_pos(battle_object_id: u32) -> cpp::simd::Vector3;
    
        #[link_name = "_ZN3app6driver21get_rhombus_front_posERNS_18ItemModuleAccessorE"]
        pub(super) fn get_rhombus_front_pos(module_accessor: *mut app::ItemModuleAccessor) -> cpp::simd::Vector2;
    
        #[link_name = "_ZN3app6driver10link_catchERNS_18ItemModuleAccessorEj"]
        pub(super) fn link_catch(module_accessor: *mut app::ItemModuleAccessor, battle_object_id: u32) -> bool;
    
        #[link_name = "_ZN3app6driver10set_attackERNS_18ItemModuleAccessorEii"]
        pub(super) fn set_attack(module_accessor: *mut app::ItemModuleAccessor, id: i32, arg: i32);
    
    }
}

pub fn get_center_pos(battle_object_id: u32) -> phx::Vector3f {
    unsafe {
        impl_::get_center_pos(battle_object_id).into()
    }
}

pub fn get_rhombus_front_pos(module_accessor: &mut app::ItemModuleAccessor) -> phx::Vector2f {
    unsafe {
        impl_::get_rhombus_front_pos(module_accessor).into()
    }
}

pub fn link_catch(module_accessor: &mut app::ItemModuleAccessor, battle_object_id: u32) -> bool {
    unsafe {
        impl_::link_catch(module_accessor, battle_object_id)
    }
}

pub fn set_attack(module_accessor: &mut app::ItemModuleAccessor, id: i32, arg: i32) {
    unsafe {
        impl_::set_attack(module_accessor, id, arg)
    }
}

