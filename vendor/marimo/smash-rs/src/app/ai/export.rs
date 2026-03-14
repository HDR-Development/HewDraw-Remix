use crate::*;

use super::impl_;

pub fn target_id(state: *mut lua_State) -> u32 {
    unsafe {
        impl_::target_id(state)
    }
}

pub fn is_valid_target(state: *mut lua_State) -> bool {
    unsafe {
        impl_::is_valid_target(state)
    }
}

pub fn fighter_kind(state: *mut lua_State) -> app::FighterKind {
    unsafe {
        impl_::fighter_kind(state)
    }
}

pub fn copy_fighter_kind(state: *mut lua_State) -> app::FighterKind {
    unsafe {
        impl_::copy_fighter_kind(state)
    }
}

pub fn rank(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::rank(state)
    }
}

pub fn cp_type(state: *mut lua_State) -> app::FighterAICPType {
    unsafe {
        impl_::cp_type(state)
    }
}

pub fn cp_flag(state: *mut lua_State) -> app::FighterAICPFlag {
    unsafe {
        impl_::cp_flag(state)
    }
}

pub fn cp_slide_type(state: *mut lua_State) -> app::FighterAICPType {
    unsafe {
        impl_::cp_slide_type(state)
    }
}

pub fn act_id(state: *mut lua_State) -> app::FighterAIAct::CmdId {
    unsafe {
        impl_::act_id(state)
    }
}

pub fn current_attack_cancel_frame(state: *mut lua_State) -> i32 {
    unsafe {
        impl_::current_attack_cancel_frame(state)
    }
}

pub fn uniq_stat(state: *mut lua_State) -> app::FighterAIUniqStat {
    unsafe {
        impl_::uniq_stat(state)
    }
}

pub fn attack_phase(state: *mut lua_State) -> app::FighterAIAttackPhase {
    unsafe {
        impl_::attack_phase(state)
    }
}

pub fn check_stat_air(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_air(state)
    }
}

pub fn check_stat_build_max(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_build_max(state)
    }
}

pub fn check_stat_build_up(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_build_up(state)
    }
}

pub fn check_stat_gorogoro(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_gorogoro(state)
    }
}

pub fn check_stat_attention(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_attention(state)
    }
}

pub fn check_stat_final(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_final(state)
    }
}

pub fn check_stat_final_act(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_final_act(state)
    }
}

pub fn check_stat_invincible(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_invincible(state)
    }
}

pub fn check_stat_invincible_l(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_invincible_l(state)
    }
}

pub fn check_stat_damage_elec(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_damage_elec(state)
    }
}

pub fn check_stat_sp_dir(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_sp_dir(state)
    }
}

pub fn check_stat_unguarded_hind(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_unguarded_hind(state)
    }
}

pub fn check_stat_unguarded(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_unguarded(state)
    }
}

pub fn check_stat_dash(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_dash(state)
    }
}

pub fn check_stat_down(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_down(state)
    }
}

pub fn check_stat_piyo(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_piyo(state)
    }
}

pub fn check_stat_dragoon(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_dragoon(state)
    }
}

pub fn check_stat_genesis(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_genesis(state)
    }
}

pub fn check_stat_catch(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_catch(state)
    }
}

pub fn check_stat_damage(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_damage(state)
    }
}

pub fn check_stat_guard(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_guard(state)
    }
}

pub fn check_stat_attack_hold(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_attack_hold(state)
    }
}

pub fn check_stat_floor_pass(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_floor_pass(state)
    }
}

pub fn check_stat_floor_damage(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_floor_damage(state)
    }
}

pub fn check_stat_ground_free(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_ground_free(state)
    }
}

pub fn check_stat_ground_free2(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_ground_free2(state)
    }
}

pub fn check_stat_air_free(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_air_free(state)
    }
}

pub fn check_stat_touch_u(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_touch_u(state)
    }
}

pub fn check_stat_touch_l(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_touch_l(state)
    }
}

pub fn check_stat_touch_r(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_touch_r(state)
    }
}

pub fn check_stat_cannot_catch_cliff(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_cannot_catch_cliff(state)
    }
}

pub fn check_stat_dive(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_dive(state)
    }
}

pub fn check_stat_unable_cliff_xlu(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_unable_cliff_xlu(state)
    }
}

pub fn check_stat_unable_escape_air(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_unable_escape_air(state)
    }
}

