use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app24FighterSpecializer_Brave6add_spERNS_7FighterEf"]
        pub(super) fn add_sp(fighter: *mut app::Fighter, amount: f32);
    
        #[link_name = "_ZN3app24FighterSpecializer_Brave18adjust_flying_ceilERNS_26BattleObjectModuleAccessorEfff"]
        pub(super) fn adjust_flying_ceil(module_accessor: *mut app::BattleObjectModuleAccessor, sting_down_int: f32, angle_min: f32, angle_max: f32);
    
        #[link_name = "_ZN3app24FighterSpecializer_Brave15alloc_log_groupERNS_26BattleObjectModuleAccessorEi"]
        pub(super) fn alloc_log_group(module_accessor: *mut app::BattleObjectModuleAccessor, group: i32);
    
        #[link_name = "_ZN3app24FighterSpecializer_Brave17call_final_moduleERNS_7FighterEi"]
        pub(super) fn call_final_module(fighter: *mut app::Fighter, call_arg: app::FighterBraveFinalModuleCall);
    
        #[link_name = "_ZN3app24FighterSpecializer_Brave17check_flying_ceilERNS_26BattleObjectModuleAccessorEfff"]
        pub(super) fn check_flying_ceil(module_accessor: *mut app::BattleObjectModuleAccessor, sting_down_int: f32, angle_min: f32, angle_max: f32) -> bool;
    
        #[link_name = "_ZN3app24FighterSpecializer_Brave24final_module_hit_successEv"]
        pub(super) fn final_module_hit_success() -> bool;
    
        #[link_name = "_ZN3app24FighterSpecializer_Brave21get_flying_target_posEv"]
        pub(super) fn get_flying_target_pos() -> cpp::simd::Vector3;
    
        #[link_name = "_ZN3app24FighterSpecializer_Brave33get_special_lw_command_from_indexERNS_7FighterEi"]
        pub(super) fn get_special_lw_command_from_index(fighter: *mut app::Fighter, index: i32) -> app::FighterBraveSpecialLwCommand;
    
        #[link_name = "_ZN3app24FighterSpecializer_Brave30get_special_lw_command_sp_costERKNS_26BattleObjectModuleAccessorENS_28FighterBraveSpecialLwCommandEb"]
        pub(super) fn get_special_lw_command_sp_cost(module_accessor: *const app::BattleObjectModuleAccessor, command: app::FighterBraveSpecialLwCommand, arg: bool) -> f32;
    
        #[link_name = "_ZN3app24FighterSpecializer_Brave26get_special_lw_param_frameERNS_26BattleObjectModuleAccessorEN3phx6Hash40E"]
        pub(super) fn get_special_lw_param_frame(module_accessor: *mut app::BattleObjectModuleAccessor, param_name: phx::Hash40) -> i32;
    
        #[link_name = "_ZN3app24FighterSpecializer_Brave35get_special_lw_various_kind2commandENS_32FighterBraveSpecialLwVariousKindE"]
        pub(super) fn get_special_lw_various_kind2command(various_kind: app::FighterBraveSpecialLwVariousKind) -> app::FighterBraveSpecialLwCommand;
    
        #[link_name = "_ZN3app24FighterSpecializer_Brave34get_special_lw_various_motion_kindERNS_26BattleObjectModuleAccessorENS_32FighterBraveSpecialLwVariousKindEb"]
        pub(super) fn get_special_lw_various_motion_kind(module_accessor: *mut app::BattleObjectModuleAccessor, various_kind: app::FighterBraveSpecialLwVariousKind, arg: bool) -> phx::Hash40;
    
        #[link_name = "_ZN3app24FighterSpecializer_Brave12lot_criticalERNS_7FighterE"]
        pub(super) fn lot_critical(fighter: *mut app::Fighter);
    
        #[link_name = "_ZN3app24FighterSpecializer_Brave6set_spERNS_7FighterEfb"]
        pub(super) fn set_sp(arg1: *mut app::Fighter, amount: f32, arg: bool);
    
        #[link_name = "_ZN3app24FighterSpecializer_Brave25special_lw_active_commandERNS_7FighterE"]
        pub(super) fn special_lw_active_command(fighter: *mut app::Fighter) -> bool;
    
        #[link_name = "_ZN3app24FighterSpecializer_Brave23special_lw_close_windowERNS_7FighterEbbb"]
        pub(super) fn special_lw_close_window(fighter: *mut app::Fighter, arg2: bool, no_decide: bool, arg4: bool);
    
        #[link_name = "_ZN3app24FighterSpecializer_Brave24special_lw_cursor_decideERNS_7FighterE"]
        pub(super) fn special_lw_cursor_decide(fighter: *mut app::Fighter);
    
        #[link_name = "_ZN3app24FighterSpecializer_Brave27special_lw_deactive_commandERNS_7FighterE"]
        pub(super) fn special_lw_deactive_command(fighter: *mut app::Fighter);
    
        #[link_name = "_ZN3app24FighterSpecializer_Brave25special_lw_decide_commandERNS_7FighterENS_28FighterBraveSpecialLwCommandEi"]
        pub(super) fn special_lw_decide_command(fighter: *mut app::Fighter, command: app::FighterBraveSpecialLwCommand, idx: i32);
    
        #[link_name = "_ZN3app24FighterSpecializer_Brave27special_lw_on_start_commandERNS_7FighterE"]
        pub(super) fn special_lw_on_start_command(fighter: *mut app::Fighter);
    
        #[link_name = "_ZN3app24FighterSpecializer_Brave23special_lw_open_commandERNS_7FighterE"]
        pub(super) fn special_lw_open_command(fighter: *mut app::Fighter);
    
        #[link_name = "_ZN3app24FighterSpecializer_Brave23special_lw_select_indexERNS_7FighterEi"]
        pub(super) fn special_lw_select_index(fighter: *mut app::Fighter, index: i32);
    
        #[link_name = "_ZN3app24FighterSpecializer_Brave29special_lw_start_cursor_blinkERNS_7FighterE"]
        pub(super) fn special_lw_start_cursor_blink(fighter: *mut app::Fighter);
    
        #[link_name = "_ZN3app24FighterSpecializer_Brave31special_lw_start_select_commandERNS_7FighterE"]
        pub(super) fn special_lw_start_select_command(fighter: *mut app::Fighter);    }
}

