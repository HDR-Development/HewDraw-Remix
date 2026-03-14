use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app6beetle24is_enable_capture_beetleERKNS_26BattleObjectModuleAccessorE"]
        pub(super) fn is_enable_capture_beetle(module_accessor: *const app::BattleObjectModuleAccessor) -> bool;
    
        #[link_name = "_ZN3app6beetle8set_grabERNS_26BattleObjectModuleAccessorE"]
        pub(super) fn set_grab(module_accessor: *mut app::BattleObjectModuleAccessor);
    
    }
}

pub fn is_enable_capture_beetle(module_accessor: &app::BattleObjectModuleAccessor) -> bool {
    unsafe {
        impl_::is_enable_capture_beetle(module_accessor)
    }
}

pub fn set_grab(module_accessor: &mut app::BattleObjectModuleAccessor) {
    unsafe {
        impl_::set_grab(module_accessor)
    }
}