pub fn check_stat_unable_attack(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_unable_attack(state)
    }
}

pub fn check_stat_unable_special(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_unable_special(state)
    }
}

pub fn check_stat_unable_jump(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_unable_jump(state)
    }
}

pub fn check_stat_unable_shield(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_unable_shield(state)
    }
}

pub fn check_stat_have(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_have(state)
    }
}

pub fn check_stat_put_bomb(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_put_bomb(state)
    }
}

pub fn check_stat_can_use_superleaf(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_can_use_superleaf(state)
    }
}

pub fn check_stat_can_use_rocketbelt(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_can_use_rocketbelt(state)
    }
}

pub fn check_stat_have_throw(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_have_throw(state)
    }
}

pub fn check_stat_have_shoot(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_have_shoot(state)
    }
}

pub fn check_stat_have_swing(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_have_swing(state)
    }
}

pub fn check_stat_dogs_blind_own(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_dogs_blind_own(state)
    }
}

pub fn check_stat_target_invisible(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_stat_target_invisible(state)
    }
}

pub fn check_skill_stat(state: *mut lua_State, stat: app::FighterAIStat::SkillStatus) -> bool {
    unsafe {
        impl_::check_skill_stat(state, stat)
    }
}

pub fn check_spirits_event_stat(state: *mut lua_State, stat: app::FighterAIStat::SpiritsEventStatus) -> bool {
    unsafe {
        impl_::check_spirits_event_stat(state, stat)
    }
}

pub fn check_chr_stat(state: *mut lua_State, stat: app::FighterAIStat::CharacterStatus, fighter_kind: app::FighterKind) -> bool {
    unsafe {
        impl_::check_chr_stat(state, stat, fighter_kind)
    }
}

pub fn air_lasso_count(state: *mut lua_State) -> i32 {
    unsafe {
        impl_::air_lasso_count(state)
    }
}

pub fn check_cliffable(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_cliffable(state)
    }
}

pub fn check_cliffable_floor_lr(state: *mut lua_State, lr: f32) -> bool {
    unsafe {
        impl_::check_cliffable_floor_lr(state, lr)
    }
}

pub fn check_passable(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_passable(state)
    }
}

pub fn shield_rate(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::shield_rate(state)
    }
}

pub fn damage_reaction_mul(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::damage_reaction_mul(state)
    }
}

pub fn stop_frame(state: *mut lua_State) -> i32 {
    unsafe {
        impl_::stop_frame(state)
    }
}

pub fn height(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::height(state)
    }
}

pub fn pos_x(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::pos_x(state)
    }
}

pub fn pos_y(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::pos_y(state)
    }
}

pub fn speed_x(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::speed_x(state)
    }
}

pub fn speed_y(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::speed_y(state)
    }
}

pub fn scale(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::scale(state)
    }
}

pub fn status_kind(state: *mut lua_State) -> i32 {
    unsafe {
        impl_::status_kind(state)
    }
}

pub fn prev_status_kind(state: *mut lua_State) -> i32 {
    unsafe {
        impl_::prev_status_kind(state)
    }
}

pub fn motion_kind(state: *mut lua_State) -> phx::Hash40 {
    unsafe {
        impl_::motion_kind(state)
    }
}

pub fn motion_frame(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::motion_frame(state)
    }
}

pub fn motion_rate(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::motion_rate(state)
    }
}

pub fn motion_cancel_frame(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::motion_cancel_frame(state)
    }
}

pub fn jump_rest_available(state: *mut lua_State) -> u16 {
    unsafe {
        impl_::jump_rest_available(state)
    }
}

pub fn is_sp_u_available(state: *mut lua_State) -> bool {
    unsafe {
        impl_::is_sp_u_available(state)
    }
}

pub fn is_sp_u_weaken_available(state: *mut lua_State) -> bool {
    unsafe {
        impl_::is_sp_u_weaken_available(state)
    }
}

pub fn damage(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::damage(state)
    }
}

pub fn hp(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::hp(state)
    }
}

pub fn lr(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::lr(state)
    }
}

pub fn customize_n(state: *mut lua_State) -> i32 {
    unsafe {
        impl_::customize_n(state)
    }
}

pub fn customize_s(state: *mut lua_State) -> i32 {
    unsafe {
        impl_::customize_s(state)
    }
}

pub fn customize_hi(state: *mut lua_State) -> i32 {
    unsafe {
        impl_::customize_hi(state)
    }
}

