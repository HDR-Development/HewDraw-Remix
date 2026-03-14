use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app10bossgalaga28is_enable_capture_bossgalagaERKNS_26BattleObjectModuleAccessorE"]
        pub(super) fn is_enable_capture_bossgalaga(module_accessor: *const app::BattleObjectModuleAccessor) -> bool;
    
        #[link_name = "_ZN3app10bossgalaga8set_grabERNS_18ItemModuleAccessorEfff"]
        pub(super) fn set_grab(module_accessor: *mut app::ItemModuleAccessor, pos_x: f32, pos_y: f32, pos_z: f32);
    
    }
}

pub fn is_enable_capture_bossgalaga(module_accessor: &app::BattleObjectModuleAccessor) -> bool {
    unsafe {
        impl_::is_enable_capture_bossgalaga(module_accessor)
    }
}

pub fn set_grab(module_accessor: &mut app::ItemModuleAccessor, pos_x: f32, pos_y: f32, pos_z: f32) {
    unsafe {
        impl_::set_grab(module_accessor, pos_x, pos_y, pos_z)
    }
}

