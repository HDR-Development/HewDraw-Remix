use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app24FighterSpecializer_Dolly17call_final_moduleERNS_7FighterEi"]
        pub(super) fn call_final_module(arg1: *mut app::Fighter, call_arg: app::FighterDollyFinalModuleCall);
    
        #[link_name = "_ZN3app24FighterSpecializer_Dolly19check_special_air_bERNS_26BattleObjectModuleAccessorEf"]
        pub(super) fn check_special_air_b(module_accessor: *mut app::BattleObjectModuleAccessor, lr: f32) -> bool;
    
        #[link_name = "_ZN3app24FighterSpecializer_Dolly24final_module_hit_successEv"]
        pub(super) fn final_module_hit_success() -> bool;
    
        #[link_name = "_ZN3app24FighterSpecializer_Dolly23update_opponent_lr_1on1ERNS_26BattleObjectModuleAccessorEi"]
        pub(super) fn update_opponent_lr_1on1(module_accessor: *mut app::BattleObjectModuleAccessor, status_kind: i32);
    
    }
}

pub fn call_final_module(arg1: &mut app::Fighter, call_arg: app::FighterDollyFinalModuleCall) {
    unsafe {
        impl_::call_final_module(arg1, call_arg)
    }
}

pub fn check_special_air_b(module_accessor: &mut app::BattleObjectModuleAccessor, lr: f32) -> bool {
    unsafe {
        impl_::check_special_air_b(module_accessor, lr)
    }
}

pub fn final_module_hit_success() -> bool {
    unsafe {
        impl_::final_module_hit_success()
    }
}

pub fn update_opponent_lr_1on1(module_accessor: &mut app::BattleObjectModuleAccessor, status_kind: i32) {
    unsafe {
        impl_::update_opponent_lr_1on1(module_accessor, status_kind)
    }
}