pub fn customize_lw(state: *mut lua_State) -> i32 {
    unsafe {
        impl_::customize_lw(state)
    }
}

pub fn check_use_command(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_use_command(state)
    }
}

pub fn check_use_command_type(state: *mut lua_State) -> u8 {
    unsafe {
        impl_::check_use_command_type(state)
    }
}

pub fn check_command_236_step(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_command_236_step(state)
    }
}

pub fn check_command_41236_step(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_command_41236_step(state)
    }
}

pub fn check_command_214_step(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_command_214_step(state)
    }
}

pub fn check_command_623_step(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_command_623_step(state)
    }
}

pub fn check_command_236236_step(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_command_236236_step(state)
    }
}

pub fn check_command_21416_step(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_command_21416_step(state)
    }
}

pub fn check_command_214214_step(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_command_214214_step(state)
    }
}

pub fn check_command_23634_step(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_command_23634_step(state)
    }
}

pub fn fighter_uniq_value(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::fighter_uniq_value(state)
    }
}

pub fn fighter_uniq_value2(state: *mut lua_State, uniq_value_idx: i32) -> f32 {
    unsafe {
        impl_::fighter_uniq_value2(state, uniq_value_idx)
    }
}

pub fn line_segment_check(state: *mut lua_State, line: &phx::Vector2f) -> bool {
    unsafe {
        impl_::line_segment_check(state, line)
    }
}

pub fn line_segment_check_from_top_n(state: *mut lua_State, line: &phx::Vector2f, top: &phx::Vector2f) -> bool {
    unsafe {
        impl_::line_segment_check_from_top_n(state, line, top)
    }
}

pub fn line_segment_check_only_roof(state: *mut lua_State, line: &phx::Vector2f) -> bool {
    unsafe {
        impl_::line_segment_check_only_roof(state, line)
    }
}

pub fn line_segment_check_only_floor(state: *mut lua_State, line: &phx::Vector2f) -> bool {
    unsafe {
        impl_::line_segment_check_only_floor(state, line)
    }
}

pub fn line_segment_check_only_wall(state: *mut lua_State, line: &phx::Vector2f) -> bool {
    unsafe {
        impl_::line_segment_check_only_wall(state, line)
    }
}

pub fn weapon_pos_x(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::weapon_pos_x(state)
    }
}

pub fn weapon_pos_y(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::weapon_pos_y(state)
    }
}

pub fn weapon_speed_x(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::weapon_speed_x(state)
    }
}

pub fn weapon_speed_y(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::weapon_speed_y(state)
    }
}

pub fn target_fighter_kind(state: *mut lua_State) -> app::FighterKind {
    unsafe {
        impl_::target_fighter_kind(state)
    }
}

pub fn target_copy_fighter_kind(state: *mut lua_State) -> app::FighterKind {
    unsafe {
        impl_::target_copy_fighter_kind(state)
    }
}

pub fn target_current_attack_start_frame(state: *mut lua_State) -> i32 {
    unsafe {
        impl_::target_current_attack_start_frame(state)
    }
}

pub fn target_current_attack_combo_end_frame(state: *mut lua_State) -> i32 {
    unsafe {
        impl_::target_current_attack_combo_end_frame(state)
    }
}

pub fn target_current_attack_cancel_frame(state: *mut lua_State) -> i32 {
    unsafe {
        impl_::target_current_attack_cancel_frame(state)
    }
}

pub fn check_target_stat_air(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_air(state)
    }
}

pub fn check_target_stat_build_up(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_build_up(state)
    }
}

pub fn check_target_stat_attention(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_attention(state)
    }
}

pub fn check_target_stat_final(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_final(state)
    }
}

pub fn check_target_stat_dead(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_dead(state)
    }
}

pub fn check_target_stat_invincible(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_invincible(state)
    }
}

pub fn check_target_stat_invincible_l(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_invincible_l(state)
    }
}

pub fn check_target_stat_attack_catch(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_attack_catch(state)
    }
}

pub fn check_target_stat_reflect(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_reflect(state)
    }
}

pub fn check_target_stat_unguarded_hind(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_unguarded_hind(state)
    }
}

pub fn check_target_stat_unguarded(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_unguarded(state)
    }
}

pub fn check_target_stat_combo(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_combo(state)
    }
}

pub fn check_target_stat_no_counter(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_no_counter(state)
    }
}

