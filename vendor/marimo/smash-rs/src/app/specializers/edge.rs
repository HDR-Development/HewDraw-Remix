use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app23FighterSpecializer_Edge20attack_lw4_ray_checkERNS_26BattleObjectModuleAccessorE"]
        pub(super) fn attack_lw4_ray_check(module_accessor: *mut app::BattleObjectModuleAccessor);
    
        #[link_name = "_ZN3app23FighterSpecializer_Edge17call_final_moduleERNS_7FighterEi"]
        pub(super) fn call_final_module(fighter: *mut app::Fighter, call_arg: app::FighterEdgeFinalModuleCall);
    
        #[link_name = "_ZN3app23FighterSpecializer_Edge28clear_special_hi_jostle_areaERNS_26BattleObjectModuleAccessorE"]
        pub(super) fn clear_special_hi_jostle_area(module_accessor: *mut app::BattleObjectModuleAccessor);
    
        #[link_name = "_ZN3app23FighterSpecializer_Edge24final_module_hit_successEv"]
        pub(super) fn final_module_hit_success() -> bool;
    
        #[link_name = "_ZN3app23FighterSpecializer_Edge32set_one_winged_light_weight_dataERNS_7FighterEb"]
        pub(super) fn set_one_winged_light_weight_data(fighter: *mut app::Fighter, arg: bool);
    
        #[link_name = "_ZN3app23FighterSpecializer_Edge30set_pierce_effect_attack_air_fERNS_26BattleObjectModuleAccessorE"]
        pub(super) fn set_pierce_effect_attack_air_f(module_accessor: *mut app::BattleObjectModuleAccessor);
    
        #[link_name = "_ZN3app23FighterSpecializer_Edge26set_special_hi_jostle_areaERNS_26BattleObjectModuleAccessorE"]
        pub(super) fn set_special_hi_jostle_area(module_accessor: *mut app::BattleObjectModuleAccessor);
    
        #[link_name = "_ZN3app23FighterSpecializer_Edge18set_vec_target_posERNS_26BattleObjectModuleAccessorEiN3phx6Hash40ERKNS3_8Vector2fEj"]
        pub(super) fn set_vec_target_pos(module_accessor: *mut app::BattleObjectModuleAccessor, attack_id: i32, bone: phx::Hash40, pos: *const phx::Vector2f, frames: u32);
    
    }
}

pub fn attack_lw4_ray_check(module_accessor: &mut app::BattleObjectModuleAccessor) {
    unsafe {
        impl_::attack_lw4_ray_check(module_accessor)
    }
}

pub fn call_final_module(fighter: &mut app::Fighter, call_arg: app::FighterEdgeFinalModuleCall) {
    unsafe {
        impl_::call_final_module(fighter, call_arg)
    }
}

pub fn clear_special_hi_jostle_area(module_accessor: &mut app::BattleObjectModuleAccessor) {
    unsafe {
        impl_::clear_special_hi_jostle_area(module_accessor)
    }
}

pub fn final_module_hit_success() -> bool {
    unsafe {
        impl_::final_module_hit_success()
    }
}

pub fn set_one_winged_light_weight_data(fighter: &mut app::Fighter, arg: bool) {
    unsafe {
        impl_::set_one_winged_light_weight_data(fighter, arg)
    }
}

pub fn set_pierce_effect_attack_air_f(module_accessor: &mut app::BattleObjectModuleAccessor) {
    unsafe {
        impl_::set_pierce_effect_attack_air_f(module_accessor)
    }
}

pub fn set_special_hi_jostle_area(module_accessor: &mut app::BattleObjectModuleAccessor) {
    unsafe {
        impl_::set_special_hi_jostle_area(module_accessor)
    }
}

pub fn set_vec_target_pos(module_accessor: &mut app::BattleObjectModuleAccessor, attack_id: i32, bone: phx::Hash40, pos: &phx::Vector2f, frames: u32) {
    unsafe {
        impl_::set_vec_target_pos(module_accessor, attack_id, bone, pos, frames)
    }
}

