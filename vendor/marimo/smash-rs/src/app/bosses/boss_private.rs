use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app12boss_private20calc_boss_hp_mul_sumENS_8ItemKindERNS_26BattleObjectModuleAccessorE"]
        pub(super) fn calc_boss_hp_mul_sum(kind: app::ItemKind, module_accessor: *mut app::BattleObjectModuleAccessor) -> f32;
    
        #[link_name = "_ZN3app12boss_private16clear_energy_allEP9lua_State"]
        pub(super) fn clear_energy_all(state: *mut lua_State);
    
        #[link_name = "_ZN3app12boss_private17clear_sub1_energyEP9lua_State"]
        pub(super) fn clear_sub1_energy(state: *mut lua_State);
    
        #[link_name = "_ZN3app12boss_private18common_param_floatENS_8ItemKindENS_20BossCommonParamFloatE"]
        pub(super) fn common_param_float(arg1: app::ItemKind, param: app::BossCommonParamFloat) -> f32;
    
        #[link_name = "_ZN3app12boss_private16common_param_intENS_8ItemKindENS_18BossCommonParamIntE"]
        pub(super) fn common_param_int(arg1: app::ItemKind, param: app::BossCommonParamInt) -> i32;
    
        #[link_name = "_ZN3app12boss_private37count_same_probability_mul_from_tableEP9lua_StateNS_8ItemKindEN3phx6Hash40Ef"]
        pub(super) fn count_same_probability_mul_from_table(state: *mut lua_State, kind: app::ItemKind, table_name: phx::Hash40, probability_mul: f32) -> i32;
    
        #[link_name = "_ZN3app12boss_private32count_target_in_range_from_paramEP9lua_StateNS_8ItemKindEN3phx6Hash40E"]
        pub(super) fn count_target_in_range_from_param(state: *mut lua_State, kind: app::ItemKind, param_name: phx::Hash40) -> i32;
    
        #[link_name = "_ZN3app12boss_private11create_bossEP9lua_StateNS_8ItemKindEi"]
        pub(super) fn create_boss(state: *mut lua_State, kind: app::ItemKind, arg3: i32) -> u32;
    
        #[link_name = "_ZN3app12boss_private13create_weaponEP9lua_StateNS_8ItemKindEffff"]
        pub(super) fn create_weapon(state: *mut lua_State, arg2: app::ItemKind, pos_x: f32, pos_y: f32, pos_z: f32, lr: f32) -> u32;
    
        #[link_name = "_ZN3app12boss_private28create_weapon_with_variationEP9lua_StateNS_8ItemKindEiffff"]
        pub(super) fn create_weapon_with_variation(state: *mut lua_State, kind: app::ItemKind, variation: i32, pos_x: f32, pos_y: f32, pos_z: f32, lr: f32) -> u32;
    
        #[link_name = "_ZN3app12boss_private26get_action_probability_mulEP9lua_StateN3phx6Hash40E"]
        pub(super) fn get_action_probability_mul(state: *mut lua_State, action: phx::Hash40) -> f32;
    
        #[link_name = "_ZN3app12boss_private30get_action_probability_mul_2ndEP9lua_StateN3phx6Hash40E"]
        pub(super) fn get_action_probability_mul_2nd(state: *mut lua_State, action: phx::Hash40) -> f32;
    
        #[link_name = "_ZN3app12boss_private14get_damage_mulEj"]
        pub(super) fn get_damage_mul(battle_object_id: u32) -> f32;
    
        #[link_name = "_ZN3app12boss_private31get_status_kind_from_alias_hashEP9lua_StateN3phx6Hash40E"]
        pub(super) fn get_status_kind_from_alias_hash(state: *mut lua_State, alias: phx::Hash40) -> i32;
    
        #[link_name = "_ZN3app12boss_private15is_boss_no_deadEv"]
        pub(super) fn is_boss_no_dead() -> bool;
    
        #[link_name = "_ZN3app12boss_private12is_boss_stopEv"]
        pub(super) fn is_boss_stop() -> bool;
    
        #[link_name = "_ZN3app12boss_private18is_multi_play_modeEb"]
        pub(super) fn is_multi_play_mode(ignore_count: bool) -> bool;
    
        #[link_name = "_ZN3app12boss_private15is_stoppable_seEv"]
        pub(super) fn is_stoppable_se() -> bool;
    
        #[link_name = "_ZN3app12boss_private15is_target_aliveEP9lua_State"]
        pub(super) fn is_target_alive(state: *mut lua_State) -> bool;
    
        #[link_name = "_ZN3app12boss_private22main_energy_from_paramEP9lua_StateNS_8ItemKindEN3phx6Hash40Ef"]
        pub(super) fn main_energy_from_param(state: *mut lua_State, kind: app::ItemKind, param_struct_name: phx::Hash40, angle_deg: f32);
    
        #[link_name = "_ZN3app12boss_private30main_energy_from_param_inheritEP9lua_StateNS_8ItemKindEN3phx6Hash40E"]
        pub(super) fn main_energy_from_param_inherit(state: *mut lua_State, kind: app::ItemKind, param_struct_name: phx::Hash40);
    
        #[link_name = "_ZN3app12boss_private38main_energy_from_param_inherit_no_bossEP9lua_StateNS_8ItemKindEN3phx6Hash40E"]
        pub(super) fn main_energy_from_param_inherit_no_boss(state: *mut lua_State, kind: app::ItemKind, param_struct_name: phx::Hash40);
    
        #[link_name = "_ZN3app12boss_private30main_energy_from_param_no_bossEP9lua_StateNS_8ItemKindEN3phx6Hash40Ef"]
        pub(super) fn main_energy_from_param_no_boss(state: *mut lua_State, kind: app::ItemKind, param_struct_name: phx::Hash40, angle_deg: f32);
    
        #[link_name = "_ZN3app12boss_private13search_targetEP9lua_StateNS_8ItemKindEf"]
        pub(super) fn search_target(state: *mut lua_State, kind: app::ItemKind, radius: f32);
    
        #[link_name = "_ZN3app12boss_private13select_actionEP9lua_StateNS_8ItemKindEN3phx6Hash40ES5_"]
        pub(super) fn select_action(state: *mut lua_State, kind: app::ItemKind, param_struct_name: phx::Hash40, param_name: phx::Hash40) -> phx::Hash40;
    
        #[link_name = "_ZN3app12boss_private16self_param_floatEP9lua_StateNS_8ItemKindEN3phx6Hash40E"]
        pub(super) fn self_param_float(state: *mut lua_State, kind: app::ItemKind, param_name: phx::Hash40) -> f32;
    
        #[link_name = "_ZN3app12boss_private14self_param_intEP9lua_StateNS_8ItemKindEN3phx6Hash40E"]
        pub(super) fn self_param_int(state: *mut lua_State, kind: app::ItemKind, param_name: phx::Hash40) -> i32;
    
        #[link_name = "_ZN3app12boss_private23send_event_on_boss_deadEP9lua_State"]
        pub(super) fn send_event_on_boss_dead(state: *mut lua_State);
    
        #[link_name = "_ZN3app12boss_private25send_event_on_boss_defeatEP9lua_State"]
        pub(super) fn send_event_on_boss_defeat(state: *mut lua_State);
    
        #[link_name = "_ZN3app12boss_private25send_event_on_boss_finishEP9lua_StateN3phx6Hash40E"]
        pub(super) fn send_event_on_boss_finish(state: *mut lua_State, arg2: phx::Hash40);
    
        #[link_name = "_ZN3app12boss_private29send_event_on_boss_keyoff_bgmEP9lua_State"]
        pub(super) fn send_event_on_boss_keyoff_bgm(state: *mut lua_State);
    
        #[link_name = "_ZN3app12boss_private30send_event_on_start_boss_entryEj"]
        pub(super) fn send_event_on_start_boss_entry(battle_object_id: u32);
    
        #[link_name = "_ZN3app12boss_private26set_action_probability_mulEP9lua_StateN3phx6Hash40Ef"]
        pub(super) fn set_action_probability_mul(state: *mut lua_State, action: phx::Hash40, mul: f32);
    
        #[link_name = "_ZN3app12boss_private30set_action_probability_mul_2ndEP9lua_StateN3phx6Hash40Ef"]
        pub(super) fn set_action_probability_mul_2nd(state: *mut lua_State, action: phx::Hash40, mul: f32);
    
        #[link_name = "_ZN3app12boss_private41set_action_probability_mul_2nd_from_tableEP9lua_StateNS_8ItemKindEN3phx6Hash40Ef"]
        pub(super) fn set_action_probability_mul_2nd_from_table(state: *mut lua_State, kind: app::ItemKind, action: phx::Hash40, mul: f32);
    
        #[link_name = "_ZN3app12boss_private37set_action_probability_mul_from_tableEP9lua_StateNS_8ItemKindEN3phx6Hash40Ef"]
        pub(super) fn set_action_probability_mul_from_table(state: *mut lua_State, kind: app::ItemKind, action: phx::Hash40, mul: f32);
    
        #[link_name = "_ZN3app12boss_private12set_stage_shEP9lua_State"]
        pub(super) fn set_stage_sh(state: *mut lua_State);
    
        #[link_name = "_ZN3app12boss_private21set_sub1_energy_angleEP9lua_Statef"]
        pub(super) fn set_sub1_energy_angle(state: *mut lua_State, angle: f32);
    
        #[link_name = "_ZN3app12boss_private22sub1_energy_from_paramEP9lua_StateNS_8ItemKindEN3phx6Hash40Ef"]
        pub(super) fn sub1_energy_from_param(state: *mut lua_State, kind: app::ItemKind, param_name: phx::Hash40, angle: f32);
    
        #[link_name = "_ZN3app12boss_private30sub1_energy_from_param_inheritEP9lua_StateNS_8ItemKindEN3phx6Hash40E"]
        pub(super) fn sub1_energy_from_param_inherit(state: *mut lua_State, kind: app::ItemKind, param_name: phx::Hash40);
    
        #[link_name = "_ZN3app12boss_private34sub1_energy_from_param_inherit_allEP9lua_StateNS_8ItemKindEN3phx6Hash40E"]
        pub(super) fn sub1_energy_from_param_inherit_all(state: *mut lua_State, kind: app::ItemKind, param_name: phx::Hash40);
    
        #[link_name = "_ZN3app12boss_private42sub1_energy_from_param_inherit_all_no_bossEP9lua_StateNS_8ItemKindEN3phx6Hash40E"]
        pub(super) fn sub1_energy_from_param_inherit_all_no_boss(state: *mut lua_State, kind: app::ItemKind, param_name: phx::Hash40);
    
        #[link_name = "_ZN3app12boss_private38sub1_energy_from_param_inherit_no_bossEP9lua_StateNS_8ItemKindEN3phx6Hash40E"]
        pub(super) fn sub1_energy_from_param_inherit_no_boss(state: *mut lua_State, kind: app::ItemKind, param_name: phx::Hash40);
    
        #[link_name = "_ZN3app12boss_private30sub1_energy_from_param_no_bossEP9lua_StateNS_8ItemKindEN3phx6Hash40Ef"]
        pub(super) fn sub1_energy_from_param_no_boss(state: *mut lua_State, kind: app::ItemKind, param_name: phx::Hash40, angle: f32);
    
        #[link_name = "_ZN3app12boss_private17unable_energy_allEP9lua_State"]
        pub(super) fn unable_energy_all(state: *mut lua_State);
    
        #[link_name = "_ZN3app12boss_private18unable_main_energyEP9lua_State"]
        pub(super) fn unable_main_energy(state: *mut lua_State);
    
        #[link_name = "_ZN3app12boss_private18unable_sub1_energyEP9lua_State"]
        pub(super) fn unable_sub1_energy(state: *mut lua_State);
    
    }
}

