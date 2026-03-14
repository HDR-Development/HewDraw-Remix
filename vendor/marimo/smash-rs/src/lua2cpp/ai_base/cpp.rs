use crate::*;
use super::class::*;

extern "C" {
    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase29main_func__constant_variablesEv"]
    pub(super) fn main_func__constant_variables(this: *mut L2CFighterAIBase);

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase20main_func__utility_1Ev"]
    pub(super) fn main_func__utility_1(this: *mut L2CFighterAIBase);

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase16vector__distanceEN3lib8L2CValueES2_S2_S2_S2_S2_"]
    pub(super) fn vector__distance(this: *mut L2CFighterAIBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack, arg6: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase23vector__distance_squareEN3lib8L2CValueES2_S2_S2_S2_S2_"]
    pub(super) fn vector__distance_square(this: *mut L2CFighterAIBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack, arg6: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase19vector__distance_2dEN3lib8L2CValueES2_S2_S2_"]
    pub(super) fn vector__distance_2d(this: *mut L2CFighterAIBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase26vector__distance_square_2dEN3lib8L2CValueES2_S2_S2_"]
    pub(super) fn vector__distance_square_2d(this: *mut L2CFighterAIBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase17vector__normalizeEN3lib8L2CValueES2_S2_"]
    pub(super) fn vector__normalize(this: *mut L2CFighterAIBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase21vector__length_squareEN3lib8L2CValueES2_S2_"]
    pub(super) fn vector__length_square(this: *mut L2CFighterAIBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase20vector__normalize_2dEN3lib8L2CValueES2_"]
    pub(super) fn vector__normalize_2d(this: *mut L2CFighterAIBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase24vector__length_square_2dEN3lib8L2CValueES2_"]
    pub(super) fn vector__length_square_2d(this: *mut L2CFighterAIBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase14vector__lengthEN3lib8L2CValueES2_S2_"]
    pub(super) fn vector__length(this: *mut L2CFighterAIBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase17vector__length_2dEN3lib8L2CValueES2_"]
    pub(super) fn vector__length_2d(this: *mut L2CFighterAIBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase13vector__crossEN3lib8L2CValueES2_S2_S2_S2_S2_"]
    pub(super) fn vector__cross(this: *mut L2CFighterAIBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack, arg6: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase16vector__cross_2dEN3lib8L2CValueES2_S2_S2_"]
    pub(super) fn vector__cross_2d(this: *mut L2CFighterAIBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase11vector__dotEN3lib8L2CValueES2_S2_S2_S2_S2_"]
    pub(super) fn vector__dot(this: *mut L2CFighterAIBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack, arg6: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase14vector__dot_2dEN3lib8L2CValueES2_S2_S2_"]
    pub(super) fn vector__dot_2d(this: *mut L2CFighterAIBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase32local_func__constant_variables_1Ev"]
    pub(super) fn local_func__constant_variables_1(this: *mut L2CFighterAIBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase5yieldEv"]
    pub(super) fn yield_(this: *mut L2CFighterAIBase);

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase4waitEN3lib8L2CValueE"]
    pub(super) fn wait(this: *mut L2CFighterAIBase, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase11wait_randomEN3lib8L2CValueES2_"]
    pub(super) fn wait_random(this: *mut L2CFighterAIBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase10wait_untilEN3lib8L2CValueE"]
    pub(super) fn wait_until(this: *mut L2CFighterAIBase, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase9add_stickEN3lib8L2CValueES2_S2_"]
    pub(super) fn add_stick(this: *mut L2CFighterAIBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase11reset_stickEN3lib8L2CValueE"]
    pub(super) fn reset_stick(this: *mut L2CFighterAIBase, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase13add_stick_absEN3lib8L2CValueES2_S2_"]
    pub(super) fn add_stick_abs(this: *mut L2CFighterAIBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase13add_stick_degEN3lib8L2CValueES2_"]
    pub(super) fn add_stick_deg(this: *mut L2CFighterAIBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase10add_buttonEN3lib8L2CValueES2_"]
    pub(super) fn add_button(this: *mut L2CFighterAIBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase25is_1on1_auto_turn_fighterEv"]
    pub(super) fn is_1on1_auto_turn_fighter(this: *mut L2CFighterAIBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase19is_status_kind_walkEv"]
    pub(super) fn is_status_kind_walk(this: *mut L2CFighterAIBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase25is_status_kind_dash_startEv"]
    pub(super) fn is_status_kind_dash_start(this: *mut L2CFighterAIBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase27is_status_kind_dash_f_startEv"]
    pub(super) fn is_status_kind_dash_f_start(this: *mut L2CFighterAIBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase27is_status_kind_dash_b_startEv"]
    pub(super) fn is_status_kind_dash_b_start(this: *mut L2CFighterAIBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase18is_status_kind_runEv"]
    pub(super) fn is_status_kind_run(this: *mut L2CFighterAIBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase26is_status_kind_escape_ableEv"]
    pub(super) fn is_status_kind_escape_able(this: *mut L2CFighterAIBase);

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase29is_status_kind_escape_lr_ableEv"]
    pub(super) fn is_status_kind_escape_lr_able(this: *mut L2CFighterAIBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase21is_absolutely_fallingEv"]
    pub(super) fn is_absolutely_falling(this: *mut L2CFighterAIBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase9rank_rateEN3lib8L2CValueES2_"]
    pub(super) fn rank_rate(this: *mut L2CFighterAIBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase20rank_rate_with_rangeEN3lib8L2CValueES2_S2_S2_"]
    pub(super) fn rank_rate_with_range(this: *mut L2CFighterAIBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase19cp_level_to_cp_rankEN3lib8L2CValueE"]
    pub(super) fn cp_level_to_cp_rank(this: *mut L2CFighterAIBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase22check_range_from_floorEN3lib8L2CValueES2_S2_S2_"]
    pub(super) fn check_range_from_floor(this: *mut L2CFighterAIBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase23local_func__utility_1_1Ev"]
    pub(super) fn local_func__utility_1_1(this: *mut L2CFighterAIBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase21is_target_waiting_youEv"]
    pub(super) fn is_target_waiting_you(this: *mut L2CFighterAIBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase17is_wire_availableEv"]
    pub(super) fn is_wire_available(this: *mut L2CFighterAIBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase23local_func__utility_1_2Ev"]
    pub(super) fn local_func__utility_1_2(this: *mut L2CFighterAIBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase23local_func__utility_1_3Ev"]
    pub(super) fn local_func__utility_1_3(this: *mut L2CFighterAIBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase23local_func__utility_1_4Ev"]
    pub(super) fn local_func__utility_1_4(this: *mut L2CFighterAIBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase23local_func__utility_1_5Ev"]
    pub(super) fn local_func__utility_1_5(this: *mut L2CFighterAIBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase14is_wire_returnEv"]
    pub(super) fn is_wire_return(this: *mut L2CFighterAIBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase22common_return_set_goalEN3lib8L2CValueE"]
    pub(super) fn common_return_set_goal(this: *mut L2CFighterAIBase, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase27is_interrupt_sp_u_availableEN3lib8L2CValueE"]
    pub(super) fn is_interrupt_sp_u_available(this: *mut L2CFighterAIBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase8is_equalEN3lib8L2CValueES2_"]
    pub(super) fn is_equal(this: *mut L2CFighterAIBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase7is_zeroEN3lib8L2CValueE"]
    pub(super) fn is_zero(this: *mut L2CFighterAIBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase4lerpEN3lib8L2CValueES2_S2_"]
    pub(super) fn lerp(this: *mut L2CFighterAIBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase4signEN3lib8L2CValueE"]
    pub(super) fn sign(this: *mut L2CFighterAIBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase5clampEN3lib8L2CValueES2_S2_"]
    pub(super) fn clamp(this: *mut L2CFighterAIBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp16L2CFighterAIBase13random_choiceEN3lib8L2CValueE"]
    pub(super) fn random_choice(this: *mut L2CFighterAIBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;
}