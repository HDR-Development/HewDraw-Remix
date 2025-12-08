use super::*;
use globals::*;

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            fl_get_squat_walk_max_speed_hook,
            fightercommon_status_squatwait_main,
            fightercommon_status_squatrv_main,
        );
    }
}

// Note: Changes were made to the moonwalk check in status_dash_main_common at the same time as
//       the dashback out of squat modifications made here as to allow dashback when coming out
//       of crouch with a dash sometimes (depending on your y value when you started dash). It 
//       was a thing previously actually for dash forward too but I only noticied it when
//       looking into how to add dashback into HDR) - rei wolf 2025-12-07

// This runs for both SquatWait and SquatRv. SquatRv actually naturally allows you to cancel it with
// dashback, but the window for it is limited to the latter portion. This covers dashback both while
// squatting and getting up from it while utilizing the same thresholds normal dashback checks that
// are defined in dash.rs.
pub unsafe fn dashback_in_squat_check(fighter: &mut L2CFighterCommon) -> bool {
    // Stick positions
    let stick_x: f32 = fighter.global_table[STICK_X].get_f32();
    let stick_y: f32 = fighter.global_table[STICK_Y].get_f32();
    let flick_x: i32 = fighter.global_table[FLICK_X].get_i32();
    // Returns -1 or 1, to align the backdash check against the input
    let left_right_modifier: f32 = PostureModule::lr(fighter.module_accessor);
    // Parameters from common.xml
    let dashback_x_threshold: f32 = ParamModule::get_float(fighter.object(), ParamType::Common, "dashback_stick_x");
    let disable_y_threshold: f32 = ParamModule::get_float(fighter.object(), ParamType::Common, "squat_disable_dashback_crouch_threshold_stick_y");
    let flick_x_threshold: i32 = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("dash_flick_x"));

    // Perform checks on input conditions
    let is_smash_back_input: bool = (stick_x * left_right_modifier <= dashback_x_threshold) && (flick_x <= flick_x_threshold);
    let is_y_position_invalid: bool = stick_y <= disable_y_threshold;

    let conditions: bool = is_smash_back_input && !is_y_position_invalid;

    /*
     * useful for debugging/testing
     */

    // println!("({}): stick_x: {}, stick_y: {}, flick_x: {}, left_right_modifier: {}, dashback_x_threshold: {}, disable_y_threshold: {}, flick_x_threshold: {}",
    //     conditions,
    //     stick_x,
    //     stick_y,
    //     flick_x,
    //     left_right_modifier,
    //     dashback_x_threshold,
    //     disable_y_threshold,
    //     flick_x_threshold,
    // );
    //
    // println!("is_smash_back_input: .......... {} --> (stick_x * left_right_modifier <= dashback_x_threshold) && (flick_x <= flick_x_threshold)", is_smash_back_input);
    // println!("is_y_position_invalid: ........ {} --> stick_y <= disable_y_threshold", is_y_position_invalid);

    conditions
}

// Skyline hooks

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_SquatRv_Main)]
pub unsafe fn fightercommon_status_squatrv_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if (dashback_in_squat_check(fighter)) {
        VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_SMASH_TURN);
        interrupt!(fighter, FIGHTER_STATUS_KIND_TURN, true);
    }
    call_original!(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_SquatWait_Main)]
pub unsafe fn fightercommon_status_squatwait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if (dashback_in_squat_check(fighter)) {
        VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_SMASH_TURN);
        interrupt!(fighter, FIGHTER_STATUS_KIND_TURN, true);
    }
    call_original!(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_FL_get_squat_walk_max_speed)]
pub unsafe fn fl_get_squat_walk_max_speed_hook(fighter: &mut L2CFighterCommon) -> L2CValue {
    let squat_walk_speed_max_mul = fighter.get_param_float("squat_walk_speed_max_mul", "");
    let squat_walk_speed_max = if fighter.is_status(*FIGHTER_STATUS_KIND_SQUAT_B) {
        fighter.get_float(*FIGHTER_INSTANCE_WORK_ID_FLOAT_SQUAT_WALK_SPEED_BACK_MAX)
    } else if fighter.is_status(*FIGHTER_STATUS_KIND_SQUAT_F) {
        fighter.get_float(*FIGHTER_INSTANCE_WORK_ID_FLOAT_SQUAT_WALK_SPEED_FORWARD_MAX)
    } else {
        0.0
    };

    let unique_speed_max_mul = if fighter.kind() == *FIGHTER_KIND_KOOPA { 0.75 } else { 1.0 };
    let speed_max = squat_walk_speed_max * squat_walk_speed_max * unique_speed_max_mul;
    return speed_max.into();
}