pub fn calc_boss_hp_mul_sum(kind: app::ItemKind, module_accessor: &mut app::BattleObjectModuleAccessor) -> f32 {
    unsafe {
        impl_::calc_boss_hp_mul_sum(kind, module_accessor)
    }
}

pub fn clear_energy_all(state: *mut lua_State) {
    unsafe {
        impl_::clear_energy_all(state)
    }
}

pub fn clear_sub1_energy(state: *mut lua_State) {
    unsafe {
        impl_::clear_sub1_energy(state)
    }
}

pub fn common_param_float(arg1: app::ItemKind, param: app::BossCommonParamFloat) -> f32 {
    unsafe {
        impl_::common_param_float(arg1, param)
    }
}

pub fn common_param_int(arg1: app::ItemKind, param: app::BossCommonParamInt) -> i32 {
    unsafe {
        impl_::common_param_int(arg1, param)
    }
}

pub fn count_same_probability_mul_from_table(state: *mut lua_State, kind: app::ItemKind, table_name: phx::Hash40, probability_mul: f32) -> i32 {
    unsafe {
        impl_::count_same_probability_mul_from_table(state, kind, table_name, probability_mul)
    }
}

pub fn count_target_in_range_from_param(state: *mut lua_State, kind: app::ItemKind, param_name: phx::Hash40) -> i32 {
    unsafe {
        impl_::count_target_in_range_from_param(state, kind, param_name)
    }
}

