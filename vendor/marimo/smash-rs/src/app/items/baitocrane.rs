use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app10baitocrane16add_camera_rangeERNS_26BattleObjectModuleAccessorEff"]
        pub(super) fn add_camera_range(module_accessor: *mut app::BattleObjectModuleAccessor, top: f32, bottom: f32);
    
        #[link_name = "_ZN3app10baitocrane28is_enable_capture_baitocraneERKNS_26BattleObjectModuleAccessorE"]
        pub(super) fn is_enable_capture_baitocrane(module_accessor: *const app::BattleObjectModuleAccessor) -> bool;
    
        #[link_name = "_ZN3app10baitocrane8set_grabERNS_26BattleObjectModuleAccessorE"]
        pub(super) fn set_grab(module_accessor: *mut app::BattleObjectModuleAccessor);
    
    }
}

pub fn add_camera_range(module_accessor: &mut app::BattleObjectModuleAccessor, top: f32, bottom: f32) {
    unsafe {
        impl_::add_camera_range(module_accessor, top, bottom)
    }
}

pub fn is_enable_capture_baitocrane(module_accessor: &app::BattleObjectModuleAccessor) -> bool {
    unsafe {
        impl_::is_enable_capture_baitocrane(module_accessor)
    }
}

pub fn set_grab(module_accessor: &mut app::BattleObjectModuleAccessor) {
    unsafe {
        impl_::set_grab(module_accessor)
    }
}

