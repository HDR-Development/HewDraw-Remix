use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app10backshield20is_enable_backshieldEPKNS_26BattleObjectModuleAccessorE"]
        pub(super) fn is_enable_backshield(module_accessor: *const app::BattleObjectModuleAccessor) -> bool;
    
        #[link_name = "_ZN3app10backshield27is_force_visible_backshieldEPKNS_26BattleObjectModuleAccessorE"]
        pub(super) fn is_force_visible_backshield(module_accessor: *const app::BattleObjectModuleAccessor) -> bool;
    
        #[link_name = "_ZN3app10backshield21is_visible_backshieldEPKNS_26BattleObjectModuleAccessorE"]
        pub(super) fn is_visible_backshield(module_accessor: *const app::BattleObjectModuleAccessor) -> bool;
    
        #[link_name = "_ZN3app10backshield13set_reflectorERNS_26BattleObjectModuleAccessorE"]
        pub(super) fn set_reflector(module_accessor: *mut app::BattleObjectModuleAccessor);
    
    }
}

pub fn is_enable_backshield(module_accessor: &app::BattleObjectModuleAccessor) -> bool {
    unsafe {
        impl_::is_enable_backshield(module_accessor)
    }
}

pub fn is_force_visible_backshield(module_accessor: &app::BattleObjectModuleAccessor) -> bool {
    unsafe {
        impl_::is_force_visible_backshield(module_accessor)
    }
}

pub fn is_visible_backshield(module_accessor: &app::BattleObjectModuleAccessor) -> bool {
    unsafe {
        impl_::is_visible_backshield(module_accessor)
    }
}

pub fn set_reflector(module_accessor: &mut app::BattleObjectModuleAccessor) {
    unsafe {
        impl_::set_reflector(module_accessor)
    }
}

