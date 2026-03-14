use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app28FighterSpecializer_GameWatch10init_panelERNS_26BattleObjectModuleAccessorE"]
        pub(super) fn init_panel(module_accessor: *mut app::BattleObjectModuleAccessor);
    
    }
}

pub fn init_panel(module_accessor: &mut app::BattleObjectModuleAccessor) {
    unsafe {
        impl_::init_panel(module_accessor)
    }
}

