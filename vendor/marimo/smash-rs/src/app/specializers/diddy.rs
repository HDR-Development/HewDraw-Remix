use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app24FighterSpecializer_Diddy23final_decide_target_dirERNS_26BattleObjectModuleAccessorEb"]
        pub(super) fn final_decide_target_dir(module_accessor: *mut app::BattleObjectModuleAccessor, arg: bool) -> bool;
    
        #[link_name = "_ZN3app24FighterSpecializer_Diddy23final_finish_attack_dirERNS_21FighterModuleAccessorE"]
        pub(super) fn final_finish_attack_dir(module_accessor: *mut app::FighterModuleAccessor) -> bool;
    
    }
}

pub fn final_decide_target_dir(module_accessor: &mut app::BattleObjectModuleAccessor, arg: bool) -> bool {
    unsafe {
        impl_::final_decide_target_dir(module_accessor, arg)
    }
}

pub fn final_finish_attack_dir(module_accessor: &mut app::FighterModuleAccessor) -> bool {
    unsafe {
        impl_::final_finish_attack_dir(module_accessor)
    }
}

