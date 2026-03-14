use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app5akira14set_all_shieldERNS_26BattleObjectModuleAccessorE"]
        pub(super) fn set_all_shield(module_accessor: *mut app::BattleObjectModuleAccessor);
    
        #[link_name = "_ZN3app5akira17set_bullet_shieldERNS_26BattleObjectModuleAccessorE"]
        pub(super) fn set_bullet_shield(module_accessor: *mut app::BattleObjectModuleAccessor);
    
    }
}

pub fn set_all_shield(module_accessor: &mut app::BattleObjectModuleAccessor) {
    unsafe {
        impl_::set_all_shield(module_accessor)
    }
}

pub fn set_bullet_shield(module_accessor: &mut app::BattleObjectModuleAccessor) {
    unsafe {
        impl_::set_bullet_shield(module_accessor)
    }
}