pub fn create_boss(state: *mut lua_State, kind: app::ItemKind, arg3: i32) -> u32 {
    unsafe {
        impl_::create_boss(state, kind, arg3)
    }
}

pub fn create_weapon(state: *mut lua_State, arg2: app::ItemKind, pos_x: f32, pos_y: f32, pos_z: f32, lr: f32) -> u32 {
    unsafe {
        impl_::create_weapon(state, arg2, pos_x, pos_y, pos_z, lr)
    }
}

pub fn create_weapon_with_variation(state: *mut lua_State, kind: app::ItemKind, variation: i32, pos_x: f32, pos_y: f32, pos_z: f32, lr: f32) -> u32 {
    unsafe {
        impl_::create_weapon_with_variation(state, kind, variation, pos_x, pos_y, pos_z, lr)
    }
}

pub fn get_action_probability_mul(state: *mut lua_State, action: phx::Hash40) -> f32 {
    unsafe {
        impl_::get_action_probability_mul(state, action)
    }
}

pub fn get_action_probability_mul_2nd(state: *mut lua_State, action: phx::Hash40) -> f32 {
    unsafe {
        impl_::get_action_probability_mul_2nd(state, action)
    }
}

pub fn get_damage_mul(battle_object_id: u32) -> f32 {
    unsafe {
        impl_::get_damage_mul(battle_object_id)
    }
}

