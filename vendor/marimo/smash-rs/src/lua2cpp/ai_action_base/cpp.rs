use crate::*;
use super::class::*;

extern "C" {
    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase37local_func__action_global_variables_1Ev"]
    pub(super) fn local_func__action_global_variables_1(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase38local_func__action_private_variables_1Ev"]
    pub(super) fn local_func__action_private_variables_1(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase26main_func__act_a_catch_atkEv"]
    pub(super) fn main_func__act_a_catch_atk(this: *mut L2CFighterAIActionBase);

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase22main_func__act_a_finalEv"]
    pub(super) fn main_func__act_a_final(this: *mut L2CFighterAIActionBase);

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase20main_func__act_c_airEv"]
    pub(super) fn main_func__act_c_air(this: *mut L2CFighterAIActionBase);

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase23main_func__act_c_groundEv"]
    pub(super) fn main_func__act_c_ground(this: *mut L2CFighterAIActionBase);

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase25main_func__act_r_act_lastEv"]
    pub(super) fn main_func__act_r_act_last(this: *mut L2CFighterAIActionBase);

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase21main_func__act_r_jumpEv"]
    pub(super) fn main_func__act_r_jump(this: *mut L2CFighterAIActionBase);

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase26main_func__phase_c_commandEv"]
    pub(super) fn main_func__phase_c_command(this: *mut L2CFighterAIActionBase);

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase27call_event_on_attack_shieldEN3lib8L2CValueE"]
    pub(super) fn call_event_on_attack_shield(this: *mut L2CFighterAIActionBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase24call_event_on_attack_hitEv"]
    pub(super) fn call_event_on_attack_hit(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase30call_function_update_coroutineEv"]
    pub(super) fn call_function_update_coroutine(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase27call_function_change_actionEN3lib8L2CValueE"]
    pub(super) fn call_function_change_action(this: *mut L2CFighterAIActionBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase22init_private_variablesEv"]
    pub(super) fn init_private_variables(this: *mut L2CFighterAIActionBase);

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase13regist_actionEN3lib8L2CValueES2_"]
    pub(super) fn regist_action(this: *mut L2CFighterAIActionBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase22regist_action_overrideEN3lib8L2CValueES2_"]
    pub(super) fn regist_action_override(this: *mut L2CFighterAIActionBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase30global__COMMON__phase_sp_afterEv"]
    pub(super) fn global__COMMON__phase_sp_after(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase4exitEv"]
    pub(super) fn exit(this: *mut L2CFighterAIActionBase);

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase28local_func__phase_sp_after_1Ev"]
    pub(super) fn local_func__phase_sp_after_1(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase28local_func__phase_sp_after_2Ev"]
    pub(super) fn local_func__phase_sp_after_2(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase28local_func__phase_sp_after_3Ev"]
    pub(super) fn local_func__phase_sp_after_3(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase28global__COMMON__phase_r_jumpEv"]
    pub(super) fn global__COMMON__phase_r_jump(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase33global__COMMON__function_set_goalEN3lib8L2CValueE"]
    pub(super) fn global__COMMON__function_set_goal(this: *mut L2CFighterAIActionBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase29local_func__phase_c_command_1Ev"]
    pub(super) fn local_func__phase_c_command_1(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase35private___COMMON__phase_command_endEv"]
    pub(super) fn private___COMMON__phase_command_end(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase12change_phaseEN3lib8L2CValueES2_"]
    pub(super) fn change_phase(this: *mut L2CFighterAIActionBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase38private___COMMON__phase_command_start2Ev"]
    pub(super) fn private___COMMON__phase_command_start2(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase37private___COMMON__phase_command_startEv"]
    pub(super) fn private___COMMON__phase_command_start(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase41private___COMMON__phase_button_or_commandEv"]
    pub(super) fn private___COMMON__phase_button_or_command(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase50global__COMMON__function_change_phase_to_c_commandEN3lib8L2CValueES2_S2_S2_"]
    pub(super) fn global__COMMON__function_change_phase_to_c_command(this: *mut L2CFighterAIActionBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase35global__COMMON__function_sp_u_checkEN3lib8L2CValueE"]
    pub(super) fn global__COMMON__function_sp_u_check(this: *mut L2CFighterAIActionBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase33local_func__function_sp_u_check_1Ev"]
    pub(super) fn local_func__function_sp_u_check_1(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase29global__COMMON__function_sp_uEv"]
    pub(super) fn global__COMMON__function_sp_u(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase27local_func__function_sp_u_1Ev"]
    pub(super) fn local_func__function_sp_u_1(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase27local_func__function_sp_u_2Ev"]
    pub(super) fn local_func__function_sp_u_2(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase44global__COMMON__function_r_jump_timing_checkEN3lib8L2CValueES2_S2_S2_"]
    pub(super) fn global__COMMON__function_r_jump_timing_check(this: *mut L2CFighterAIActionBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase42global__COMMON__function_r_jump_sp_u_checkEN3lib8L2CValueE"]
    pub(super) fn global__COMMON__function_r_jump_sp_u_check(this: *mut L2CFighterAIActionBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase40local_func__function_r_jump_sp_u_check_1Ev"]
    pub(super) fn local_func__function_r_jump_sp_u_check_1(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase42global__COMMON__function_r_jump_exit_checkEv"]
    pub(super) fn global__COMMON__function_r_jump_exit_check(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase24local_func__act_r_jump_2Ev"]
    pub(super) fn local_func__act_r_jump_2(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase25private___R_JUMP__phase_1Ev"]
    pub(super) fn private___R_JUMP__phase_1(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase22private___R_JUMP__initEv"]
    pub(super) fn private___R_JUMP__init(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase24local_func__act_r_jump_1Ev"]
    pub(super) fn local_func__act_r_jump_1(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase28local_func__act_r_act_last_2Ev"]
    pub(super) fn local_func__act_r_act_last_2(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase29private___R_ACT_LAST__phase_2Ev"]
    pub(super) fn private___R_ACT_LAST__phase_2(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase29private___R_ACT_LAST__phase_1Ev"]
    pub(super) fn private___R_ACT_LAST__phase_1(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase28local_func__act_r_act_last_3Ev"]
    pub(super) fn local_func__act_r_act_last_3(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase26private___R_ACT_LAST__initEv"]
    pub(super) fn private___R_ACT_LAST__init(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase28local_func__act_r_act_last_1Ev"]
    pub(super) fn local_func__act_r_act_last_1(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase26local_func__act_c_ground_1Ev"]
    pub(super) fn local_func__act_c_ground_1(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase47private___C_GROUND__function_is_special_commandEN3lib8L2CValueE"]
    pub(super) fn private___C_GROUND__function_is_special_command(this: *mut L2CFighterAIActionBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase44private___C_GROUND__function_force_end_checkEv"]
    pub(super) fn private___C_GROUND__function_force_end_check(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase45private___C_GROUND__function_hi_low_use_checkEv"]
    pub(super) fn private___C_GROUND__function_hi_low_use_check(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase39private___C_GROUND__function_set_hi_lowEv"]
    pub(super) fn private___C_GROUND__function_set_hi_low(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase38private___C_GROUND__function_set_stickEv"]
    pub(super) fn private___C_GROUND__function_set_stick(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase42private___C_GROUND__function_set_end_frameEv"]
    pub(super) fn private___C_GROUND__function_set_end_frame(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase42private___C_GROUND__function_set_key_frameEv"]
    pub(super) fn private___C_GROUND__function_set_key_frame(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase43private___C_GROUND__function_check_stat_airEv"]
    pub(super) fn private___C_GROUND__function_check_stat_air(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase34private___C_GROUND__phase_end_waitEv"]
    pub(super) fn private___C_GROUND__phase_end_wait(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase37private___C_GROUND__phase_hi_low_waitEv"]
    pub(super) fn private___C_GROUND__phase_hi_low_wait(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase31private___C_GROUND__phase_startEv"]
    pub(super) fn private___C_GROUND__phase_start(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase45private___C_GROUND__function_is_status_shieldEv"]
    pub(super) fn private___C_GROUND__function_is_status_shield(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase36private___C_GROUND__phase_start_dashEv"]
    pub(super) fn private___C_GROUND__phase_start_dash(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase34private___C_GROUND__phase_check_lrEv"]
    pub(super) fn private___C_GROUND__phase_check_lr(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase33private___C_GROUND__phase_neutralEv"]
    pub(super) fn private___C_GROUND__phase_neutral(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase30private___C_GROUND__phase_initEv"]
    pub(super) fn private___C_GROUND__phase_init(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase24private___C_GROUND__initEv"]
    pub(super) fn private___C_GROUND__init(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase23local_func__act_c_air_1Ev"]
    pub(super) fn local_func__act_c_air_1(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase32private___C_AIR__function_set_lrEv"]
    pub(super) fn private___C_AIR__function_set_lr(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase36private___C_AIR__function_set_hi_lowEv"]
    pub(super) fn private___C_AIR__function_set_hi_low(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase30private___C_AIR__phase_lr_waitEv"]
    pub(super) fn private___C_AIR__phase_lr_wait(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase31private___C_AIR__phase_end_waitEv"]
    pub(super) fn private___C_AIR__phase_end_wait(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase34private___C_AIR__phase_hi_low_waitEv"]
    pub(super) fn private___C_AIR__phase_hi_low_wait(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase28private___C_AIR__phase_startEv"]
    pub(super) fn private___C_AIR__phase_start(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase32private___C_AIR__phase_pre_startEv"]
    pub(super) fn private___C_AIR__phase_pre_start(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase27private___C_AIR__phase_initEv"]
    pub(super) fn private___C_AIR__phase_init(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase21private___C_AIR__initEv"]
    pub(super) fn private___C_AIR__init(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase23private___A_FINAL__initEv"]
    pub(super) fn private___A_FINAL__init(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase29local_func__act_a_catch_atk_1Ev"]
    pub(super) fn local_func__act_a_catch_atk_1(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase42private___A_CATCH_ATK__function_after_waitEv"]
    pub(super) fn private___A_CATCH_ATK__function_after_wait(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase34private___A_CATCH_ATK__phase_rightEv"]
    pub(super) fn private___A_CATCH_ATK__phase_right(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase33private___A_CATCH_ATK__phase_leftEv"]
    pub(super) fn private___A_CATCH_ATK__phase_left(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase35private___A_CATCH_ATK__phase_bottomEv"]
    pub(super) fn private___A_CATCH_ATK__phase_bottom(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase31private___A_CATCH_ATK__phase_upEv"]
    pub(super) fn private___A_CATCH_ATK__phase_up(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase33private___A_CATCH_ATK__phase_backEv"]
    pub(super) fn private___A_CATCH_ATK__phase_back(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase34private___A_CATCH_ATK__phase_frontEv"]
    pub(super) fn private___A_CATCH_ATK__phase_front(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase34private___A_CATCH_ATK__phase_throwEv"]
    pub(super) fn private___A_CATCH_ATK__phase_throw(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase48private___A_CATCH_ATK__function_get_padding_waitEv"]
    pub(super) fn private___A_CATCH_ATK__function_get_padding_wait(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase35private___A_CATCH_ATK__phase_attackEv"]
    pub(super) fn private___A_CATCH_ATK__phase_attack(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase33private___A_CATCH_ATK__phase_waitEv"]
    pub(super) fn private___A_CATCH_ATK__phase_wait(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase27private___A_CATCH_ATK__initEv"]
    pub(super) fn private___A_CATCH_ATK__init(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase34main_func__action_global_variablesEv"]
    pub(super) fn main_func__action_global_variables(this: *mut L2CFighterAIActionBase);

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase35main_func__action_private_variablesEv"]
    pub(super) fn main_func__action_private_variables(this: *mut L2CFighterAIActionBase);

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase37main_func__function_r_jump_exit_checkEv"]
    pub(super) fn main_func__function_r_jump_exit_check(this: *mut L2CFighterAIActionBase);

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase37main_func__function_r_jump_sp_u_checkEv"]
    pub(super) fn main_func__function_r_jump_sp_u_check(this: *mut L2CFighterAIActionBase);

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase39main_func__function_r_jump_timing_checkEv"]
    pub(super) fn main_func__function_r_jump_timing_check(this: *mut L2CFighterAIActionBase);

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase28main_func__function_set_goalEv"]
    pub(super) fn main_func__function_set_goal(this: *mut L2CFighterAIActionBase);

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase24main_func__function_sp_uEv"]
    pub(super) fn main_func__function_sp_u(this: *mut L2CFighterAIActionBase);

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase30main_func__function_sp_u_checkEv"]
    pub(super) fn main_func__function_sp_u_check(this: *mut L2CFighterAIActionBase);

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase23main_func__phase_r_jumpEv"]
    pub(super) fn main_func__phase_r_jump(this: *mut L2CFighterAIActionBase);

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase25main_func__phase_sp_afterEv"]
    pub(super) fn main_func__phase_sp_after(this: *mut L2CFighterAIActionBase);

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase5ENTRYEv"]
    pub(super) fn ENTRY(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase10init_linesEv"]
    pub(super) fn init_lines(this: *mut L2CFighterAIActionBase);

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase21init_global_variablesEv"]
    pub(super) fn init_global_variables(this: *mut L2CFighterAIActionBase);

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase30function_phase_init_for_groundEv"]
    pub(super) fn function_phase_init_for_ground(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase40function_phase_init_wait_for_dash_cancelEv"]
    pub(super) fn function_phase_init_wait_for_dash_cancel(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase27function_phase_init_for_airEv"]
    pub(super) fn function_phase_init_for_air(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp22L2CFighterAIActionBase29private___A_FINAL__phase_initEv"]
    pub(super) fn private___A_FINAL__phase_init(this: *mut L2CFighterAIActionBase) -> lib::L2CValueHack;
}