pub fn check_target_stat_down(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_down(state)
    }
}

pub fn check_target_stat_fall_sp(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_fall_sp(state)
    }
}

pub fn check_target_stat_piyo(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_piyo(state)
    }
}

pub fn check_target_stat_piyo_l(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_piyo_l(state)
    }
}

pub fn check_target_stat_dragoon(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_dragoon(state)
    }
}

pub fn check_target_stat_cliff(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_cliff(state)
    }
}

pub fn check_target_stat_cliff_act(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_cliff_act(state)
    }
}

pub fn check_target_stat_catch(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_catch(state)
    }
}

pub fn check_target_stat_damage(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_damage(state)
    }
}

pub fn check_target_stat_guard(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_guard(state)
    }
}

pub fn check_target_stat_escape(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_escape(state)
    }
}

pub fn check_target_stat_rebirth(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_rebirth(state)
    }
}

pub fn check_target_stat_attack(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_attack(state)
    }
}

pub fn check_target_stat_attack_hold(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_attack_hold(state)
    }
}

pub fn check_target_stat_squat(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_squat(state)
    }
}

pub fn check_target_stat_unable_attack(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_unable_attack(state)
    }
}

pub fn check_target_stat_unable_special(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_unable_special(state)
    }
}

pub fn check_target_stat_specialflag_hoist(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_stat_specialflag_hoist(state)
    }
}

pub fn check_target_chr_stat(state: *mut lua_State, stat: app::FighterAIStat::CharacterStatus, fighter_kind: app::FighterKind) -> bool {
    unsafe {
        impl_::check_target_chr_stat(state, stat, fighter_kind)
    }
}

pub fn target_width(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::target_width(state)
    }
}

pub fn target_height(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::target_height(state)
    }
}

pub fn target_pos_x(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::target_pos_x(state)
    }
}

pub fn target_pos_y(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::target_pos_y(state)
    }
}

pub fn target_speed_x(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::target_speed_x(state)
    }
}

pub fn target_speed_y(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::target_speed_y(state)
    }
}

pub fn target_scale(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::target_scale(state)
    }
}

pub fn target_jump_rest_available(state: *mut lua_State) -> u16 {
    unsafe {
        impl_::target_jump_rest_available(state)
    }
}

pub fn target_damage(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::target_damage(state)
    }
}

pub fn target_lr(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::target_lr(state)
    }
}

pub fn target_status_kind(state: *mut lua_State) -> i32 {
    unsafe {
        impl_::target_status_kind(state)
    }
}

pub fn target_motion_kind(state: *mut lua_State) -> phx::Hash40 {
    unsafe {
        impl_::target_motion_kind(state)
    }
}

pub fn target_motion_frame(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::target_motion_frame(state)
    }
}

pub fn target_hit_collision_rect(state: *mut lua_State) -> lib::Rect {
    unsafe {
        impl_::target_hit_collision_rect(state).into()
    }
}

pub fn lr_to_target(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::lr_to_target(state)
    }
}

pub fn is_looking_at_target(state: *mut lua_State) -> bool {
    unsafe {
        impl_::is_looking_at_target(state)
    }
}

pub fn distance_to_target(state: *mut lua_State) -> phx::Vec2 {
    unsafe {
        impl_::distance_to_target(state).into()
    }
}

pub fn distance_x_to_target(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::distance_x_to_target(state)
    }
}

pub fn distance_y_to_target(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::distance_y_to_target(state)
    }
}

pub fn is_target_on_same_floor(state: *mut lua_State) -> bool {
    unsafe {
        impl_::is_target_on_same_floor(state)
    }
}

pub fn check_any_danger_target(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_any_danger_target(state)
    }
}

pub fn check_parent_over_ground(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_parent_over_ground(state)
    }
}

pub fn check_parent_same_floor(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_parent_same_floor(state)
    }
}

pub fn parent_pos_y(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::parent_pos_y(state)
    }
}

pub fn parent_speed_y(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::parent_speed_y(state)
    }
}

pub fn floor_edge_distance_lr(state: *mut lua_State, lr: f32) -> f32 {
    unsafe {
        impl_::floor_edge_distance_lr(state, lr)
    }
}

pub fn floor_edge_distance_f(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::floor_edge_distance_f(state)
    }
}

pub fn floor_edge_distance_b(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::floor_edge_distance_b(state)
    }
}

