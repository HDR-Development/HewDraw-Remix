use crate::*;

use super::class::*;

extern "C" {
    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase32main_func__mode_global_variablesEv"]
    pub(super) fn main_func__mode_global_variables(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase33main_func__mode_private_variablesEv"]
    pub(super) fn main_func__mode_private_variables(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase28main_func__action_air_attackEv"]
    pub(super) fn main_func__action_air_attack(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase24main_func__action_appealEv"]
    pub(super) fn main_func__action_appeal(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase28main_func__action_attack_123Ev"]
    pub(super) fn main_func__action_attack_123(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase41main_func__action_avoid_donkey_special_lwEv"]
    pub(super) fn main_func__action_avoid_donkey_special_lw(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase43main_func__action_avoid_ganon_special_air_sEv"]
    pub(super) fn main_func__action_avoid_ganon_special_air_s(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase39main_func__action_avoid_koopa_special_sEv"]
    pub(super) fn main_func__action_avoid_koopa_special_s(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase25main_func__action_buildupEv"]
    pub(super) fn main_func__action_buildup(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase23main_func__action_cliffEv"]
    pub(super) fn main_func__action_cliff(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase23main_func__action_comboEv"]
    pub(super) fn main_func__action_combo(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase25main_func__action_damagedEv"]
    pub(super) fn main_func__action_damaged(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase29main_func__action_dash_attackEv"]
    pub(super) fn main_func__action_dash_attack(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase24main_func__action_dash_bEv"]
    pub(super) fn main_func__action_dash_b(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase31main_func__action_dash_b_dash_fEv"]
    pub(super) fn main_func__action_dash_b_dash_f(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase31main_func__action_dash_b_jump_sEv"]
    pub(super) fn main_func__action_dash_b_jump_s(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase31main_func__action_dash_f_dash_bEv"]
    pub(super) fn main_func__action_dash_f_dash_b(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase31main_func__action_dash_f_jump_bEv"]
    pub(super) fn main_func__action_dash_f_jump_b(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase29main_func__action_dash_f_waitEv"]
    pub(super) fn main_func__action_dash_f_wait(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase28main_func__action_dash_guardEv"]
    pub(super) fn main_func__action_dash_guard(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase29main_func__action_dash_roll_bEv"]
    pub(super) fn main_func__action_dash_roll_b(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase29main_func__action_dash_roll_fEv"]
    pub(super) fn main_func__action_dash_roll_f(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37main_func__action_deadarea_air_attackEv"]
    pub(super) fn main_func__action_deadarea_air_attack(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38main_func__action_deadarea_dash_attackEv"]
    pub(super) fn main_func__action_deadarea_dash_attack(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase22main_func__action_downEv"]
    pub(super) fn main_func__action_down(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase25main_func__action_dragoonEv"]
    pub(super) fn main_func__action_dragoon(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase23main_func__action_drillEv"]
    pub(super) fn main_func__action_drill(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase25main_func__action_entry_1Ev"]
    pub(super) fn main_func__action_entry_1(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase24main_func__action_escapeEv"]
    pub(super) fn main_func__action_escape(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase28main_func__action_escape_airEv"]
    pub(super) fn main_func__action_escape_air(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase33main_func__action_escape_air_moveEv"]
    pub(super) fn main_func__action_escape_air_move(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase28main_func__action_escape_farEv"]
    pub(super) fn main_func__action_escape_far(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase29main_func__action_escape_nearEv"]
    pub(super) fn main_func__action_escape_near(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase22main_func__action_fallEv"]
    pub(super) fn main_func__action_fall(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase32main_func__action_final_end_jumpEv"]
    pub(super) fn main_func__action_final_end_jump(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase23main_func__action_guardEv"]
    pub(super) fn main_func__action_guard(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase33main_func__action_high_speed_dashEv"]
    pub(super) fn main_func__action_high_speed_dash(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase29main_func__action_jump_attackEv"]
    pub(super) fn main_func__action_jump_attack(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase31main_func__action_jump_attack_fEv"]
    pub(super) fn main_func__action_jump_attack_f(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase31main_func__action_jump_s_attackEv"]
    pub(super) fn main_func__action_jump_s_attack(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase26main_func__action_jump_s_bEv"]
    pub(super) fn main_func__action_jump_s_b(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase33main_func__action_jump_s_n_attackEv"]
    pub(super) fn main_func__action_jump_s_n_attack(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase27main_func__action_machstampEv"]
    pub(super) fn main_func__action_machstamp(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase24main_func__action_meteorEv"]
    pub(super) fn main_func__action_meteor(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase28main_func__action_pass_floorEv"]
    pub(super) fn main_func__action_pass_floor(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase24main_func__action_pursueEv"]
    pub(super) fn main_func__action_pursue(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase25main_func__action_rebirthEv"]
    pub(super) fn main_func__action_rebirth(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase24main_func__action_returnEv"]
    pub(super) fn main_func__action_return(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase24main_func__action_roll_bEv"]
    pub(super) fn main_func__action_roll_b(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase24main_func__action_roll_fEv"]
    pub(super) fn main_func__action_roll_f(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase21main_func__action_runEv"]
    pub(super) fn main_func__action_run(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase28main_func__action_run_attackEv"]
    pub(super) fn main_func__action_run_attack(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase26main_func__action_stroll_lEv"]
    pub(super) fn main_func__action_stroll_l(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase26main_func__action_stroll_sEv"]
    pub(super) fn main_func__action_stroll_s(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase30main_func__action_stroll_squatEv"]
    pub(super) fn main_func__action_stroll_squat(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase27main_func__action_turn_turnEv"]
    pub(super) fn main_func__action_turn_turn(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase24main_func__action_wait_lEv"]
    pub(super) fn main_func__action_wait_l(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase24main_func__action_wait_sEv"]
    pub(super) fn main_func__action_wait_s(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase28main_func__action_wait_squatEv"]
    pub(super) fn main_func__action_wait_squat(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase22main_func__action_walkEv"]
    pub(super) fn main_func__action_walk(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase18call_event_on_deadEv"]
    pub(super) fn call_event_on_dead(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase18change_mode_actionEN3lib8L2CValueE"]
    pub(super) fn change_mode_action(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase27call_event_on_attack_shieldEN3lib8L2CValueE"]
    pub(super) fn call_event_on_attack_shield(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase23notify_on_attack_shieldEN3lib8L2CValueE"]
    pub(super) fn notify_on_attack_shield(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase41global__tactics___notify_on_attack_shieldEv"]
    pub(super) fn global__tactics___notify_on_attack_shield(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase24call_event_on_attack_hitEv"]
    pub(super) fn call_event_on_attack_hit(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase20notify_on_attack_hitEv"]
    pub(super) fn notify_on_attack_hit(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38global__tactics___notify_on_attack_hitEv"]
    pub(super) fn global__tactics___notify_on_attack_hit(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase17is_normal_cp_typeEN3lib8L2CValueE"]
    pub(super) fn is_normal_cp_type(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase20call_event_on_shieldEN3lib8L2CValueES2_S2_S2_"]
    pub(super) fn call_event_on_shield(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase23local_func__mode_line_4EN3lib8L2CValueES2_S2_S2_"]
    pub(super) fn local_func__mode_line_4(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase16notify_on_shieldEN3lib8L2CValueE"]
    pub(super) fn notify_on_shield(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35global__learning___notify_on_shieldEv"]
    pub(super) fn global__learning___notify_on_shield(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35global__tactics___notify_on_captureEv"]
    pub(super) fn global__tactics___notify_on_capture(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36global__learning___notify_on_captureEv"]
    pub(super) fn global__learning___notify_on_capture(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase20call_event_on_damageEN3lib8L2CValueES2_S2_S2_S2_S2_S2_S2_S2_S2_S2_S2_"]
    pub(super) fn call_event_on_damage(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack, arg6: lib::L2CValueHack, arg7: lib::L2CValueHack, arg8: lib::L2CValueHack, arg9: lib::L2CValueHack, arg10: lib::L2CValueHack, arg11: lib::L2CValueHack, arg12: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase23local_func__mode_line_3EN3lib8L2CValueES2_S2_S2_S2_S2_S2_S2_S2_S2_S2_S2_"]
    pub(super) fn local_func__mode_line_3(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack, arg6: lib::L2CValueHack, arg7: lib::L2CValueHack, arg8: lib::L2CValueHack, arg9: lib::L2CValueHack, arg10: lib::L2CValueHack, arg11: lib::L2CValueHack, arg12: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase16notify_on_damageEN3lib8L2CValueE"]
    pub(super) fn notify_on_damage(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase33global__damage___notify_on_damageEN3lib8L2CValueE"]
    pub(super) fn global__damage___notify_on_damage(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase34global__tactics___notify_on_damageEN3lib8L2CValueE"]
    pub(super) fn global__tactics___notify_on_damage(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35global__learning___notify_on_damageEN3lib8L2CValueE"]
    pub(super) fn global__learning___notify_on_damage(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36local_func__mode_global_variables_17Ev"]
    pub(super) fn local_func__mode_global_variables_17(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase23local_func__mode_line_1EN3lib8L2CValueES2_"]
    pub(super) fn local_func__mode_line_1(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase23local_func__mode_line_2EN3lib8L2CValueES2_"]
    pub(super) fn local_func__mode_line_2(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37set_action_probability_mul_as_forwardEv"]
    pub(super) fn set_action_probability_mul_as_forward(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37set_action_probability_mul_as_chargerEv"]
    pub(super) fn set_action_probability_mul_as_charger(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36set_action_probability_mul_as_walkerEv"]
    pub(super) fn set_action_probability_mul_as_walker(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38set_action_probability_mul_3rd_foreachEN3lib8L2CValueES2_"]
    pub(super) fn set_action_probability_mul_3rd_foreach(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase18is_run_mode_actionEN3lib8L2CValueE"]
    pub(super) fn is_run_mode_action(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36set_action_probability_mul_as_dasherEv"]
    pub(super) fn set_action_probability_mul_as_dasher(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36set_action_probability_mul_as_jumperEv"]
    pub(super) fn set_action_probability_mul_as_jumper(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38set_action_probability_mul_as_grounderEv"]
    pub(super) fn set_action_probability_mul_as_grounder(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase19is_jump_mode_actionEN3lib8L2CValueE"]
    pub(super) fn is_jump_mode_action(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase25is_small_jump_mode_actionEN3lib8L2CValueE"]
    pub(super) fn is_small_jump_mode_action(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase49call_function_is_cancelable_mode_action_by_attackEv"]
    pub(super) fn call_function_is_cancelable_mode_action_by_attack(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase30call_function_update_coroutineEv"]
    pub(super) fn call_function_update_coroutine(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase13update_actionEv"]
    pub(super) fn update_action(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35global__difficulty___is_action_waitEv"]
    pub(super) fn global__difficulty___is_action_wait(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase13select_actionEv"]
    pub(super) fn select_action(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase17deadzone_distanceEv"]
    pub(super) fn deadzone_distance(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38check_line_segment_is_danger_with_infoEN3lib8L2CValueES2_S2_S2_"]
    pub(super) fn check_line_segment_is_danger_with_info(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase28check_line_segment_is_dangerEN3lib8L2CValueES2_S2_S2_"]
    pub(super) fn check_line_segment_is_danger(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase25local_func__mode_action_1Ev"]
    pub(super) fn local_func__mode_action_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35local_func__mode_predict_utility_13Ev"]
    pub(super) fn local_func__mode_predict_utility_13(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase20analyst_is_attackingEN3lib8L2CValueE"]
    pub(super) fn analyst_is_attacking(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase32analyst_is_attacking_only_groundEN3lib8L2CValueE"]
    pub(super) fn analyst_is_attacking_only_ground(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35local_func__mode_predict_utility_15Ev"]
    pub(super) fn local_func__mode_predict_utility_15(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35local_func__mode_predict_utility_14Ev"]
    pub(super) fn local_func__mode_predict_utility_14(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35local_func__mode_predict_utility_12Ev"]
    pub(super) fn local_func__mode_predict_utility_12(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38set_action_probability_mul_1st_foreachEN3lib8L2CValueES2_"]
    pub(super) fn set_action_probability_mul_1st_foreach(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase23specializer_can_buildupEv"]
    pub(super) fn specializer_can_buildup(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase25local_func__mode_action_2Ev"]
    pub(super) fn local_func__mode_action_2(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase15escape_distanceEv"]
    pub(super) fn escape_distance(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase25select_mode_action_groundEv"]
    pub(super) fn select_mode_action_ground(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase32reset_action_probability_mul_1stEv"]
    pub(super) fn reset_action_probability_mul_1st(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase32reset_action_probability_mul_2ndEv"]
    pub(super) fn reset_action_probability_mul_2nd(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase22select_mode_action_airEv"]
    pub(super) fn select_mode_action_air(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase4diveEv"]
    pub(super) fn dive(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase23select_action_from_listEN3lib8L2CValueE"]
    pub(super) fn select_action_from_list(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase27is_dash_forward_mode_actionEN3lib8L2CValueE"]
    pub(super) fn is_dash_forward_mode_action(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase19is_roll_mode_actionEN3lib8L2CValueE"]
    pub(super) fn is_roll_mode_action(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase20is_feint_mode_actionEN3lib8L2CValueE"]
    pub(super) fn is_feint_mode_action(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase23update_global_variablesEv"]
    pub(super) fn update_global_variables(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase16update_interruptEv"]
    pub(super) fn update_interrupt(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35global__difficulty___is_attack_waitEv"]
    pub(super) fn global__difficulty___is_attack_wait(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase26select_weapon_guard_actionEv"]
    pub(super) fn select_weapon_guard_action(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase46specializer_interrupt_action_on_target_rebirthEN3lib8L2CValueES2_S2_"]
    pub(super) fn specializer_interrupt_action_on_target_rebirth(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase42change_mode_action_no_restart_by_interruptEN3lib8L2CValueE"]
    pub(super) fn change_mode_action_no_restart_by_interrupt(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35local_func__mode_action_interrupt_1Ev"]
    pub(super) fn local_func__mode_action_interrupt_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase23restrict_always_commandEv"]
    pub(super) fn restrict_always_command(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase29select_aerial_jump_action_airEv"]
    pub(super) fn select_aerial_jump_action_air(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase32select_interrupt_action_ground_cEN3lib8L2CValueES2_"]
    pub(super) fn select_interrupt_action_ground_c(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase15analyst_is_dashEN3lib8L2CValueE"]
    pub(super) fn analyst_is_dash(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35select_interrupt_action_ground_dashEN3lib8L2CValueES2_"]
    pub(super) fn select_interrupt_action_ground_dash(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase30select_interrupt_action_groundEN3lib8L2CValueES2_"]
    pub(super) fn select_interrupt_action_ground(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase27select_interrupt_action_airEN3lib8L2CValueES2_S2_"]
    pub(super) fn select_interrupt_action_air(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase22select_dive_action_airEv"]
    pub(super) fn select_dive_action_air(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35local_func__mode_action_interrupt_8Ev"]
    pub(super) fn local_func__mode_action_interrupt_8(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36select_interrupt_action_air_internalEN3lib8L2CValueES2_S2_S2_S2_"]
    pub(super) fn select_interrupt_action_air_internal(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase32predict_able_to_return_from_hereEN3lib8L2CValueE"]
    pub(super) fn predict_able_to_return_from_here(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase25restrict_buildmax_commandEv"]
    pub(super) fn restrict_buildmax_command(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase33restrict_air_command_by_no_groundEv"]
    pub(super) fn restrict_air_command_by_no_ground(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase24select_attack_action_airEN3lib8L2CValueE"]
    pub(super) fn select_attack_action_air(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase45predict_guard_opponent_attack_probability_airEN3lib8L2CValueE"]
    pub(super) fn predict_guard_opponent_attack_probability_air(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase23select_guard_action_airEv"]
    pub(super) fn select_guard_action_air(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35local_func__mode_action_interrupt_9Ev"]
    pub(super) fn local_func__mode_action_interrupt_9(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase55global__learning___update_guard_rate_on_nothing_happendEv"]
    pub(super) fn global__learning___update_guard_rate_on_nothing_happend(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase45global__learning___start_observe_guard_statusEv"]
    pub(super) fn global__learning___start_observe_guard_status(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase48predict_guard_opponent_attack_probability_groundEN3lib8L2CValueE"]
    pub(super) fn predict_guard_opponent_attack_probability_ground(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36local_func__mode_action_interrupt_10Ev"]
    pub(super) fn local_func__mode_action_interrupt_10(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase21floor_pos_with_adjustEv"]
    pub(super) fn floor_pos_with_adjust(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase28specializer_aerial_jump_highEv"]
    pub(super) fn specializer_aerial_jump_high(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35local_func__mode_predict_utility_11Ev"]
    pub(super) fn local_func__mode_predict_utility_11(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35local_func__mode_action_interrupt_4Ev"]
    pub(super) fn local_func__mode_action_interrupt_4(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37select_interrupt_action_ground_commonEN3lib8L2CValueES2_S2_S2_"]
    pub(super) fn select_interrupt_action_ground_common(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase25analyst_is_free_to_attackEN3lib8L2CValueE"]
    pub(super) fn analyst_is_free_to_attack(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase27select_attack_action_groundEN3lib8L2CValueE"]
    pub(super) fn select_attack_action_ground(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase26change_ground_guard_actionEv"]
    pub(super) fn change_ground_guard_action(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase31analyst_is_or_will_be_attackingEN3lib8L2CValueE"]
    pub(super) fn analyst_is_or_will_be_attacking(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase27analyst_is_in_rest_to_comboEN3lib8L2CValueE"]
    pub(super) fn analyst_is_in_rest_to_combo(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase33global__difficulty___start_actionEv"]
    pub(super) fn global__difficulty___start_action(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36local_func__mode_action_interrupt_11Ev"]
    pub(super) fn local_func__mode_action_interrupt_11(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36local_func__mode_action_interrupt_12Ev"]
    pub(super) fn local_func__mode_action_interrupt_12(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase25deadzone_nearest_distanceEv"]
    pub(super) fn deadzone_nearest_distance(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase33select_weapon_guard_action_groundEv"]
    pub(super) fn select_weapon_guard_action_ground(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase30select_weapon_guard_action_airEv"]
    pub(super) fn select_weapon_guard_action_air(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35local_func__mode_action_interrupt_2Ev"]
    pub(super) fn local_func__mode_action_interrupt_2(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase25analyst_is_kind_of_attackEN3lib8L2CValueE"]
    pub(super) fn analyst_is_kind_of_attack(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase24update_global_difficultyEv"]
    pub(super) fn update_global_difficulty(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase20update_global_damageEv"]
    pub(super) fn update_global_damage(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase23update_global_situationEv"]
    pub(super) fn update_global_situation(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase21update_global_tacticsEv"]
    pub(super) fn update_global_tactics(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase22update_global_learningEv"]
    pub(super) fn update_global_learning(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase26update_global_predict_selfEv"]
    pub(super) fn update_global_predict_self(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase28update_global_predict_targetEv"]
    pub(super) fn update_global_predict_target(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase28update_global_predict_weaponEv"]
    pub(super) fn update_global_predict_weapon(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase20update_global_koopagEv"]
    pub(super) fn update_global_koopag(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase29predict_target_any_attack_hitEN3lib8L2CValueE"]
    pub(super) fn predict_target_any_attack_hit(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase28predict_target_any_catch_hitEN3lib8L2CValueE"]
    pub(super) fn predict_target_any_catch_hit(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase34local_func__mode_predict_utility_6EN3lib8L2CValueES2_"]
    pub(super) fn local_func__mode_predict_utility_6(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase32predict_target_catch_hit_on_listEN3lib8L2CValueES2_"]
    pub(super) fn predict_target_catch_hit_on_list(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase34local_func__mode_predict_utility_7EN3lib8L2CValueE"]
    pub(super) fn local_func__mode_predict_utility_7(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase34local_func__mode_predict_utility_8EN3lib8L2CValueES2_"]
    pub(super) fn local_func__mode_predict_utility_8(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase34local_func__mode_predict_utility_9EN3lib8L2CValueE"]
    pub(super) fn local_func__mode_predict_utility_9(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35local_func__mode_predict_utility_10Ev"]
    pub(super) fn local_func__mode_predict_utility_10(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase25predict_target_attack_hitEN3lib8L2CValueES2_"]
    pub(super) fn predict_target_attack_hit(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase34local_func__mode_predict_utility_1EN3lib8L2CValueES2_"]
    pub(super) fn local_func__mode_predict_utility_1(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase33predict_target_attack_hit_on_listEN3lib8L2CValueES2_"]
    pub(super) fn predict_target_attack_hit_on_list(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase34local_func__mode_predict_utility_2EN3lib8L2CValueE"]
    pub(super) fn local_func__mode_predict_utility_2(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase34local_func__mode_predict_utility_3EN3lib8L2CValueES2_"]
    pub(super) fn local_func__mode_predict_utility_3(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase34local_func__mode_predict_utility_4EN3lib8L2CValueE"]
    pub(super) fn local_func__mode_predict_utility_4(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase34local_func__mode_predict_utility_5EN3lib8L2CValueE"]
    pub(super) fn local_func__mode_predict_utility_5(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36local_func__mode_global_variables_18Ev"]
    pub(super) fn local_func__mode_global_variables_18(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase25call_function_change_modeEv"]
    pub(super) fn call_function_change_mode(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase19call_function_resetEN3lib8L2CValueE"]
    pub(super) fn call_function_reset(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase21init_global_variablesEv"]
    pub(super) fn init_global_variables(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase22init_private_variablesEv"]
    pub(super) fn init_private_variables(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36local_func__mode_private_variables_3Ev"]
    pub(super) fn local_func__mode_private_variables_3(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36local_func__mode_private_variables_4Ev"]
    pub(super) fn local_func__mode_private_variables_4(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36local_func__mode_private_variables_5Ev"]
    pub(super) fn local_func__mode_private_variables_5(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36local_func__mode_private_variables_6Ev"]
    pub(super) fn local_func__mode_private_variables_6(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36local_func__mode_private_variables_7Ev"]
    pub(super) fn local_func__mode_private_variables_7(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36local_func__mode_private_variables_8Ev"]
    pub(super) fn local_func__mode_private_variables_8(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36local_func__mode_private_variables_9Ev"]
    pub(super) fn local_func__mode_private_variables_9(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_10Ev"]
    pub(super) fn local_func__mode_private_variables_10(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_11Ev"]
    pub(super) fn local_func__mode_private_variables_11(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_12Ev"]
    pub(super) fn local_func__mode_private_variables_12(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_13Ev"]
    pub(super) fn local_func__mode_private_variables_13(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_14Ev"]
    pub(super) fn local_func__mode_private_variables_14(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_15Ev"]
    pub(super) fn local_func__mode_private_variables_15(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_16Ev"]
    pub(super) fn local_func__mode_private_variables_16(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_17Ev"]
    pub(super) fn local_func__mode_private_variables_17(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_18Ev"]
    pub(super) fn local_func__mode_private_variables_18(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_19Ev"]
    pub(super) fn local_func__mode_private_variables_19(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_20Ev"]
    pub(super) fn local_func__mode_private_variables_20(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_21Ev"]
    pub(super) fn local_func__mode_private_variables_21(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_22Ev"]
    pub(super) fn local_func__mode_private_variables_22(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_23Ev"]
    pub(super) fn local_func__mode_private_variables_23(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_24Ev"]
    pub(super) fn local_func__mode_private_variables_24(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_25Ev"]
    pub(super) fn local_func__mode_private_variables_25(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_26Ev"]
    pub(super) fn local_func__mode_private_variables_26(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_27Ev"]
    pub(super) fn local_func__mode_private_variables_27(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_28Ev"]
    pub(super) fn local_func__mode_private_variables_28(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_29Ev"]
    pub(super) fn local_func__mode_private_variables_29(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_30Ev"]
    pub(super) fn local_func__mode_private_variables_30(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_31Ev"]
    pub(super) fn local_func__mode_private_variables_31(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_32Ev"]
    pub(super) fn local_func__mode_private_variables_32(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_33Ev"]
    pub(super) fn local_func__mode_private_variables_33(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_34Ev"]
    pub(super) fn local_func__mode_private_variables_34(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_35Ev"]
    pub(super) fn local_func__mode_private_variables_35(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_36Ev"]
    pub(super) fn local_func__mode_private_variables_36(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_37Ev"]
    pub(super) fn local_func__mode_private_variables_37(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_38Ev"]
    pub(super) fn local_func__mode_private_variables_38(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_39Ev"]
    pub(super) fn local_func__mode_private_variables_39(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_40Ev"]
    pub(super) fn local_func__mode_private_variables_40(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_41Ev"]
    pub(super) fn local_func__mode_private_variables_41(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_42Ev"]
    pub(super) fn local_func__mode_private_variables_42(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_43Ev"]
    pub(super) fn local_func__mode_private_variables_43(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_44Ev"]
    pub(super) fn local_func__mode_private_variables_44(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_45Ev"]
    pub(super) fn local_func__mode_private_variables_45(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_46Ev"]
    pub(super) fn local_func__mode_private_variables_46(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_47Ev"]
    pub(super) fn local_func__mode_private_variables_47(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_48Ev"]
    pub(super) fn local_func__mode_private_variables_48(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_49Ev"]
    pub(super) fn local_func__mode_private_variables_49(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_50Ev"]
    pub(super) fn local_func__mode_private_variables_50(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_51Ev"]
    pub(super) fn local_func__mode_private_variables_51(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_52Ev"]
    pub(super) fn local_func__mode_private_variables_52(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_53Ev"]
    pub(super) fn local_func__mode_private_variables_53(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_54Ev"]
    pub(super) fn local_func__mode_private_variables_54(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_55Ev"]
    pub(super) fn local_func__mode_private_variables_55(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_56Ev"]
    pub(super) fn local_func__mode_private_variables_56(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_57Ev"]
    pub(super) fn local_func__mode_private_variables_57(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_58Ev"]
    pub(super) fn local_func__mode_private_variables_58(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_59Ev"]
    pub(super) fn local_func__mode_private_variables_59(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_60Ev"]
    pub(super) fn local_func__mode_private_variables_60(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_61Ev"]
    pub(super) fn local_func__mode_private_variables_61(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_62Ev"]
    pub(super) fn local_func__mode_private_variables_62(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_63Ev"]
    pub(super) fn local_func__mode_private_variables_63(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_64Ev"]
    pub(super) fn local_func__mode_private_variables_64(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_65Ev"]
    pub(super) fn local_func__mode_private_variables_65(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_66Ev"]
    pub(super) fn local_func__mode_private_variables_66(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_67Ev"]
    pub(super) fn local_func__mode_private_variables_67(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_68Ev"]
    pub(super) fn local_func__mode_private_variables_68(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_69Ev"]
    pub(super) fn local_func__mode_private_variables_69(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_70Ev"]
    pub(super) fn local_func__mode_private_variables_70(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_71Ev"]
    pub(super) fn local_func__mode_private_variables_71(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_72Ev"]
    pub(super) fn local_func__mode_private_variables_72(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_73Ev"]
    pub(super) fn local_func__mode_private_variables_73(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_74Ev"]
    pub(super) fn local_func__mode_private_variables_74(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_75Ev"]
    pub(super) fn local_func__mode_private_variables_75(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_76Ev"]
    pub(super) fn local_func__mode_private_variables_76(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_77Ev"]
    pub(super) fn local_func__mode_private_variables_77(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_78Ev"]
    pub(super) fn local_func__mode_private_variables_78(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_79Ev"]
    pub(super) fn local_func__mode_private_variables_79(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_80Ev"]
    pub(super) fn local_func__mode_private_variables_80(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_81Ev"]
    pub(super) fn local_func__mode_private_variables_81(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_82Ev"]
    pub(super) fn local_func__mode_private_variables_82(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_83Ev"]
    pub(super) fn local_func__mode_private_variables_83(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_84Ev"]
    pub(super) fn local_func__mode_private_variables_84(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_85Ev"]
    pub(super) fn local_func__mode_private_variables_85(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_86Ev"]
    pub(super) fn local_func__mode_private_variables_86(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_87Ev"]
    pub(super) fn local_func__mode_private_variables_87(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_88Ev"]
    pub(super) fn local_func__mode_private_variables_88(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_89Ev"]
    pub(super) fn local_func__mode_private_variables_89(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_90Ev"]
    pub(super) fn local_func__mode_private_variables_90(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_91Ev"]
    pub(super) fn local_func__mode_private_variables_91(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_92Ev"]
    pub(super) fn local_func__mode_private_variables_92(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_93Ev"]
    pub(super) fn local_func__mode_private_variables_93(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_94Ev"]
    pub(super) fn local_func__mode_private_variables_94(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_95Ev"]
    pub(super) fn local_func__mode_private_variables_95(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_96Ev"]
    pub(super) fn local_func__mode_private_variables_96(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_97Ev"]
    pub(super) fn local_func__mode_private_variables_97(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_98Ev"]
    pub(super) fn local_func__mode_private_variables_98(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37local_func__mode_private_variables_99Ev"]
    pub(super) fn local_func__mode_private_variables_99(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_100Ev"]
    pub(super) fn local_func__mode_private_variables_100(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_101Ev"]
    pub(super) fn local_func__mode_private_variables_101(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_102Ev"]
    pub(super) fn local_func__mode_private_variables_102(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_103Ev"]
    pub(super) fn local_func__mode_private_variables_103(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_104Ev"]
    pub(super) fn local_func__mode_private_variables_104(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_105Ev"]
    pub(super) fn local_func__mode_private_variables_105(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_106Ev"]
    pub(super) fn local_func__mode_private_variables_106(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_107Ev"]
    pub(super) fn local_func__mode_private_variables_107(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_108Ev"]
    pub(super) fn local_func__mode_private_variables_108(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_109Ev"]
    pub(super) fn local_func__mode_private_variables_109(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_110Ev"]
    pub(super) fn local_func__mode_private_variables_110(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_111Ev"]
    pub(super) fn local_func__mode_private_variables_111(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_112Ev"]
    pub(super) fn local_func__mode_private_variables_112(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_113Ev"]
    pub(super) fn local_func__mode_private_variables_113(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_114Ev"]
    pub(super) fn local_func__mode_private_variables_114(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_115Ev"]
    pub(super) fn local_func__mode_private_variables_115(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_116Ev"]
    pub(super) fn local_func__mode_private_variables_116(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_117Ev"]
    pub(super) fn local_func__mode_private_variables_117(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_118Ev"]
    pub(super) fn local_func__mode_private_variables_118(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_119Ev"]
    pub(super) fn local_func__mode_private_variables_119(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_120Ev"]
    pub(super) fn local_func__mode_private_variables_120(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_121Ev"]
    pub(super) fn local_func__mode_private_variables_121(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_122Ev"]
    pub(super) fn local_func__mode_private_variables_122(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_123Ev"]
    pub(super) fn local_func__mode_private_variables_123(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_124Ev"]
    pub(super) fn local_func__mode_private_variables_124(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_125Ev"]
    pub(super) fn local_func__mode_private_variables_125(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_126Ev"]
    pub(super) fn local_func__mode_private_variables_126(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_127Ev"]
    pub(super) fn local_func__mode_private_variables_127(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_128Ev"]
    pub(super) fn local_func__mode_private_variables_128(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_129Ev"]
    pub(super) fn local_func__mode_private_variables_129(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_130Ev"]
    pub(super) fn local_func__mode_private_variables_130(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_131Ev"]
    pub(super) fn local_func__mode_private_variables_131(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_132Ev"]
    pub(super) fn local_func__mode_private_variables_132(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_133Ev"]
    pub(super) fn local_func__mode_private_variables_133(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_134Ev"]
    pub(super) fn local_func__mode_private_variables_134(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_135Ev"]
    pub(super) fn local_func__mode_private_variables_135(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_136Ev"]
    pub(super) fn local_func__mode_private_variables_136(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_137Ev"]
    pub(super) fn local_func__mode_private_variables_137(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_138Ev"]
    pub(super) fn local_func__mode_private_variables_138(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_139Ev"]
    pub(super) fn local_func__mode_private_variables_139(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_140Ev"]
    pub(super) fn local_func__mode_private_variables_140(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_141Ev"]
    pub(super) fn local_func__mode_private_variables_141(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_142Ev"]
    pub(super) fn local_func__mode_private_variables_142(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_143Ev"]
    pub(super) fn local_func__mode_private_variables_143(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_144Ev"]
    pub(super) fn local_func__mode_private_variables_144(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_145Ev"]
    pub(super) fn local_func__mode_private_variables_145(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_146Ev"]
    pub(super) fn local_func__mode_private_variables_146(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_147Ev"]
    pub(super) fn local_func__mode_private_variables_147(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_148Ev"]
    pub(super) fn local_func__mode_private_variables_148(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_149Ev"]
    pub(super) fn local_func__mode_private_variables_149(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_150Ev"]
    pub(super) fn local_func__mode_private_variables_150(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_151Ev"]
    pub(super) fn local_func__mode_private_variables_151(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_152Ev"]
    pub(super) fn local_func__mode_private_variables_152(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_153Ev"]
    pub(super) fn local_func__mode_private_variables_153(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38local_func__mode_private_variables_154Ev"]
    pub(super) fn local_func__mode_private_variables_154(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase20is_chase_mode_actionEN3lib8L2CValueE"]
    pub(super) fn is_chase_mode_action(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase5ENTRYEv"]
    pub(super) fn ENTRY(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase10init_linesEv"]
    pub(super) fn init_lines(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase25local_func__action_walk_1Ev"]
    pub(super) fn local_func__action_walk_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase19ACTION_WALK__updateEv"]
    pub(super) fn ACTION_WALK__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase17ACTION_WALK__initEv"]
    pub(super) fn ACTION_WALK__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase31local_func__action_wait_squat_1Ev"]
    pub(super) fn local_func__action_wait_squat_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase25ACTION_WAIT_SQUAT__updateEv"]
    pub(super) fn ACTION_WAIT_SQUAT__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase23ACTION_WAIT_SQUAT__initEv"]
    pub(super) fn ACTION_WAIT_SQUAT__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase27local_func__action_wait_s_1Ev"]
    pub(super) fn local_func__action_wait_s_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase21ACTION_WAIT_S__updateEv"]
    pub(super) fn ACTION_WAIT_S__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase19ACTION_WAIT_S__initEv"]
    pub(super) fn ACTION_WAIT_S__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase27local_func__action_wait_l_1Ev"]
    pub(super) fn local_func__action_wait_l_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase21ACTION_WAIT_L__updateEv"]
    pub(super) fn ACTION_WAIT_L__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase19ACTION_WAIT_L__initEv"]
    pub(super) fn ACTION_WAIT_L__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase30local_func__action_turn_turn_1Ev"]
    pub(super) fn local_func__action_turn_turn_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase24ACTION_TURN_TURN__updateEv"]
    pub(super) fn ACTION_TURN_TURN__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase33local_func__action_stroll_squat_1Ev"]
    pub(super) fn local_func__action_stroll_squat_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase29local_func__action_stroll_s_1Ev"]
    pub(super) fn local_func__action_stroll_s_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase29local_func__action_stroll_l_1Ev"]
    pub(super) fn local_func__action_stroll_l_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase31local_func__action_run_attack_1Ev"]
    pub(super) fn local_func__action_run_attack_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase25ACTION_RUN_ATTACK__updateEv"]
    pub(super) fn ACTION_RUN_ATTACK__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase23ACTION_RUN_ATTACK__initEv"]
    pub(super) fn ACTION_RUN_ATTACK__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase24local_func__action_run_1Ev"]
    pub(super) fn local_func__action_run_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase18ACTION_RUN__updateEv"]
    pub(super) fn ACTION_RUN__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase16ACTION_RUN__initEv"]
    pub(super) fn ACTION_RUN__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase27local_func__action_roll_f_1Ev"]
    pub(super) fn local_func__action_roll_f_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase21ACTION_ROLL_F__updateEv"]
    pub(super) fn ACTION_ROLL_F__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase19ACTION_ROLL_F__initEv"]
    pub(super) fn ACTION_ROLL_F__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase27local_func__action_roll_b_1Ev"]
    pub(super) fn local_func__action_roll_b_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase21ACTION_ROLL_B__updateEv"]
    pub(super) fn ACTION_ROLL_B__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase19ACTION_ROLL_B__initEv"]
    pub(super) fn ACTION_ROLL_B__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase27local_func__action_return_1Ev"]
    pub(super) fn local_func__action_return_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37ACTION_RETURN__interrupt_guard_weaponEv"]
    pub(super) fn ACTION_RETURN__interrupt_guard_weapon(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase31ACTION_RETURN__interrupt_attackEv"]
    pub(super) fn ACTION_RETURN__interrupt_attack(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase21ACTION_RETURN__updateEv"]
    pub(super) fn ACTION_RETURN__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase27local_func__action_return_2Ev"]
    pub(super) fn local_func__action_return_2(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38specializer_interrupt_action_on_returnEN3lib8L2CValueES2_"]
    pub(super) fn specializer_interrupt_action_on_return(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase27local_func__action_return_3Ev"]
    pub(super) fn local_func__action_return_3(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase27damage_motion_cancel_cmd_idEv"]
    pub(super) fn damage_motion_cancel_cmd_id(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase19ACTION_RETURN__initEv"]
    pub(super) fn ACTION_RETURN__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase28local_func__action_rebirth_1Ev"]
    pub(super) fn local_func__action_rebirth_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase22ACTION_REBIRTH__updateEv"]
    pub(super) fn ACTION_REBIRTH__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase27local_func__action_pursue_1Ev"]
    pub(super) fn local_func__action_pursue_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase21ACTION_PURSUE__updateEv"]
    pub(super) fn ACTION_PURSUE__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase19ACTION_PURSUE__initEv"]
    pub(super) fn ACTION_PURSUE__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase31local_func__action_pass_floor_1Ev"]
    pub(super) fn local_func__action_pass_floor_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase25ACTION_PASS_FLOOR__updateEv"]
    pub(super) fn ACTION_PASS_FLOOR__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase31local_func__action_pass_floor_2Ev"]
    pub(super) fn local_func__action_pass_floor_2(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase27local_func__action_meteor_2Ev"]
    pub(super) fn local_func__action_meteor_2(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36ACTION_METEOR__interrupt_aerial_jumpEv"]
    pub(super) fn ACTION_METEOR__interrupt_aerial_jump(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase30ACTION_METEOR__interrupt_guardEv"]
    pub(super) fn ACTION_METEOR__interrupt_guard(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase34ACTION_METEOR__set_no_return_frameEv"]
    pub(super) fn ACTION_METEOR__set_no_return_frame(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase25ACTION_METEOR__lot_actionEv"]
    pub(super) fn ACTION_METEOR__lot_action(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase27local_func__action_meteor_3Ev"]
    pub(super) fn local_func__action_meteor_3(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase34ACTION_METEOR__is_target_on_returnEv"]
    pub(super) fn ACTION_METEOR__is_target_on_return(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase21ACTION_METEOR__updateEv"]
    pub(super) fn ACTION_METEOR__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase19ACTION_METEOR__initEv"]
    pub(super) fn ACTION_METEOR__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase27local_func__action_meteor_1Ev"]
    pub(super) fn local_func__action_meteor_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase30local_func__action_machstamp_1Ev"]
    pub(super) fn local_func__action_machstamp_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase32ACTION_MACHSTAMP__interrupt_diveEv"]
    pub(super) fn ACTION_MACHSTAMP__interrupt_dive(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase24ACTION_MACHSTAMP__updateEv"]
    pub(super) fn ACTION_MACHSTAMP__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase22ACTION_MACHSTAMP__initEv"]
    pub(super) fn ACTION_MACHSTAMP__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36local_func__action_jump_s_n_attack_1Ev"]
    pub(super) fn local_func__action_jump_s_n_attack_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase30ACTION_JUMP_S_N_ATTACK__updateEv"]
    pub(super) fn ACTION_JUMP_S_N_ATTACK__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase28ACTION_JUMP_S_N_ATTACK__initEv"]
    pub(super) fn ACTION_JUMP_S_N_ATTACK__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase18is_cmd_jump_attackEN3lib8L2CValueE"]
    pub(super) fn is_cmd_jump_attack(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase29local_func__action_jump_s_b_1Ev"]
    pub(super) fn local_func__action_jump_s_b_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase23ACTION_JUMP_S_B__updateEv"]
    pub(super) fn ACTION_JUMP_S_B__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase34local_func__action_jump_s_attack_1Ev"]
    pub(super) fn local_func__action_jump_s_attack_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase28ACTION_JUMP_S_ATTACK__updateEv"]
    pub(super) fn ACTION_JUMP_S_ATTACK__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase26ACTION_JUMP_S_ATTACK__initEv"]
    pub(super) fn ACTION_JUMP_S_ATTACK__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase34local_func__action_jump_attack_f_1Ev"]
    pub(super) fn local_func__action_jump_attack_f_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase28ACTION_JUMP_ATTACK_F__updateEv"]
    pub(super) fn ACTION_JUMP_ATTACK_F__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase26ACTION_JUMP_ATTACK_F__initEv"]
    pub(super) fn ACTION_JUMP_ATTACK_F__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase32local_func__action_jump_attack_1Ev"]
    pub(super) fn local_func__action_jump_attack_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase26ACTION_JUMP_ATTACK__updateEv"]
    pub(super) fn ACTION_JUMP_ATTACK__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase24ACTION_JUMP_ATTACK__initEv"]
    pub(super) fn ACTION_JUMP_ATTACK__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36local_func__action_high_speed_dash_1Ev"]
    pub(super) fn local_func__action_high_speed_dash_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase39ACTION_HIGH_SPEED_DASH__interrupt_comboEv"]
    pub(super) fn ACTION_HIGH_SPEED_DASH__interrupt_combo(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase30ACTION_HIGH_SPEED_DASH__updateEv"]
    pub(super) fn ACTION_HIGH_SPEED_DASH__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase26local_func__action_guard_1Ev"]
    pub(super) fn local_func__action_guard_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase20ACTION_GUARD__updateEv"]
    pub(super) fn ACTION_GUARD__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase26analyst_is_not_need_shieldEN3lib8L2CValueE"]
    pub(super) fn analyst_is_not_need_shield(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase18predict_attack_hitEN3lib8L2CValueES2_"]
    pub(super) fn predict_attack_hit(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase28ACTION_GUARD__update_on_stopEv"]
    pub(super) fn ACTION_GUARD__update_on_stop(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase30ACTION_GUARD__notify_on_shieldEN3lib8L2CValueE"]
    pub(super) fn ACTION_GUARD__notify_on_shield(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase18ACTION_GUARD__initEv"]
    pub(super) fn ACTION_GUARD__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase29ACTION_FINAL_END_JUMP__updateEv"]
    pub(super) fn ACTION_FINAL_END_JUMP__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase25local_func__action_fall_1Ev"]
    pub(super) fn local_func__action_fall_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase19ACTION_FALL__updateEv"]
    pub(super) fn ACTION_FALL__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase25local_func__action_fall_2Ev"]
    pub(super) fn local_func__action_fall_2(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase17ACTION_FALL__initEv"]
    pub(super) fn ACTION_FALL__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase32local_func__action_escape_near_1Ev"]
    pub(super) fn local_func__action_escape_near_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase26ACTION_ESCAPE_NEAR__updateEv"]
    pub(super) fn ACTION_ESCAPE_NEAR__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase31local_func__action_escape_far_1Ev"]
    pub(super) fn local_func__action_escape_far_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase25ACTION_ESCAPE_FAR__updateEv"]
    pub(super) fn ACTION_ESCAPE_FAR__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36local_func__action_escape_air_move_1Ev"]
    pub(super) fn local_func__action_escape_air_move_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase30ACTION_ESCAPE_AIR_MOVE__updateEv"]
    pub(super) fn ACTION_ESCAPE_AIR_MOVE__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase28ACTION_ESCAPE_AIR_MOVE__initEv"]
    pub(super) fn ACTION_ESCAPE_AIR_MOVE__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36local_func__action_escape_air_move_2Ev"]
    pub(super) fn local_func__action_escape_air_move_2(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36local_func__action_escape_air_move_3Ev"]
    pub(super) fn local_func__action_escape_air_move_3(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase31local_func__action_escape_air_1Ev"]
    pub(super) fn local_func__action_escape_air_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase25ACTION_ESCAPE_AIR__updateEv"]
    pub(super) fn ACTION_ESCAPE_AIR__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase27local_func__action_escape_1Ev"]
    pub(super) fn local_func__action_escape_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase20ACTION_ENTRY__updateEv"]
    pub(super) fn ACTION_ENTRY__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase26local_func__action_drill_1Ev"]
    pub(super) fn local_func__action_drill_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase20ACTION_DRILL__updateEv"]
    pub(super) fn ACTION_DRILL__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase18ACTION_DRILL__initEv"]
    pub(super) fn ACTION_DRILL__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase28local_func__action_dragoon_1Ev"]
    pub(super) fn local_func__action_dragoon_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase22ACTION_DRAGOON__updateEv"]
    pub(super) fn ACTION_DRAGOON__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase20ACTION_DRAGOON__initEv"]
    pub(super) fn ACTION_DRAGOON__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase25local_func__action_down_2Ev"]
    pub(super) fn local_func__action_down_2(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase19ACTION_DOWN__updateEv"]
    pub(super) fn ACTION_DOWN__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase17ACTION_DOWN__initEv"]
    pub(super) fn ACTION_DOWN__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase25local_func__action_down_3Ev"]
    pub(super) fn local_func__action_down_3(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase25local_func__action_down_4Ev"]
    pub(super) fn local_func__action_down_4(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase25local_func__action_down_1Ev"]
    pub(super) fn local_func__action_down_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase41local_func__action_deadarea_dash_attack_1Ev"]
    pub(super) fn local_func__action_deadarea_dash_attack_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35ACTION_DEADAREA_DASH_ATTACK__updateEv"]
    pub(super) fn ACTION_DEADAREA_DASH_ATTACK__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase40local_func__action_deadarea_air_attack_1Ev"]
    pub(super) fn local_func__action_deadarea_air_attack_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase50ACTION_DEADAREA_AIR_ATTACK__interrupt_guard_weaponEv"]
    pub(super) fn ACTION_DEADAREA_AIR_ATTACK__interrupt_guard_weapon(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase43ACTION_DEADAREA_AIR_ATTACK__interrupt_guardEv"]
    pub(super) fn ACTION_DEADAREA_AIR_ATTACK__interrupt_guard(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase44ACTION_DEADAREA_AIR_ATTACK__interrupt_attackEv"]
    pub(super) fn ACTION_DEADAREA_AIR_ATTACK__interrupt_attack(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase34ACTION_DEADAREA_AIR_ATTACK__updateEv"]
    pub(super) fn ACTION_DEADAREA_AIR_ATTACK__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase32ACTION_DEADAREA_AIR_ATTACK__initEv"]
    pub(super) fn ACTION_DEADAREA_AIR_ATTACK__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase32local_func__action_dash_roll_f_1Ev"]
    pub(super) fn local_func__action_dash_roll_f_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase26ACTION_DASH_ROLL_F__updateEv"]
    pub(super) fn ACTION_DASH_ROLL_F__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase32local_func__action_dash_roll_b_1Ev"]
    pub(super) fn local_func__action_dash_roll_b_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase26ACTION_DASH_ROLL_B__updateEv"]
    pub(super) fn ACTION_DASH_ROLL_B__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase31local_func__action_dash_guard_1Ev"]
    pub(super) fn local_func__action_dash_guard_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase30ACTION_DASH_GUARD__next_actionEv"]
    pub(super) fn ACTION_DASH_GUARD__next_action(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase25ACTION_DASH_GUARD__updateEv"]
    pub(super) fn ACTION_DASH_GUARD__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase32local_func__action_dash_f_wait_1Ev"]
    pub(super) fn local_func__action_dash_f_wait_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase26ACTION_DASH_F_WAIT__updateEv"]
    pub(super) fn ACTION_DASH_F_WAIT__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase34local_func__action_dash_f_jump_b_1Ev"]
    pub(super) fn local_func__action_dash_f_jump_b_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37ACTION_DASH_F_JUMP_B__interrupt_guardEv"]
    pub(super) fn ACTION_DASH_F_JUMP_B__interrupt_guard(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase28ACTION_DASH_F_JUMP_B__updateEv"]
    pub(super) fn ACTION_DASH_F_JUMP_B__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase34local_func__action_dash_f_dash_b_1Ev"]
    pub(super) fn local_func__action_dash_f_dash_b_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase29global__tactics___unlock_wazaEv"]
    pub(super) fn global__tactics___unlock_waza(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase28ACTION_DASH_F_DASH_B__updateEv"]
    pub(super) fn ACTION_DASH_F_DASH_B__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase26ACTION_DASH_F_DASH_B__initEv"]
    pub(super) fn ACTION_DASH_F_DASH_B__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase32global__tactics___lock_waza_dashEv"]
    pub(super) fn global__tactics___lock_waza_dash(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36local_func__mode_global_variables_19Ev"]
    pub(super) fn local_func__mode_global_variables_19(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase34local_func__action_dash_b_jump_s_1Ev"]
    pub(super) fn local_func__action_dash_b_jump_s_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase28ACTION_DASH_B_JUMP_S__updateEv"]
    pub(super) fn ACTION_DASH_B_JUMP_S__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase34local_func__action_dash_b_dash_f_1Ev"]
    pub(super) fn local_func__action_dash_b_dash_f_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase28ACTION_DASH_B_DASH_F__updateEv"]
    pub(super) fn ACTION_DASH_B_DASH_F__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase26ACTION_DASH_B_DASH_F__initEv"]
    pub(super) fn ACTION_DASH_B_DASH_F__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase27local_func__action_dash_b_1Ev"]
    pub(super) fn local_func__action_dash_b_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase21ACTION_DASH_B__updateEv"]
    pub(super) fn ACTION_DASH_B__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase19ACTION_DASH_B__initEv"]
    pub(super) fn ACTION_DASH_B__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase32local_func__action_dash_attack_1Ev"]
    pub(super) fn local_func__action_dash_attack_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase28local_func__action_damaged_1Ev"]
    pub(super) fn local_func__action_damaged_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37ACTION_DAMAGED__function_input_groundEv"]
    pub(super) fn ACTION_DAMAGED__function_input_ground(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase34ACTION_DAMAGED__function_input_airEv"]
    pub(super) fn ACTION_DAMAGED__function_input_air(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase30ACTION_DAMAGED__hit_stop_slideEv"]
    pub(super) fn ACTION_DAMAGED__hit_stop_slide(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase22ACTION_DAMAGED__updateEv"]
    pub(super) fn ACTION_DAMAGED__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase20ACTION_DAMAGED__initEv"]
    pub(super) fn ACTION_DAMAGED__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase26local_func__action_cliff_2Ev"]
    pub(super) fn local_func__action_cliff_2(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase20ACTION_CLIFF__updateEv"]
    pub(super) fn ACTION_CLIFF__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase18ACTION_CLIFF__initEv"]
    pub(super) fn ACTION_CLIFF__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase26local_func__action_cliff_3Ev"]
    pub(super) fn local_func__action_cliff_3(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase26local_func__action_cliff_4Ev"]
    pub(super) fn local_func__action_cliff_4(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase26local_func__action_cliff_5Ev"]
    pub(super) fn local_func__action_cliff_5(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase26local_func__action_cliff_1Ev"]
    pub(super) fn local_func__action_cliff_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase28local_func__action_buildup_1Ev"]
    pub(super) fn local_func__action_buildup_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase42local_func__action_avoid_koopa_special_s_1Ev"]
    pub(super) fn local_func__action_avoid_koopa_special_s_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36ACTION_AVOID_KOOPA_SPECIAL_S__updateEv"]
    pub(super) fn ACTION_AVOID_KOOPA_SPECIAL_S__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase34ACTION_AVOID_KOOPA_SPECIAL_S__initEv"]
    pub(super) fn ACTION_AVOID_KOOPA_SPECIAL_S__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase46local_func__action_avoid_ganon_special_air_s_1Ev"]
    pub(super) fn local_func__action_avoid_ganon_special_air_s_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase40ACTION_AVOID_GANON_SPECIAL_AIR_S__updateEv"]
    pub(super) fn ACTION_AVOID_GANON_SPECIAL_AIR_S__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38ACTION_AVOID_GANON_SPECIAL_AIR_S__initEv"]
    pub(super) fn ACTION_AVOID_GANON_SPECIAL_AIR_S__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase44local_func__action_avoid_donkey_special_lw_1Ev"]
    pub(super) fn local_func__action_avoid_donkey_special_lw_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase52ACTION_AVOID_DONKEY_SPECIAL_LW__functon_common_inputEv"]
    pub(super) fn ACTION_AVOID_DONKEY_SPECIAL_LW__functon_common_input(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38ACTION_AVOID_DONKEY_SPECIAL_LW__updateEv"]
    pub(super) fn ACTION_AVOID_DONKEY_SPECIAL_LW__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase48ACTION_AVOID_DONKEY_SPECIAL_LW__notify_on_shieldEN3lib8L2CValueE"]
    pub(super) fn ACTION_AVOID_DONKEY_SPECIAL_LW__notify_on_shield(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36ACTION_AVOID_DONKEY_SPECIAL_LW__initEv"]
    pub(super) fn ACTION_AVOID_DONKEY_SPECIAL_LW__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase31local_func__action_attack_123_1Ev"]
    pub(super) fn local_func__action_attack_123_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase25ACTION_ATTACK_123__updateEv"]
    pub(super) fn ACTION_ATTACK_123__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase23ACTION_ATTACK_123__initEv"]
    pub(super) fn ACTION_ATTACK_123__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase21ACTION_APPEAL__updateEv"]
    pub(super) fn ACTION_APPEAL__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase27local_func__action_appeal_1Ev"]
    pub(super) fn local_func__action_appeal_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase31local_func__action_air_attack_1Ev"]
    pub(super) fn local_func__action_air_attack_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase41ACTION_AIR_ATTACK__interrupt_guard_weaponEv"]
    pub(super) fn ACTION_AIR_ATTACK__interrupt_guard_weapon(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase34ACTION_AIR_ATTACK__interrupt_guardEv"]
    pub(super) fn ACTION_AIR_ATTACK__interrupt_guard(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35ACTION_AIR_ATTACK__interrupt_attackEv"]
    pub(super) fn ACTION_AIR_ATTACK__interrupt_attack(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase25ACTION_AIR_ATTACK__updateEv"]
    pub(super) fn ACTION_AIR_ATTACK__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase23ACTION_AIR_ATTACK__initEv"]
    pub(super) fn ACTION_AIR_ATTACK__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase5enum_Ev"]
    pub(super) fn enum_(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36local_func__mode_private_variables_2Ev"]
    pub(super) fn local_func__mode_private_variables_2(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36local_func__mode_private_variables_1Ev"]
    pub(super) fn local_func__mode_private_variables_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36local_func__mode_global_variables_16Ev"]
    pub(super) fn local_func__mode_global_variables_16(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase33global__learning___set_guard_rateEN3lib8L2CValueE"]
    pub(super) fn global__learning___set_guard_rate(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase47global__learning___update_guard_rate_on_damagedEv"]
    pub(super) fn global__learning___update_guard_rate_on_damaged(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase43global__learning___end_observe_guard_statusEv"]
    pub(super) fn global__learning___end_observe_guard_status(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35local_func__mode_global_variables_3Ev"]
    pub(super) fn local_func__mode_global_variables_3(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35local_func__mode_global_variables_4Ev"]
    pub(super) fn local_func__mode_global_variables_4(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35local_func__mode_global_variables_5Ev"]
    pub(super) fn local_func__mode_global_variables_5(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35local_func__mode_global_variables_6Ev"]
    pub(super) fn local_func__mode_global_variables_6(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35local_func__mode_global_variables_7Ev"]
    pub(super) fn local_func__mode_global_variables_7(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36local_func__mode_global_variables_10Ev"]
    pub(super) fn local_func__mode_global_variables_10(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36local_func__mode_global_variables_11Ev"]
    pub(super) fn local_func__mode_global_variables_11(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36local_func__mode_global_variables_12Ev"]
    pub(super) fn local_func__mode_global_variables_12(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36local_func__mode_global_variables_13Ev"]
    pub(super) fn local_func__mode_global_variables_13(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36local_func__mode_global_variables_14Ev"]
    pub(super) fn local_func__mode_global_variables_14(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36local_func__mode_global_variables_15Ev"]
    pub(super) fn local_func__mode_global_variables_15(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35local_func__mode_global_variables_8Ev"]
    pub(super) fn local_func__mode_global_variables_8(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35local_func__mode_global_variables_9Ev"]
    pub(super) fn local_func__mode_global_variables_9(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35local_func__mode_global_variables_1Ev"]
    pub(super) fn local_func__mode_global_variables_1(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35local_func__mode_global_variables_2Ev"]
    pub(super) fn local_func__mode_global_variables_2(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase28specializer_interrupt_actionEN3lib8L2CValueES2_S2_"]
    pub(super) fn specializer_interrupt_action(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase20ACTION_BUILDUP__nameEv"]
    pub(super) fn ACTION_BUILDUP__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase20ACTION_BUILDUP__initEv"]
    pub(super) fn ACTION_BUILDUP__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase22ACTION_BUILDUP__updateEv"]
    pub(super) fn ACTION_BUILDUP__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase22ACTION_BUILDUP__cancelEv"]
    pub(super) fn ACTION_BUILDUP__cancel(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase32ACTION_BUILDUP__interrupt_attackEv"]
    pub(super) fn ACTION_BUILDUP__interrupt_attack(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase18ACTION_COMBO__nameEv"]
    pub(super) fn ACTION_COMBO__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase18ACTION_COMBO__initEv"]
    pub(super) fn ACTION_COMBO__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase20ACTION_COMBO__updateEv"]
    pub(super) fn ACTION_COMBO__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase30ACTION_COMBO__interrupt_attackEv"]
    pub(super) fn ACTION_COMBO__interrupt_attack(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase29ACTION_COMBO__interrupt_guardEv"]
    pub(super) fn ACTION_COMBO__interrupt_guard(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36ACTION_COMBO__interrupt_guard_weaponEv"]
    pub(super) fn ACTION_COMBO__interrupt_guard_weapon(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase28ACTION_COMBO__interrupt_diveEv"]
    pub(super) fn ACTION_COMBO__interrupt_dive(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35ACTION_COMBO__interrupt_aerial_jumpEv"]
    pub(super) fn ACTION_COMBO__interrupt_aerial_jump(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase24ACTION_DASH_ATTACK__nameEv"]
    pub(super) fn ACTION_DASH_ATTACK__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase24ACTION_DASH_ATTACK__initEv"]
    pub(super) fn ACTION_DASH_ATTACK__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase26ACTION_DASH_ATTACK__updateEv"]
    pub(super) fn ACTION_DASH_ATTACK__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase24ACTION_DASH_ATTACK__exitEv"]
    pub(super) fn ACTION_DASH_ATTACK__exit(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase19ACTION_ESCAPE__nameEv"]
    pub(super) fn ACTION_ESCAPE__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase19ACTION_ESCAPE__initEv"]
    pub(super) fn ACTION_ESCAPE__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase21ACTION_ESCAPE__updateEv"]
    pub(super) fn ACTION_ESCAPE__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase31ACTION_ESCAPE__interrupt_attackEv"]
    pub(super) fn ACTION_ESCAPE__interrupt_attack(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase30ACTION_ESCAPE__interrupt_guardEv"]
    pub(super) fn ACTION_ESCAPE__interrupt_guard(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37ACTION_ESCAPE__interrupt_guard_weaponEv"]
    pub(super) fn ACTION_ESCAPE__interrupt_guard_weapon(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase29ACTION_ESCAPE__interrupt_diveEv"]
    pub(super) fn ACTION_ESCAPE__interrupt_dive(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase21ACTION_STROLL_L__nameEv"]
    pub(super) fn ACTION_STROLL_L__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase21ACTION_STROLL_L__initEv"]
    pub(super) fn ACTION_STROLL_L__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase23ACTION_STROLL_L__updateEv"]
    pub(super) fn ACTION_STROLL_L__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase22ACTION_STROLL_L__resetEv"]
    pub(super) fn ACTION_STROLL_L__reset(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase21ACTION_STROLL_S__nameEv"]
    pub(super) fn ACTION_STROLL_S__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase21ACTION_STROLL_S__initEv"]
    pub(super) fn ACTION_STROLL_S__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase23ACTION_STROLL_S__updateEv"]
    pub(super) fn ACTION_STROLL_S__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase22ACTION_STROLL_S__resetEv"]
    pub(super) fn ACTION_STROLL_S__reset(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase25ACTION_STROLL_SQUAT__nameEv"]
    pub(super) fn ACTION_STROLL_SQUAT__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase25ACTION_STROLL_SQUAT__initEv"]
    pub(super) fn ACTION_STROLL_SQUAT__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase27ACTION_STROLL_SQUAT__updateEv"]
    pub(super) fn ACTION_STROLL_SQUAT__update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase26ACTION_STROLL_SQUAT__resetEv"]
    pub(super) fn ACTION_STROLL_SQUAT__reset(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase50ACTION_DASH_ATTACK__interrupt_auto_avoid_dead_zoneEv"]
    pub(super) fn ACTION_DASH_ATTACK__interrupt_auto_avoid_dead_zone(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase28local_func__action_buildup_2Ev"]
    pub(super) fn local_func__action_buildup_2(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase28local_func__action_buildup_3Ev"]
    pub(super) fn local_func__action_buildup_3(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase19specializer_buildupEv"]
    pub(super) fn specializer_buildup(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase23change_mode_action_noneEv"]
    pub(super) fn change_mode_action_none(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase30set_action_probability_mul_1stEN3lib8L2CValueES2_"]
    pub(super) fn set_action_probability_mul_1st(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase29change_mode_action_no_restartEN3lib8L2CValueE"]
    pub(super) fn change_mode_action_no_restart(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase27specializer_interrupt_comboEv"]
    pub(super) fn specializer_interrupt_combo(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase17notify_on_captureEv"]
    pub(super) fn notify_on_capture(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase31change_mode_action_by_interruptEN3lib8L2CValueE"]
    pub(super) fn change_mode_action_by_interrupt(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase28specializer_interrupt_alwaysEv"]
    pub(super) fn specializer_interrupt_always(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase26specializer_interrupt_diveEv"]
    pub(super) fn specializer_interrupt_dive(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38specializer_interrupt_action_on_meteorEv"]
    pub(super) fn specializer_interrupt_action_on_meteor(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36change_mode_action_none_by_interruptEv"]
    pub(super) fn change_mode_action_none_by_interrupt(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase33specializer_interrupt_action_postEv"]
    pub(super) fn specializer_interrupt_action_post(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase25specializer_has_reflectorEv"]
    pub(super) fn specializer_has_reflector(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase26specializer_reflect_cmd_idEv"]
    pub(super) fn specializer_reflect_cmd_id(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase24specializer_has_absorberEv"]
    pub(super) fn specializer_has_absorber(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase25specializer_absorb_cmd_idEv"]
    pub(super) fn specializer_absorb_cmd_id(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase26select_guard_action_groundEv"]
    pub(super) fn select_guard_action_ground(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35specializer_can_use_before_buildmaxEv"]
    pub(super) fn specializer_can_use_before_buildmax(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35specializer_disable_buildmax_cmd_idEv"]
    pub(super) fn specializer_disable_buildmax_cmd_id(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35specializer_restrict_always_commandEv"]
    pub(super) fn specializer_restrict_always_command(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase30get_action_probability_mul_1stEN3lib8L2CValueE"]
    pub(super) fn get_action_probability_mul_1st(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase30set_action_probability_mul_2ndEN3lib8L2CValueES2_"]
    pub(super) fn set_action_probability_mul_2nd(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase30get_action_probability_mul_2ndEN3lib8L2CValueE"]
    pub(super) fn get_action_probability_mul_2nd(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase30set_action_probability_mul_3rdEN3lib8L2CValueES2_"]
    pub(super) fn set_action_probability_mul_3rd(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase30get_action_probability_mul_3rdEN3lib8L2CValueE"]
    pub(super) fn get_action_probability_mul_3rd(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38set_action_probability_mul_2nd_foreachEN3lib8L2CValueES2_"]
    pub(super) fn set_action_probability_mul_2nd_foreach(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase32reset_action_probability_mul_3rdEv"]
    pub(super) fn reset_action_probability_mul_3rd(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase23is_movement_mode_actionEN3lib8L2CValueE"]
    pub(super) fn is_movement_mode_action(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase19is_wait_mode_actionEN3lib8L2CValueE"]
    pub(super) fn is_wait_mode_action(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase21is_stroll_mode_actionEN3lib8L2CValueE"]
    pub(super) fn is_stroll_mode_action(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase28is_dash_backward_mode_actionEN3lib8L2CValueE"]
    pub(super) fn is_dash_backward_mode_action(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase28is_jump_s_attack_mode_actionEN3lib8L2CValueE"]
    pub(super) fn is_jump_s_attack_mode_action(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase20is_guard_mode_actionEN3lib8L2CValueE"]
    pub(super) fn is_guard_mode_action(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase21is_escape_mode_actionEN3lib8L2CValueE"]
    pub(super) fn is_escape_mode_action(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase25is_large_jump_mode_actionEN3lib8L2CValueE"]
    pub(super) fn is_large_jump_mode_action(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35set_action_probability_weapondefmaxEv"]
    pub(super) fn set_action_probability_weapondefmax(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase11init_eventsEv"]
    pub(super) fn init_events(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase16init_specializerEv"]
    pub(super) fn init_specializer(this: *mut L2CFighterAIModeBase);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase32global__tactics___change_tacticsEN3lib8L2CValueE"]
    pub(super) fn global__tactics___change_tactics(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase20call_function_updateEv"]
    pub(super) fn call_function_update(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36call_function_is_running_mode_actionEv"]
    pub(super) fn call_function_is_running_mode_action(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase52call_function_set_action_probability_mul_as_grounderEv"]
    pub(super) fn call_function_set_action_probability_mul_as_grounder(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase50call_function_set_action_probability_mul_as_jumperEv"]
    pub(super) fn call_function_set_action_probability_mul_as_jumper(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase50call_function_set_action_probability_mul_as_dasherEv"]
    pub(super) fn call_function_set_action_probability_mul_as_dasher(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase50call_function_set_action_probability_mul_as_walkerEv"]
    pub(super) fn call_function_set_action_probability_mul_as_walker(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase51call_function_set_action_probability_mul_as_chargerEv"]
    pub(super) fn call_function_set_action_probability_mul_as_charger(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase51call_function_set_action_probability_mul_as_forwardEv"]
    pub(super) fn call_function_set_action_probability_mul_as_forward(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase49call_function_set_action_probability_weapondefmaxEv"]
    pub(super) fn call_function_set_action_probability_weapondefmax(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase21call_event_on_captureEv"]
    pub(super) fn call_event_on_capture(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase18regist_mode_actionEN3lib8L2CValueES2_"]
    pub(super) fn regist_mode_action(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase14is_cmd_specialEN3lib8L2CValueE"]
    pub(super) fn is_cmd_special(this: *mut L2CFighterAIModeBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase23ACTION_AIR_ATTACK__nameEv"]
    pub(super) fn ACTION_AIR_ATTACK__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase33ACTION_AIR_ATTACK__interrupt_diveEv"]
    pub(super) fn ACTION_AIR_ATTACK__interrupt_dive(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase49ACTION_AIR_ATTACK__interrupt_auto_avoid_dead_zoneEv"]
    pub(super) fn ACTION_AIR_ATTACK__interrupt_auto_avoid_dead_zone(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase19ACTION_APPEAL__nameEv"]
    pub(super) fn ACTION_APPEAL__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase19ACTION_APPEAL__initEv"]
    pub(super) fn ACTION_APPEAL__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase31ACTION_APPEAL__interrupt_attackEv"]
    pub(super) fn ACTION_APPEAL__interrupt_attack(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase30ACTION_APPEAL__interrupt_guardEv"]
    pub(super) fn ACTION_APPEAL__interrupt_guard(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37ACTION_APPEAL__interrupt_guard_weaponEv"]
    pub(super) fn ACTION_APPEAL__interrupt_guard_weapon(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase29ACTION_APPEAL__interrupt_diveEv"]
    pub(super) fn ACTION_APPEAL__interrupt_dive(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36ACTION_APPEAL__interrupt_aerial_jumpEv"]
    pub(super) fn ACTION_APPEAL__interrupt_aerial_jump(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase23ACTION_ATTACK_123__nameEv"]
    pub(super) fn ACTION_ATTACK_123__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35ACTION_ATTACK_123__interrupt_attackEv"]
    pub(super) fn ACTION_ATTACK_123__interrupt_attack(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase34ACTION_ATTACK_123__interrupt_guardEv"]
    pub(super) fn ACTION_ATTACK_123__interrupt_guard(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase41ACTION_ATTACK_123__interrupt_guard_weaponEv"]
    pub(super) fn ACTION_ATTACK_123__interrupt_guard_weapon(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase34ACTION_ATTACK_123__interrupt_comboEv"]
    pub(super) fn ACTION_ATTACK_123__interrupt_combo(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36ACTION_AVOID_DONKEY_SPECIAL_LW__nameEv"]
    pub(super) fn ACTION_AVOID_DONKEY_SPECIAL_LW__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase46ACTION_AVOID_DONKEY_SPECIAL_LW__update_on_stopEv"]
    pub(super) fn ACTION_AVOID_DONKEY_SPECIAL_LW__update_on_stop(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase48ACTION_AVOID_DONKEY_SPECIAL_LW__interrupt_attackEv"]
    pub(super) fn ACTION_AVOID_DONKEY_SPECIAL_LW__interrupt_attack(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase47ACTION_AVOID_DONKEY_SPECIAL_LW__interrupt_guardEv"]
    pub(super) fn ACTION_AVOID_DONKEY_SPECIAL_LW__interrupt_guard(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase54ACTION_AVOID_DONKEY_SPECIAL_LW__interrupt_guard_weaponEv"]
    pub(super) fn ACTION_AVOID_DONKEY_SPECIAL_LW__interrupt_guard_weapon(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38ACTION_AVOID_GANON_SPECIAL_AIR_S__nameEv"]
    pub(super) fn ACTION_AVOID_GANON_SPECIAL_AIR_S__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase34ACTION_AVOID_KOOPA_SPECIAL_S__nameEv"]
    pub(super) fn ACTION_AVOID_KOOPA_SPECIAL_S__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase18ACTION_CLIFF__nameEv"]
    pub(super) fn ACTION_CLIFF__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase30ACTION_CLIFF__interrupt_attackEv"]
    pub(super) fn ACTION_CLIFF__interrupt_attack(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase29ACTION_CLIFF__interrupt_guardEv"]
    pub(super) fn ACTION_CLIFF__interrupt_guard(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36ACTION_CLIFF__interrupt_guard_weaponEv"]
    pub(super) fn ACTION_CLIFF__interrupt_guard_weapon(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase28ACTION_CLIFF__interrupt_diveEv"]
    pub(super) fn ACTION_CLIFF__interrupt_dive(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase20ACTION_DAMAGED__nameEv"]
    pub(super) fn ACTION_DAMAGED__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase30ACTION_DAMAGED__update_on_stopEv"]
    pub(super) fn ACTION_DAMAGED__update_on_stop(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase32ACTION_DAMAGED__interrupt_attackEv"]
    pub(super) fn ACTION_DAMAGED__interrupt_attack(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase31ACTION_DAMAGED__interrupt_guardEv"]
    pub(super) fn ACTION_DAMAGED__interrupt_guard(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38ACTION_DAMAGED__interrupt_guard_weaponEv"]
    pub(super) fn ACTION_DAMAGED__interrupt_guard_weapon(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase30ACTION_DAMAGED__interrupt_diveEv"]
    pub(super) fn ACTION_DAMAGED__interrupt_dive(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37ACTION_DAMAGED__interrupt_aerial_jumpEv"]
    pub(super) fn ACTION_DAMAGED__interrupt_aerial_jump(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase19ACTION_DASH_B__nameEv"]
    pub(super) fn ACTION_DASH_B__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase31ACTION_DASH_B__interrupt_attackEv"]
    pub(super) fn ACTION_DASH_B__interrupt_attack(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase30ACTION_DASH_B__interrupt_guardEv"]
    pub(super) fn ACTION_DASH_B__interrupt_guard(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37ACTION_DASH_B__interrupt_guard_weaponEv"]
    pub(super) fn ACTION_DASH_B__interrupt_guard_weapon(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase45ACTION_DASH_B__interrupt_auto_avoid_dead_zoneEv"]
    pub(super) fn ACTION_DASH_B__interrupt_auto_avoid_dead_zone(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase26ACTION_DASH_B_DASH_F__nameEv"]
    pub(super) fn ACTION_DASH_B_DASH_F__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase26ACTION_DASH_B_DASH_F__exitEv"]
    pub(super) fn ACTION_DASH_B_DASH_F__exit(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38ACTION_DASH_B_DASH_F__interrupt_attackEv"]
    pub(super) fn ACTION_DASH_B_DASH_F__interrupt_attack(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase52ACTION_DASH_B_DASH_F__interrupt_auto_avoid_dead_zoneEv"]
    pub(super) fn ACTION_DASH_B_DASH_F__interrupt_auto_avoid_dead_zone(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase26ACTION_DASH_B_JUMP_S__nameEv"]
    pub(super) fn ACTION_DASH_B_JUMP_S__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase26ACTION_DASH_B_JUMP_S__initEv"]
    pub(super) fn ACTION_DASH_B_JUMP_S__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38ACTION_DASH_B_JUMP_S__interrupt_attackEv"]
    pub(super) fn ACTION_DASH_B_JUMP_S__interrupt_attack(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37ACTION_DASH_B_JUMP_S__interrupt_guardEv"]
    pub(super) fn ACTION_DASH_B_JUMP_S__interrupt_guard(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase44ACTION_DASH_B_JUMP_S__interrupt_guard_weaponEv"]
    pub(super) fn ACTION_DASH_B_JUMP_S__interrupt_guard_weapon(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36ACTION_DASH_B_JUMP_S__interrupt_diveEv"]
    pub(super) fn ACTION_DASH_B_JUMP_S__interrupt_dive(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase52ACTION_DASH_B_JUMP_S__interrupt_auto_avoid_dead_zoneEv"]
    pub(super) fn ACTION_DASH_B_JUMP_S__interrupt_auto_avoid_dead_zone(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase26ACTION_DASH_F_DASH_B__nameEv"]
    pub(super) fn ACTION_DASH_F_DASH_B__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase26ACTION_DASH_F_DASH_B__exitEv"]
    pub(super) fn ACTION_DASH_F_DASH_B__exit(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38ACTION_DASH_F_DASH_B__interrupt_attackEv"]
    pub(super) fn ACTION_DASH_F_DASH_B__interrupt_attack(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase52ACTION_DASH_F_DASH_B__interrupt_auto_avoid_dead_zoneEv"]
    pub(super) fn ACTION_DASH_F_DASH_B__interrupt_auto_avoid_dead_zone(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase26ACTION_DASH_F_JUMP_B__nameEv"]
    pub(super) fn ACTION_DASH_F_JUMP_B__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase26ACTION_DASH_F_JUMP_B__initEv"]
    pub(super) fn ACTION_DASH_F_JUMP_B__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38ACTION_DASH_F_JUMP_B__interrupt_attackEv"]
    pub(super) fn ACTION_DASH_F_JUMP_B__interrupt_attack(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase52ACTION_DASH_F_JUMP_B__interrupt_auto_avoid_dead_zoneEv"]
    pub(super) fn ACTION_DASH_F_JUMP_B__interrupt_auto_avoid_dead_zone(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase24ACTION_DASH_F_WAIT__nameEv"]
    pub(super) fn ACTION_DASH_F_WAIT__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase24ACTION_DASH_F_WAIT__initEv"]
    pub(super) fn ACTION_DASH_F_WAIT__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase50ACTION_DASH_F_WAIT__interrupt_auto_avoid_dead_zoneEv"]
    pub(super) fn ACTION_DASH_F_WAIT__interrupt_auto_avoid_dead_zone(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase23ACTION_DASH_GUARD__nameEv"]
    pub(super) fn ACTION_DASH_GUARD__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase23ACTION_DASH_GUARD__initEv"]
    pub(super) fn ACTION_DASH_GUARD__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35ACTION_DASH_GUARD__interrupt_attackEv"]
    pub(super) fn ACTION_DASH_GUARD__interrupt_attack(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase34ACTION_DASH_GUARD__interrupt_guardEv"]
    pub(super) fn ACTION_DASH_GUARD__interrupt_guard(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase49ACTION_DASH_GUARD__interrupt_auto_avoid_dead_zoneEv"]
    pub(super) fn ACTION_DASH_GUARD__interrupt_auto_avoid_dead_zone(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase24ACTION_DASH_ROLL_B__nameEv"]
    pub(super) fn ACTION_DASH_ROLL_B__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase24ACTION_DASH_ROLL_B__initEv"]
    pub(super) fn ACTION_DASH_ROLL_B__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36ACTION_DASH_ROLL_B__interrupt_attackEv"]
    pub(super) fn ACTION_DASH_ROLL_B__interrupt_attack(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35ACTION_DASH_ROLL_B__interrupt_guardEv"]
    pub(super) fn ACTION_DASH_ROLL_B__interrupt_guard(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase50ACTION_DASH_ROLL_B__interrupt_auto_avoid_dead_zoneEv"]
    pub(super) fn ACTION_DASH_ROLL_B__interrupt_auto_avoid_dead_zone(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase24ACTION_DASH_ROLL_F__nameEv"]
    pub(super) fn ACTION_DASH_ROLL_F__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase24ACTION_DASH_ROLL_F__initEv"]
    pub(super) fn ACTION_DASH_ROLL_F__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36ACTION_DASH_ROLL_F__interrupt_attackEv"]
    pub(super) fn ACTION_DASH_ROLL_F__interrupt_attack(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35ACTION_DASH_ROLL_F__interrupt_guardEv"]
    pub(super) fn ACTION_DASH_ROLL_F__interrupt_guard(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase50ACTION_DASH_ROLL_F__interrupt_auto_avoid_dead_zoneEv"]
    pub(super) fn ACTION_DASH_ROLL_F__interrupt_auto_avoid_dead_zone(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase32ACTION_DEADAREA_AIR_ATTACK__nameEv"]
    pub(super) fn ACTION_DEADAREA_AIR_ATTACK__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase42ACTION_DEADAREA_AIR_ATTACK__interrupt_diveEv"]
    pub(super) fn ACTION_DEADAREA_AIR_ATTACK__interrupt_dive(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase33ACTION_DEADAREA_DASH_ATTACK__nameEv"]
    pub(super) fn ACTION_DEADAREA_DASH_ATTACK__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase33ACTION_DEADAREA_DASH_ATTACK__initEv"]
    pub(super) fn ACTION_DEADAREA_DASH_ATTACK__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase17ACTION_DOWN__nameEv"]
    pub(super) fn ACTION_DOWN__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase29ACTION_DOWN__interrupt_attackEv"]
    pub(super) fn ACTION_DOWN__interrupt_attack(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase28ACTION_DOWN__interrupt_guardEv"]
    pub(super) fn ACTION_DOWN__interrupt_guard(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35ACTION_DOWN__interrupt_guard_weaponEv"]
    pub(super) fn ACTION_DOWN__interrupt_guard_weapon(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase27ACTION_DOWN__interrupt_diveEv"]
    pub(super) fn ACTION_DOWN__interrupt_dive(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase20ACTION_DRAGOON__nameEv"]
    pub(super) fn ACTION_DRAGOON__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase18ACTION_DRILL__nameEv"]
    pub(super) fn ACTION_DRILL__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase44ACTION_DRILL__interrupt_auto_avoid_dead_zoneEv"]
    pub(super) fn ACTION_DRILL__interrupt_auto_avoid_dead_zone(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase18ACTION_ENTRY__nameEv"]
    pub(super) fn ACTION_ENTRY__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase18ACTION_ENTRY__initEv"]
    pub(super) fn ACTION_ENTRY__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase30ACTION_ENTRY__interrupt_attackEv"]
    pub(super) fn ACTION_ENTRY__interrupt_attack(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase29ACTION_ENTRY__interrupt_guardEv"]
    pub(super) fn ACTION_ENTRY__interrupt_guard(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36ACTION_ENTRY__interrupt_guard_weaponEv"]
    pub(super) fn ACTION_ENTRY__interrupt_guard_weapon(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase28ACTION_ENTRY__interrupt_diveEv"]
    pub(super) fn ACTION_ENTRY__interrupt_dive(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35ACTION_ENTRY__interrupt_aerial_jumpEv"]
    pub(super) fn ACTION_ENTRY__interrupt_aerial_jump(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase23ACTION_ESCAPE_AIR__nameEv"]
    pub(super) fn ACTION_ESCAPE_AIR__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase23ACTION_ESCAPE_AIR__initEv"]
    pub(super) fn ACTION_ESCAPE_AIR__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35ACTION_ESCAPE_AIR__interrupt_attackEv"]
    pub(super) fn ACTION_ESCAPE_AIR__interrupt_attack(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase34ACTION_ESCAPE_AIR__interrupt_guardEv"]
    pub(super) fn ACTION_ESCAPE_AIR__interrupt_guard(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase41ACTION_ESCAPE_AIR__interrupt_guard_weaponEv"]
    pub(super) fn ACTION_ESCAPE_AIR__interrupt_guard_weapon(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase33ACTION_ESCAPE_AIR__interrupt_diveEv"]
    pub(super) fn ACTION_ESCAPE_AIR__interrupt_dive(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase28ACTION_ESCAPE_AIR_MOVE__nameEv"]
    pub(super) fn ACTION_ESCAPE_AIR_MOVE__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase40ACTION_ESCAPE_AIR_MOVE__interrupt_attackEv"]
    pub(super) fn ACTION_ESCAPE_AIR_MOVE__interrupt_attack(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase39ACTION_ESCAPE_AIR_MOVE__interrupt_guardEv"]
    pub(super) fn ACTION_ESCAPE_AIR_MOVE__interrupt_guard(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase46ACTION_ESCAPE_AIR_MOVE__interrupt_guard_weaponEv"]
    pub(super) fn ACTION_ESCAPE_AIR_MOVE__interrupt_guard_weapon(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38ACTION_ESCAPE_AIR_MOVE__interrupt_diveEv"]
    pub(super) fn ACTION_ESCAPE_AIR_MOVE__interrupt_dive(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase23ACTION_ESCAPE_FAR__nameEv"]
    pub(super) fn ACTION_ESCAPE_FAR__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase23ACTION_ESCAPE_FAR__initEv"]
    pub(super) fn ACTION_ESCAPE_FAR__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35ACTION_ESCAPE_FAR__interrupt_attackEv"]
    pub(super) fn ACTION_ESCAPE_FAR__interrupt_attack(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase34ACTION_ESCAPE_FAR__interrupt_guardEv"]
    pub(super) fn ACTION_ESCAPE_FAR__interrupt_guard(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase41ACTION_ESCAPE_FAR__interrupt_guard_weaponEv"]
    pub(super) fn ACTION_ESCAPE_FAR__interrupt_guard_weapon(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase33ACTION_ESCAPE_FAR__interrupt_diveEv"]
    pub(super) fn ACTION_ESCAPE_FAR__interrupt_dive(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase24ACTION_ESCAPE_NEAR__nameEv"]
    pub(super) fn ACTION_ESCAPE_NEAR__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase24ACTION_ESCAPE_NEAR__initEv"]
    pub(super) fn ACTION_ESCAPE_NEAR__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36ACTION_ESCAPE_NEAR__interrupt_attackEv"]
    pub(super) fn ACTION_ESCAPE_NEAR__interrupt_attack(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35ACTION_ESCAPE_NEAR__interrupt_guardEv"]
    pub(super) fn ACTION_ESCAPE_NEAR__interrupt_guard(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase42ACTION_ESCAPE_NEAR__interrupt_guard_weaponEv"]
    pub(super) fn ACTION_ESCAPE_NEAR__interrupt_guard_weapon(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase34ACTION_ESCAPE_NEAR__interrupt_diveEv"]
    pub(super) fn ACTION_ESCAPE_NEAR__interrupt_dive(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase17ACTION_FALL__nameEv"]
    pub(super) fn ACTION_FALL__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase29ACTION_FALL__interrupt_attackEv"]
    pub(super) fn ACTION_FALL__interrupt_attack(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase28ACTION_FALL__interrupt_guardEv"]
    pub(super) fn ACTION_FALL__interrupt_guard(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35ACTION_FALL__interrupt_guard_weaponEv"]
    pub(super) fn ACTION_FALL__interrupt_guard_weapon(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase27ACTION_FALL__interrupt_diveEv"]
    pub(super) fn ACTION_FALL__interrupt_dive(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase34ACTION_FALL__interrupt_aerial_jumpEv"]
    pub(super) fn ACTION_FALL__interrupt_aerial_jump(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase43ACTION_FALL__interrupt_auto_avoid_dead_zoneEv"]
    pub(super) fn ACTION_FALL__interrupt_auto_avoid_dead_zone(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase27ACTION_FINAL_END_JUMP__nameEv"]
    pub(super) fn ACTION_FINAL_END_JUMP__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase27ACTION_FINAL_END_JUMP__initEv"]
    pub(super) fn ACTION_FINAL_END_JUMP__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase53ACTION_FINAL_END_JUMP__interrupt_auto_avoid_dead_zoneEv"]
    pub(super) fn ACTION_FINAL_END_JUMP__interrupt_auto_avoid_dead_zone(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase18ACTION_GUARD__nameEv"]
    pub(super) fn ACTION_GUARD__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase30ACTION_GUARD__interrupt_attackEv"]
    pub(super) fn ACTION_GUARD__interrupt_attack(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase29ACTION_GUARD__interrupt_guardEv"]
    pub(super) fn ACTION_GUARD__interrupt_guard(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36ACTION_GUARD__interrupt_guard_weaponEv"]
    pub(super) fn ACTION_GUARD__interrupt_guard_weapon(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase28ACTION_HIGH_SPEED_DASH__nameEv"]
    pub(super) fn ACTION_HIGH_SPEED_DASH__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase28ACTION_HIGH_SPEED_DASH__initEv"]
    pub(super) fn ACTION_HIGH_SPEED_DASH__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase40ACTION_HIGH_SPEED_DASH__interrupt_attackEv"]
    pub(super) fn ACTION_HIGH_SPEED_DASH__interrupt_attack(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase39ACTION_HIGH_SPEED_DASH__interrupt_guardEv"]
    pub(super) fn ACTION_HIGH_SPEED_DASH__interrupt_guard(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase54ACTION_HIGH_SPEED_DASH__interrupt_auto_avoid_dead_zoneEv"]
    pub(super) fn ACTION_HIGH_SPEED_DASH__interrupt_auto_avoid_dead_zone(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase24ACTION_JUMP_ATTACK__nameEv"]
    pub(super) fn ACTION_JUMP_ATTACK__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36ACTION_JUMP_ATTACK__interrupt_attackEv"]
    pub(super) fn ACTION_JUMP_ATTACK__interrupt_attack(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35ACTION_JUMP_ATTACK__interrupt_guardEv"]
    pub(super) fn ACTION_JUMP_ATTACK__interrupt_guard(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase42ACTION_JUMP_ATTACK__interrupt_guard_weaponEv"]
    pub(super) fn ACTION_JUMP_ATTACK__interrupt_guard_weapon(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase34ACTION_JUMP_ATTACK__interrupt_diveEv"]
    pub(super) fn ACTION_JUMP_ATTACK__interrupt_dive(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase50ACTION_JUMP_ATTACK__interrupt_auto_avoid_dead_zoneEv"]
    pub(super) fn ACTION_JUMP_ATTACK__interrupt_auto_avoid_dead_zone(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase26ACTION_JUMP_ATTACK_F__nameEv"]
    pub(super) fn ACTION_JUMP_ATTACK_F__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38ACTION_JUMP_ATTACK_F__interrupt_attackEv"]
    pub(super) fn ACTION_JUMP_ATTACK_F__interrupt_attack(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37ACTION_JUMP_ATTACK_F__interrupt_guardEv"]
    pub(super) fn ACTION_JUMP_ATTACK_F__interrupt_guard(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase44ACTION_JUMP_ATTACK_F__interrupt_guard_weaponEv"]
    pub(super) fn ACTION_JUMP_ATTACK_F__interrupt_guard_weapon(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36ACTION_JUMP_ATTACK_F__interrupt_diveEv"]
    pub(super) fn ACTION_JUMP_ATTACK_F__interrupt_dive(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase52ACTION_JUMP_ATTACK_F__interrupt_auto_avoid_dead_zoneEv"]
    pub(super) fn ACTION_JUMP_ATTACK_F__interrupt_auto_avoid_dead_zone(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase26ACTION_JUMP_S_ATTACK__nameEv"]
    pub(super) fn ACTION_JUMP_S_ATTACK__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase42ACTION_JUMP_S_ATTACK__notify_on_attack_hitEv"]
    pub(super) fn ACTION_JUMP_S_ATTACK__notify_on_attack_hit(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase45ACTION_JUMP_S_ATTACK__notify_on_attack_shieldEv"]
    pub(super) fn ACTION_JUMP_S_ATTACK__notify_on_attack_shield(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38ACTION_JUMP_S_ATTACK__interrupt_attackEv"]
    pub(super) fn ACTION_JUMP_S_ATTACK__interrupt_attack(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37ACTION_JUMP_S_ATTACK__interrupt_guardEv"]
    pub(super) fn ACTION_JUMP_S_ATTACK__interrupt_guard(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36ACTION_JUMP_S_ATTACK__interrupt_diveEv"]
    pub(super) fn ACTION_JUMP_S_ATTACK__interrupt_dive(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase52ACTION_JUMP_S_ATTACK__interrupt_auto_avoid_dead_zoneEv"]
    pub(super) fn ACTION_JUMP_S_ATTACK__interrupt_auto_avoid_dead_zone(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase21ACTION_JUMP_S_B__nameEv"]
    pub(super) fn ACTION_JUMP_S_B__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase21ACTION_JUMP_S_B__initEv"]
    pub(super) fn ACTION_JUMP_S_B__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase33ACTION_JUMP_S_B__interrupt_attackEv"]
    pub(super) fn ACTION_JUMP_S_B__interrupt_attack(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase32ACTION_JUMP_S_B__interrupt_guardEv"]
    pub(super) fn ACTION_JUMP_S_B__interrupt_guard(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase39ACTION_JUMP_S_B__interrupt_guard_weaponEv"]
    pub(super) fn ACTION_JUMP_S_B__interrupt_guard_weapon(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase31ACTION_JUMP_S_B__interrupt_diveEv"]
    pub(super) fn ACTION_JUMP_S_B__interrupt_dive(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase28ACTION_JUMP_S_N_ATTACK__nameEv"]
    pub(super) fn ACTION_JUMP_S_N_ATTACK__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase44ACTION_JUMP_S_N_ATTACK__notify_on_attack_hitEv"]
    pub(super) fn ACTION_JUMP_S_N_ATTACK__notify_on_attack_hit(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase47ACTION_JUMP_S_N_ATTACK__notify_on_attack_shieldEv"]
    pub(super) fn ACTION_JUMP_S_N_ATTACK__notify_on_attack_shield(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase40ACTION_JUMP_S_N_ATTACK__interrupt_attackEv"]
    pub(super) fn ACTION_JUMP_S_N_ATTACK__interrupt_attack(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase39ACTION_JUMP_S_N_ATTACK__interrupt_guardEv"]
    pub(super) fn ACTION_JUMP_S_N_ATTACK__interrupt_guard(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38ACTION_JUMP_S_N_ATTACK__interrupt_diveEv"]
    pub(super) fn ACTION_JUMP_S_N_ATTACK__interrupt_dive(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase54ACTION_JUMP_S_N_ATTACK__interrupt_auto_avoid_dead_zoneEv"]
    pub(super) fn ACTION_JUMP_S_N_ATTACK__interrupt_auto_avoid_dead_zone(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase22ACTION_MACHSTAMP__nameEv"]
    pub(super) fn ACTION_MACHSTAMP__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase34ACTION_MACHSTAMP__interrupt_attackEv"]
    pub(super) fn ACTION_MACHSTAMP__interrupt_attack(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase33ACTION_MACHSTAMP__interrupt_guardEv"]
    pub(super) fn ACTION_MACHSTAMP__interrupt_guard(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase40ACTION_MACHSTAMP__interrupt_guard_weaponEv"]
    pub(super) fn ACTION_MACHSTAMP__interrupt_guard_weapon(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase48ACTION_MACHSTAMP__interrupt_auto_avoid_dead_zoneEv"]
    pub(super) fn ACTION_MACHSTAMP__interrupt_auto_avoid_dead_zone(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase19ACTION_METEOR__nameEv"]
    pub(super) fn ACTION_METEOR__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase34ACTION_METEOR__is_cancelable_phaseEv"]
    pub(super) fn ACTION_METEOR__is_cancelable_phase(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase31ACTION_METEOR__interrupt_attackEv"]
    pub(super) fn ACTION_METEOR__interrupt_attack(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37ACTION_METEOR__interrupt_guard_weaponEv"]
    pub(super) fn ACTION_METEOR__interrupt_guard_weapon(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase30ACTION_METEOR__interrupt_comboEv"]
    pub(super) fn ACTION_METEOR__interrupt_combo(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase29ACTION_METEOR__interrupt_diveEv"]
    pub(super) fn ACTION_METEOR__interrupt_dive(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase45ACTION_METEOR__interrupt_auto_avoid_dead_zoneEv"]
    pub(super) fn ACTION_METEOR__interrupt_auto_avoid_dead_zone(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase23ACTION_PASS_FLOOR__nameEv"]
    pub(super) fn ACTION_PASS_FLOOR__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase23ACTION_PASS_FLOOR__initEv"]
    pub(super) fn ACTION_PASS_FLOOR__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase35ACTION_PASS_FLOOR__interrupt_attackEv"]
    pub(super) fn ACTION_PASS_FLOOR__interrupt_attack(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase34ACTION_PASS_FLOOR__interrupt_guardEv"]
    pub(super) fn ACTION_PASS_FLOOR__interrupt_guard(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase41ACTION_PASS_FLOOR__interrupt_guard_weaponEv"]
    pub(super) fn ACTION_PASS_FLOOR__interrupt_guard_weapon(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase40ACTION_PASS_FLOOR__interrupt_aerial_jumpEv"]
    pub(super) fn ACTION_PASS_FLOOR__interrupt_aerial_jump(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase19ACTION_PURSUE__nameEv"]
    pub(super) fn ACTION_PURSUE__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase29ACTION_PURSUE__interrupt_diveEv"]
    pub(super) fn ACTION_PURSUE__interrupt_dive(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36ACTION_PURSUE__interrupt_aerial_jumpEv"]
    pub(super) fn ACTION_PURSUE__interrupt_aerial_jump(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase45ACTION_PURSUE__interrupt_auto_avoid_dead_zoneEv"]
    pub(super) fn ACTION_PURSUE__interrupt_auto_avoid_dead_zone(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase20ACTION_REBIRTH__nameEv"]
    pub(super) fn ACTION_REBIRTH__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase20ACTION_REBIRTH__initEv"]
    pub(super) fn ACTION_REBIRTH__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase31ACTION_REBIRTH__interrupt_guardEv"]
    pub(super) fn ACTION_REBIRTH__interrupt_guard(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase38ACTION_REBIRTH__interrupt_guard_weaponEv"]
    pub(super) fn ACTION_REBIRTH__interrupt_guard_weapon(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37ACTION_REBIRTH__interrupt_aerial_jumpEv"]
    pub(super) fn ACTION_REBIRTH__interrupt_aerial_jump(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase30ACTION_REBIRTH__interrupt_diveEv"]
    pub(super) fn ACTION_REBIRTH__interrupt_dive(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase19ACTION_RETURN__nameEv"]
    pub(super) fn ACTION_RETURN__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase30ACTION_RETURN__interrupt_guardEv"]
    pub(super) fn ACTION_RETURN__interrupt_guard(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase29ACTION_RETURN__interrupt_diveEv"]
    pub(super) fn ACTION_RETURN__interrupt_dive(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase36ACTION_RETURN__interrupt_aerial_jumpEv"]
    pub(super) fn ACTION_RETURN__interrupt_aerial_jump(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase30ACTION_RETURN__interrupt_comboEv"]
    pub(super) fn ACTION_RETURN__interrupt_combo(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase19ACTION_ROLL_B__nameEv"]
    pub(super) fn ACTION_ROLL_B__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase31ACTION_ROLL_B__interrupt_attackEv"]
    pub(super) fn ACTION_ROLL_B__interrupt_attack(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase30ACTION_ROLL_B__interrupt_guardEv"]
    pub(super) fn ACTION_ROLL_B__interrupt_guard(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37ACTION_ROLL_B__interrupt_guard_weaponEv"]
    pub(super) fn ACTION_ROLL_B__interrupt_guard_weapon(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase29ACTION_ROLL_B__interrupt_diveEv"]
    pub(super) fn ACTION_ROLL_B__interrupt_dive(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase19ACTION_ROLL_F__nameEv"]
    pub(super) fn ACTION_ROLL_F__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase31ACTION_ROLL_F__interrupt_attackEv"]
    pub(super) fn ACTION_ROLL_F__interrupt_attack(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase30ACTION_ROLL_F__interrupt_guardEv"]
    pub(super) fn ACTION_ROLL_F__interrupt_guard(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase37ACTION_ROLL_F__interrupt_guard_weaponEv"]
    pub(super) fn ACTION_ROLL_F__interrupt_guard_weapon(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase29ACTION_ROLL_F__interrupt_diveEv"]
    pub(super) fn ACTION_ROLL_F__interrupt_dive(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase16ACTION_RUN__nameEv"]
    pub(super) fn ACTION_RUN__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase42ACTION_RUN__interrupt_auto_avoid_dead_zoneEv"]
    pub(super) fn ACTION_RUN__interrupt_auto_avoid_dead_zone(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase23ACTION_RUN_ATTACK__nameEv"]
    pub(super) fn ACTION_RUN_ATTACK__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase47ACTION_STROLL_L__interrupt_auto_avoid_dead_zoneEv"]
    pub(super) fn ACTION_STROLL_L__interrupt_auto_avoid_dead_zone(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase47ACTION_STROLL_S__interrupt_auto_avoid_dead_zoneEv"]
    pub(super) fn ACTION_STROLL_S__interrupt_auto_avoid_dead_zone(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase51ACTION_STROLL_SQUAT__interrupt_auto_avoid_dead_zoneEv"]
    pub(super) fn ACTION_STROLL_SQUAT__interrupt_auto_avoid_dead_zone(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase22ACTION_TURN_TURN__nameEv"]
    pub(super) fn ACTION_TURN_TURN__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase22ACTION_TURN_TURN__initEv"]
    pub(super) fn ACTION_TURN_TURN__init(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase19ACTION_WAIT_L__nameEv"]
    pub(super) fn ACTION_WAIT_L__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase19ACTION_WAIT_S__nameEv"]
    pub(super) fn ACTION_WAIT_S__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase23ACTION_WAIT_SQUAT__nameEv"]
    pub(super) fn ACTION_WAIT_SQUAT__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase17ACTION_WALK__nameEv"]
    pub(super) fn ACTION_WALK__name(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp20L2CFighterAIModeBase43ACTION_WALK__interrupt_auto_avoid_dead_zoneEv"]
    pub(super) fn ACTION_WALK__interrupt_auto_avoid_dead_zone(this: *mut L2CFighterAIModeBase) -> lib::L2CValueHack;
}