pub fn get_status_kind_from_alias_hash(state: *mut lua_State, alias: phx::Hash40) -> i32 {
    unsafe {
        impl_::get_status_kind_from_alias_hash(state, alias)
    }
}

pub fn is_boss_no_dead() -> bool {
    unsafe {
        impl_::is_boss_no_dead()
    }
}

pub fn is_boss_stop() -> bool {
    unsafe {
        impl_::is_boss_stop()
    }
}

pub fn is_multi_play_mode(ignore_count: bool) -> bool {
    unsafe {
        impl_::is_multi_play_mode(ignore_count)
    }
}

pub fn is_stoppable_se() -> bool {
    unsafe {
        impl_::is_stoppable_se()
    }
}

pub fn is_target_alive(state: *mut lua_State) -> bool {
    unsafe {
        impl_::is_target_alive(state)
    }
}

pub fn main_energy_from_param(state: *mut lua_State, kind: app::ItemKind, param_struct_name: phx::Hash40, angle_deg: f32) {
    unsafe {
        impl_::main_energy_from_param(state, kind, param_struct_name, angle_deg)
    }
}

pub fn main_energy_from_param_inherit(state: *mut lua_State, kind: app::ItemKind, param_struct_name: phx::Hash40) {
    unsafe {
        impl_::main_energy_from_param_inherit(state, kind, param_struct_name)
    }
}

pub fn main_energy_from_param_inherit_no_boss(state: *mut lua_State, kind: app::ItemKind, param_struct_name: phx::Hash40) {
    unsafe {
        impl_::main_energy_from_param_inherit_no_boss(state, kind, param_struct_name)
    }
}

pub fn main_energy_from_param_no_boss(state: *mut lua_State, kind: app::ItemKind, param_struct_name: phx::Hash40, angle_deg: f32) {
    unsafe {
        impl_::main_energy_from_param_no_boss(state, kind, param_struct_name, angle_deg)
    }
}

pub fn search_target(state: *mut lua_State, kind: app::ItemKind, radius: f32) {
    unsafe {
        impl_::search_target(state, kind, radius)
    }
}

pub fn select_action(state: *mut lua_State, kind: app::ItemKind, param_struct_name: phx::Hash40, param_name: phx::Hash40) -> phx::Hash40 {
    unsafe {
        impl_::select_action(state, kind, param_struct_name, param_name)
    }
}

pub fn self_param_float(state: *mut lua_State, kind: app::ItemKind, param_name: phx::Hash40) -> f32 {
    unsafe {
        impl_::self_param_float(state, kind, param_name)
    }
}