pub fn floor_edge_distance_floor_lr(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::floor_edge_distance_floor_lr(state)
    }
}

pub fn floor_edge_distance_floor_lr_moved(state: *mut lua_State, move_amount: f32) -> f32 {
    unsafe {
        impl_::floor_edge_distance_floor_lr_moved(state, move_amount)
    }
}

pub fn floor_width(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::floor_width(state)
    }
}

pub fn floor_center(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::floor_center(state)
    }
}

pub fn floor_lr(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::floor_lr(state)
    }
}

pub fn target_floor_edge_distance_lr(state: *mut lua_State, lr: f32) -> f32 {
    unsafe {
        impl_::target_floor_edge_distance_lr(state, lr)
    }
}

pub fn check_over_ground(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_over_ground(state)
    }
}

pub fn check_over_ground_distance_current_lr(state: *mut lua_State, lr: f32) -> bool {
    unsafe {
        impl_::check_over_ground_distance_current_lr(state, lr)
    }
}

pub fn check_target_over_ground(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_target_over_ground(state)
    }
}

pub fn check_over_goal(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_over_goal(state)
    }
}

pub fn floor_pos(state: *mut lua_State, flag: bool) -> phx::Vec2 {
    unsafe {
        impl_::floor_pos(state, flag).into()
    }
}

pub fn floor_pos_moved(state: *mut lua_State, move_amount: f32, flag: bool) -> phx::Vec2 {
    unsafe {
        impl_::floor_pos_moved(state, move_amount, flag).into()
    }
}

pub fn floor_pos_floor_lr(state: *mut lua_State, lr: f32, flag: bool) -> phx::Vec2 {
    unsafe {
        impl_::floor_pos_floor_lr(state, lr, flag).into()
    }
}

pub fn floor_moves(state: *mut lua_State) -> phx::Vec2 {
    unsafe {
        impl_::floor_moves(state).into()
    }
}

pub fn return_pos(state: *mut lua_State, flag: bool) -> phx::Vec2 {
    unsafe {
        impl_::return_pos(state, flag).into()
    }
}

pub fn safe_return_pos(state: *mut lua_State, flag: bool) -> phx::Vec2 {
    unsafe {
        impl_::safe_return_pos(state, flag).into()
    }
}

pub fn goal_pos(state: *mut lua_State) -> phx::Vec2 {
    unsafe {
        impl_::goal_pos(state).into()
    }
}

pub fn check_away_floor(state: *mut lua_State) -> bool {
    unsafe {
        impl_::check_away_floor(state)
    }
}

pub fn is_offensive_on_strategy(state: *mut lua_State) -> bool {
    unsafe {
        impl_::is_offensive_on_strategy(state)
    }
}

pub fn is_defensive_on_strategy(state: *mut lua_State) -> bool {
    unsafe {
        impl_::is_defensive_on_strategy(state)
    }
}

pub fn press_frame(state: *mut lua_State) -> u8 {
    unsafe {
        impl_::press_frame(state)
    }
}

pub fn push_wait(state: *mut lua_State) -> u8 {
    unsafe {
        impl_::push_wait(state)
    }
}

pub fn change_action(state: *mut lua_State, command_id: app::FighterAIAct::CmdId) {
    unsafe {
        impl_::change_action(state, command_id)
    }
}

pub fn set_auto_stop(state: *mut lua_State, stop_frame: i32) {
    unsafe {
        impl_::set_auto_stop(state, stop_frame)
    }
}

pub fn update_count(state: *mut lua_State) -> i32 {
    unsafe {
        impl_::update_count(state)
    }
}

pub fn is_update_count_odd(state: *mut lua_State) -> bool {
    unsafe {
        impl_::is_update_count_odd(state)
    }
}

pub fn reset_return_count(state: *mut lua_State) {
    unsafe {
        impl_::reset_return_count(state)
    }
}

pub fn set_no_return_frame(state: *mut lua_State, frame: i32) {
    unsafe {
        impl_::set_no_return_frame(state, frame)
    }
}

pub fn get_cmd_id_from_req_id(state: *mut lua_State, request_id: app::FighterAIAct::ReqId) -> app::FighterAIAct::CmdId {
    unsafe {
        impl_::get_cmd_id_from_req_id(state, request_id)
    }
}

