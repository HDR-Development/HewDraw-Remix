use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app24FighterSpecializer_Demon14add_attack_logERNS_7FighterEib"]
        pub(super) fn add_attack_log(arg1: *mut app::Fighter, arg2: i32, arg3: bool);
    
        #[link_name = "_ZN3app24FighterSpecializer_Demon17call_final_moduleERNS_7FighterEi"]
        pub(super) fn call_final_module(fighter: *mut app::Fighter, call_arg: app::FighterDemonFinalModuleCall);
    
        #[link_name = "_ZN3app24FighterSpecializer_Demon37check_disabled_motion_camera_of_scaleERNS_26BattleObjectModuleAccessorE"]
        pub(super) fn check_disabled_motion_camera_of_scale(module_accessor: *mut app::BattleObjectModuleAccessor);
    
        #[link_name = "_ZN3app24FighterSpecializer_Demon37check_disabled_motion_camera_of_stageERNS_26BattleObjectModuleAccessorE"]
        pub(super) fn check_disabled_motion_camera_of_stage(module_accessor: *mut app::BattleObjectModuleAccessor);
    
        #[link_name = "_ZN3app24FighterSpecializer_Demon20clear_past_log_throwERNS_26BattleObjectModuleAccessorE"]
        pub(super) fn clear_past_log_throw(module_accessor: *mut app::BattleObjectModuleAccessor);
    
        #[link_name = "_ZN3app24FighterSpecializer_Demon24final_module_hit_successEv"]
        pub(super) fn final_module_hit_success() -> bool;
    
        #[link_name = "_ZN3app24FighterSpecializer_Demon44is_enable_motion_camera_ray_check_special_lwERNS_26BattleObjectModuleAccessorE"]
        pub(super) fn is_enable_motion_camera_ray_check_special_lw(module_accessor: *mut app::BattleObjectModuleAccessor) -> bool;
    
        #[link_name = "_ZN3app24FighterSpecializer_Demon27request_critical_hit_cut_inERNS_26BattleObjectModuleAccessorE"]
        pub(super) fn request_critical_hit_cut_in(module_accessor: *mut app::BattleObjectModuleAccessor) -> bool;
    
        #[link_name = "_ZN3app24FighterSpecializer_Demon9set_devilERNS_26BattleObjectModuleAccessorEbf"]
        pub(super) fn set_devil(module_accessor: *mut app::BattleObjectModuleAccessor, enable: bool, wing_start_frame: f32);
    
        #[link_name = "_ZN3app24FighterSpecializer_Demon22set_jostle_area_offsetERNS_21FighterModuleAccessorERKN3phx8Vector2fE"]
        pub(super) fn set_jostle_area_offset(module_accessor: *mut app::FighterModuleAccessor, offset: *const phx::Vector2f);
    
        #[link_name = "_ZN3app24FighterSpecializer_Demon15sub_rage_systemERNS_7FighterEb"]
        pub(super) fn sub_rage_system(arg1: *mut app::Fighter, arg: bool);
    
        #[link_name = "_ZN3app24FighterSpecializer_Demon23update_opponent_lr_1on1ERNS_26BattleObjectModuleAccessorEi"]
        pub(super) fn update_opponent_lr_1on1(module_accessor: *mut app::BattleObjectModuleAccessor, status_kind: i32);
    
    }
}

pub fn add_attack_log(arg1: &mut app::Fighter, arg2: i32, arg3: bool) {
    unsafe {
        impl_::add_attack_log(arg1, arg2, arg3)
    }
}

pub fn call_final_module(fighter: &mut app::Fighter, call_arg: app::FighterDemonFinalModuleCall) {
    unsafe {
        impl_::call_final_module(fighter, call_arg)
    }
}

pub fn check_disabled_motion_camera_of_scale(module_accessor: &mut app::BattleObjectModuleAccessor) {
    unsafe {
        impl_::check_disabled_motion_camera_of_scale(module_accessor)
    }
}

pub fn check_disabled_motion_camera_of_stage(module_accessor: &mut app::BattleObjectModuleAccessor) {
    unsafe {
        impl_::check_disabled_motion_camera_of_stage(module_accessor)
    }
}

pub fn clear_past_log_throw(module_accessor: &mut app::BattleObjectModuleAccessor) {
    unsafe {
        impl_::clear_past_log_throw(module_accessor)
    }
}

pub fn final_module_hit_success() -> bool {
    unsafe {
        impl_::final_module_hit_success()
    }
}

pub fn is_enable_motion_camera_ray_check_special_lw(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    unsafe {
        impl_::is_enable_motion_camera_ray_check_special_lw(module_accessor)
    }
}

pub fn request_critical_hit_cut_in(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    unsafe {
        impl_::request_critical_hit_cut_in(module_accessor)
    }
}

pub fn set_devil(module_accessor: &mut app::BattleObjectModuleAccessor, enable: bool, wing_start_frame: f32) {
    unsafe {
        impl_::set_devil(module_accessor, enable, wing_start_frame)
    }
}

pub fn set_jostle_area_offset(module_accessor: &mut app::FighterModuleAccessor, offset: &phx::Vector2f) {
    unsafe {
        impl_::set_jostle_area_offset(module_accessor, offset)
    }
}

pub fn sub_rage_system(arg1: &mut app::Fighter, arg: bool) {
    unsafe {
        impl_::sub_rage_system(arg1, arg)
    }
}

pub fn update_opponent_lr_1on1(module_accessor: &mut app::BattleObjectModuleAccessor, status_kind: i32) {
    unsafe {
        impl_::update_opponent_lr_1on1(module_accessor, status_kind)
    }
}