pub fn add_sp(fighter: &mut app::Fighter, amount: f32) {
    unsafe {
        impl_::add_sp(fighter, amount)
    }
}

pub fn adjust_flying_ceil(module_accessor: &mut app::BattleObjectModuleAccessor, sting_down_int: f32, angle_min: f32, angle_max: f32) {
    unsafe {
        impl_::adjust_flying_ceil(module_accessor, sting_down_int, angle_min, angle_max)
    }
}

pub fn alloc_log_group(module_accessor: &mut app::BattleObjectModuleAccessor, group: i32) {
    unsafe {
        impl_::alloc_log_group(module_accessor, group)
    }
}

pub fn call_final_module(fighter: &mut app::Fighter, call_arg: app::FighterBraveFinalModuleCall) {
    unsafe {
        impl_::call_final_module(fighter, call_arg)
    }
}

pub fn check_flying_ceil(module_accessor: &mut app::BattleObjectModuleAccessor, sting_down_int: f32, angle_min: f32, angle_max: f32) -> bool {
    unsafe {
        impl_::check_flying_ceil(module_accessor, sting_down_int, angle_min, angle_max)
    }
}

pub fn final_module_hit_success() -> bool {
    unsafe {
        impl_::final_module_hit_success()
    }
}

pub fn get_flying_target_pos() -> phx::Vec3 {
    unsafe {
        impl_::get_flying_target_pos().into()
    }
}

pub fn get_special_lw_command_from_index(fighter: &mut app::Fighter, index: i32) -> app::FighterBraveSpecialLwCommand {
    unsafe {
        impl_::get_special_lw_command_from_index(fighter, index)
    }
}

pub fn get_special_lw_command_sp_cost(module_accessor: &app::BattleObjectModuleAccessor, command: app::FighterBraveSpecialLwCommand, arg: bool) -> f32 {
    unsafe {
        impl_::get_special_lw_command_sp_cost(module_accessor, command, arg)
    }
}

pub fn get_special_lw_param_frame(module_accessor: &mut app::BattleObjectModuleAccessor, param_name: phx::Hash40) -> i32 {
    unsafe {
        impl_::get_special_lw_param_frame(module_accessor, param_name)
    }
}

pub fn get_special_lw_various_kind2command(various_kind: app::FighterBraveSpecialLwVariousKind) -> app::FighterBraveSpecialLwCommand {
    unsafe {
        impl_::get_special_lw_various_kind2command(various_kind)
    }
}

pub fn get_special_lw_various_motion_kind(module_accessor: &mut app::BattleObjectModuleAccessor, various_kind: app::FighterBraveSpecialLwVariousKind, arg: bool) -> phx::Hash40 {
    unsafe {
        impl_::get_special_lw_various_motion_kind(module_accessor, various_kind, arg)
    }
}

pub fn lot_critical(fighter: &mut app::Fighter) {
    unsafe {
        impl_::lot_critical(fighter)
    }
}

pub fn set_sp(arg1: &mut app::Fighter, amount: f32, arg: bool) {
    unsafe {
        impl_::set_sp(arg1, amount, arg)
    }
}

pub fn special_lw_active_command(fighter: &mut app::Fighter) -> bool {
    unsafe {
        impl_::special_lw_active_command(fighter)
    }
}

pub fn special_lw_close_window(fighter: &mut app::Fighter, arg2: bool, no_decide: bool, arg4: bool) {
    unsafe {
        impl_::special_lw_close_window(fighter, arg2, no_decide, arg4)
    }
}

pub fn special_lw_cursor_decide(fighter: &mut app::Fighter) {
    unsafe {
        impl_::special_lw_cursor_decide(fighter)
    }
}

pub fn special_lw_deactive_command(fighter: &mut app::Fighter) {
    unsafe {
        impl_::special_lw_deactive_command(fighter)
    }
}

pub fn special_lw_decide_command(fighter: &mut app::Fighter, command: app::FighterBraveSpecialLwCommand, idx: i32) {
    unsafe {
        impl_::special_lw_decide_command(fighter, command, idx)
    }
}

pub fn special_lw_on_start_command(fighter: &mut app::Fighter) {
    unsafe {
        impl_::special_lw_on_start_command(fighter)
    }
}

pub fn special_lw_open_command(fighter: &mut app::Fighter) {
    unsafe {
        impl_::special_lw_open_command(fighter)
    }
}

pub fn special_lw_select_index(fighter: &mut app::Fighter, index: i32) {
    unsafe {
        impl_::special_lw_select_index(fighter, index)
    }
}

pub fn special_lw_start_cursor_blink(fighter: &mut app::Fighter) {
    unsafe {
        impl_::special_lw_start_cursor_blink(fighter)
    }
}

pub fn special_lw_start_select_command(fighter: &mut app::Fighter) {
    unsafe {
        impl_::special_lw_start_select_command(fighter)
    }
}