pub fn get_cmd_id_from_req_id_with_predict(state: *mut lua_State, request_id: app::FighterAIAct::ReqId, lr: f32, predict1: f32, predict2: f32) -> app::FighterAIAct::CmdId {
    unsafe {
        impl_::get_cmd_id_from_req_id_with_predict(state, request_id, lr, predict1, predict2)
    }
}

pub fn get_cmd_probability_from_req_id(state: *mut lua_State, request_id: app::FighterAIAct::ReqId, command_id: app::FighterAIAct::CmdId) -> f32 {
    unsafe {
        impl_::get_cmd_probability_from_req_id(state, request_id, command_id)
    }
}

pub fn enable_command(state: *mut lua_State, command_id: app::FighterAIAct::CmdId) {
    unsafe {
        impl_::enable_command(state, command_id)
    }
}

pub fn disable_command(state: *mut lua_State, command_id: app::FighterAIAct::CmdId) {
    unsafe {
        impl_::disable_command(state, command_id)
    }
}

pub fn disable_command_ground_all(state: *mut lua_State) {
    unsafe {
        impl_::disable_command_ground_all(state)
    }
}

pub fn disable_command_air_all(state: *mut lua_State) {
    unsafe {
        impl_::disable_command_air_all(state)
    }
}

pub fn disable_command_attack_button_all(state: *mut lua_State) {
    unsafe {
        impl_::disable_command_attack_button_all(state)
    }
}

pub fn disable_command_special_button_all(state: *mut lua_State) {
    unsafe {
        impl_::disable_command_special_button_all(state)
    }
}

pub fn reset_cmd_id_probability_add_2nd(state: *mut lua_State) {
    unsafe {
        impl_::reset_cmd_id_probability_add_2nd(state)
    }
}

pub fn set_cmd_id_probability_add_2nd(state: *mut lua_State, command_id: app::FighterAIAct::CmdId, add_2nd: f32) {
    unsafe {
        impl_::set_cmd_id_probability_add_2nd(state, command_id, add_2nd)
    }
}

pub fn reset_cmd_id_probability_mul_2nd(state: *mut lua_State) {
    unsafe {
        impl_::reset_cmd_id_probability_mul_2nd(state)
    }
}

pub fn set_cmd_id_probability_mul_2nd(state: *mut lua_State, command_id: app::FighterAIAct::CmdId, mul_2nd: f32) {
    unsafe {
        impl_::set_cmd_id_probability_mul_2nd(state, command_id, mul_2nd)
    }
}

pub fn set_cmd_id_probability_mul_for_specializer(state: *mut lua_State, command_id: app::FighterAIAct::CmdId, mul: f32) {
    unsafe {
        impl_::set_cmd_id_probability_mul_for_specializer(state, command_id, mul)
    }
}

pub fn get_cmd_id_probability_mul(state: *mut lua_State, command_id: app::FighterAIAct::CmdId) -> f32 {
    unsafe {
        impl_::get_cmd_id_probability_mul(state, command_id)
    }
}

pub fn predict_landing_frame(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::predict_landing_frame(state)
    }
}

pub fn predict_target_landing_frame(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::predict_target_landing_frame(state)
    }
}

pub fn predict_hit_in_target_attack(state: *mut lua_State, command_id: app::FighterAIAct::CmdId, predict1: f32, predict2: f32) -> bool {
    unsafe {
        impl_::predict_hit_in_target_attack(state, command_id, predict1, predict2)
    }
}

pub fn predict_hit_in_target_attack_from_motion(state: *mut lua_State, motion: phx::Hash40, predict1: f32, predict2: f32) -> bool {
    unsafe {
        impl_::predict_hit_in_target_attack_from_motion(state, motion, predict1, predict2)
    }
}

pub fn predict_target_hit_in_attack(state: *mut lua_State, command_id: app::FighterAIAct::CmdId, start_frame: f32, mul_start_frame: f32, predict1: f32, predict2: f32) -> bool {
    unsafe {
        impl_::predict_target_hit_in_attack(state, command_id, start_frame, mul_start_frame, predict1, predict2)
    }
}

pub fn check_line_segment_vs_target_attack(state: *mut lua_State, motion: phx::Hash40, line_a: &phx::Vector2f, line_b: &phx::Vector2f, out: &mut phx::Vector4f) -> bool {
    unsafe {
        impl_::check_line_segment_vs_target_attack(state, motion, line_a, line_b, out)
    }
}