pub fn self_param_int(state: *mut lua_State, kind: app::ItemKind, param_name: phx::Hash40) -> i32 {
    unsafe {
        impl_::self_param_int(state, kind, param_name)
    }
}

pub fn send_event_on_boss_dead(state: *mut lua_State) {
    unsafe {
        impl_::send_event_on_boss_dead(state)
    }
}

pub fn send_event_on_boss_defeat(state: *mut lua_State) {
    unsafe {
        impl_::send_event_on_boss_defeat(state)
    }
}

pub fn send_event_on_boss_finish(state: *mut lua_State, arg2: phx::Hash40) {
    unsafe {
        impl_::send_event_on_boss_finish(state, arg2)
    }
}

pub fn send_event_on_boss_keyoff_bgm(state: *mut lua_State) {
    unsafe {
        impl_::send_event_on_boss_keyoff_bgm(state)
    }
}

pub fn send_event_on_start_boss_entry(battle_object_id: u32) {
    unsafe {
        impl_::send_event_on_start_boss_entry(battle_object_id)
    }
}

pub fn set_action_probability_mul(state: *mut lua_State, action: phx::Hash40, mul: f32) {
    unsafe {
        impl_::set_action_probability_mul(state, action, mul)
    }
}

pub fn set_action_probability_mul_2nd(state: *mut lua_State, action: phx::Hash40, mul: f32) {
    unsafe {
        impl_::set_action_probability_mul_2nd(state, action, mul)
    }
}

pub fn set_action_probability_mul_2nd_from_table(state: *mut lua_State, kind: app::ItemKind, action: phx::Hash40, mul: f32) {
    unsafe {
        impl_::set_action_probability_mul_2nd_from_table(state, kind, action, mul)
    }
}

pub fn set_action_probability_mul_from_table(state: *mut lua_State, kind: app::ItemKind, action: phx::Hash40, mul: f32) {
    unsafe {
        impl_::set_action_probability_mul_from_table(state, kind, action, mul)
    }
}

pub fn set_stage_sh(state: *mut lua_State) {
    unsafe {
        impl_::set_stage_sh(state)
    }
}

pub fn set_sub1_energy_angle(state: *mut lua_State, angle: f32) {
    unsafe {
        impl_::set_sub1_energy_angle(state, angle)
    }
}

pub fn sub1_energy_from_param(state: *mut lua_State, kind: app::ItemKind, param_name: phx::Hash40, angle: f32) {
    unsafe {
        impl_::sub1_energy_from_param(state, kind, param_name, angle)
    }
}

pub fn sub1_energy_from_param_inherit(state: *mut lua_State, kind: app::ItemKind, param_name: phx::Hash40) {
    unsafe {
        impl_::sub1_energy_from_param_inherit(state, kind, param_name)
    }
}

pub fn sub1_energy_from_param_inherit_all(state: *mut lua_State, kind: app::ItemKind, param_name: phx::Hash40) {
    unsafe {
        impl_::sub1_energy_from_param_inherit_all(state, kind, param_name)
    }
}

pub fn sub1_energy_from_param_inherit_all_no_boss(state: *mut lua_State, kind: app::ItemKind, param_name: phx::Hash40) {
    unsafe {
        impl_::sub1_energy_from_param_inherit_all_no_boss(state, kind, param_name)
    }
}

pub fn sub1_energy_from_param_inherit_no_boss(state: *mut lua_State, kind: app::ItemKind, param_name: phx::Hash40) {
    unsafe {
        impl_::sub1_energy_from_param_inherit_no_boss(state, kind, param_name)
    }
}

pub fn sub1_energy_from_param_no_boss(state: *mut lua_State, kind: app::ItemKind, param_name: phx::Hash40, angle: f32) {
    unsafe {
        impl_::sub1_energy_from_param_no_boss(state, kind, param_name, angle)
    }
}

pub fn unable_energy_all(state: *mut lua_State) {
    unsafe {
        impl_::unable_energy_all(state)
    }
}

pub fn unable_main_energy(state: *mut lua_State) {
    unsafe {
        impl_::unable_main_energy(state)
    }
}

pub fn unable_sub1_energy(state: *mut lua_State) {
    unsafe {
        impl_::unable_sub1_energy(state)
    }
}