pub fn attack_start_frame(state: *mut lua_State, command_id: app::FighterAIAct::CmdId) -> i32 {
    unsafe {
        impl_::attack_start_frame(state, command_id)
    }
}

pub fn target_attack_start_frame(state: *mut lua_State, command_id: app::FighterAIAct::CmdId) -> i32 {
    unsafe {
        impl_::target_attack_start_frame(state, command_id)
    }
}

pub fn attack_end_frame(state: *mut lua_State, command_id: app::FighterAIAct::CmdId) -> i32 {
    unsafe {
        impl_::attack_end_frame(state, command_id)
    }
}

pub fn attack_cancel_frame(state: *mut lua_State, command_id: app::FighterAIAct::CmdId) -> i32 {
    unsafe {
        impl_::attack_cancel_frame(state, command_id)
    }
}

pub fn attack_data_x0(state: *mut lua_State, motion: phx::Hash40) -> f32 {
    unsafe {
        impl_::attack_data_x0(state, motion)
    }
}

pub fn attack_data_x1(state: *mut lua_State, motion: phx::Hash40) -> f32 {
    unsafe {
        impl_::attack_data_x1(state, motion)
    }
}

pub fn attack_data_y0(state: *mut lua_State, motion: phx::Hash40) -> f32 {
    unsafe {
        impl_::attack_data_y0(state, motion)
    }
}

pub fn attack_data_y1(state: *mut lua_State, motion: phx::Hash40) -> f32 {
    unsafe {
        impl_::attack_data_y1(state, motion)
    }
}

pub fn attack_info_needs_turn(state: *mut lua_State, command_id: app::FighterAIAct::CmdId) -> bool {
    unsafe {
        impl_::attack_info_needs_turn(state, command_id)
    }
}

pub fn attack_info_reaction(state: *mut lua_State, command_id: app::FighterAIAct::CmdId) -> f32 {
    unsafe {
        impl_::attack_info_reaction(state, command_id)
    }
}

pub fn attack_info_no_shield(state: *mut lua_State, command_id: app::FighterAIAct::CmdId) -> bool {
    unsafe {
        impl_::attack_info_no_shield(state, command_id)
    }
}

pub fn attack_info_meteor(state: *mut lua_State, command_id: app::FighterAIAct::CmdId) -> bool {
    unsafe {
        impl_::attack_info_meteor(state, command_id)
    }
}

pub fn attack_info_reflectable(state: *mut lua_State, command_id: app::FighterAIAct::CmdId) -> bool {
    unsafe {
        impl_::attack_info_reflectable(state, command_id)
    }
}

pub fn attack_is_as_weapon(state: *mut lua_State, command_id: app::FighterAIAct::CmdId) -> bool {
    unsafe {
        impl_::attack_is_as_weapon(state, command_id)
    }
}

pub fn attack_info_distance(state: *mut lua_State, command_id: app::FighterAIAct::CmdId) -> f32 {
    unsafe {
        impl_::attack_info_distance(state, command_id)
    }
}

pub fn motion_to_cmd_id(state: *mut lua_State, motion: phx::Hash40) -> app::FighterAIAct::CmdId {
    unsafe {
        impl_::motion_to_cmd_id(state, motion)
    }
}

pub fn target_attack_info_needs_turn(state: *mut lua_State, command_id: app::FighterAIAct::CmdId) -> bool {
    unsafe {
        impl_::target_attack_info_needs_turn(state, command_id)
    }
}

pub fn target_attack_info_no_shield(state: *mut lua_State, command_id: app::FighterAIAct::CmdId) -> bool {
    unsafe {
        impl_::target_attack_info_no_shield(state, command_id)
    }
}

pub fn target_motion_to_cmd_id(state: *mut lua_State, motion: phx::Hash40) -> app::FighterAIAct::CmdId {
    unsafe {
        impl_::target_motion_to_cmd_id(state, motion)
    }
}

pub fn current_attack_combo_next_motion(state: *mut lua_State) -> phx::Hash40 {
    unsafe {
        impl_::current_attack_combo_next_motion(state)
    }
}

pub fn target_current_attack_combo_next_motion(state: *mut lua_State) -> phx::Hash40 {
    unsafe {
        impl_::target_current_attack_combo_next_motion(state)
    }
}

pub fn target_attack_start_frame_from_motion(state: *mut lua_State, motion: phx::Hash40) -> i32 {
    unsafe {
        impl_::target_attack_start_frame_from_motion(state, motion)
    }
}

