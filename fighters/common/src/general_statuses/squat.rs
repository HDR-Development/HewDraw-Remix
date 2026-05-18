use super::*;
use globals::*;

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            fl_get_squat_walk_max_speed_hook,
            status_Squat_Main,
            status_end_Squat,
            status_SquatWait_Main,
            status_end_SquatWait,
            status_squatrv_main
        );
    }
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
    let speed_max = squat_walk_speed_max * squat_walk_speed_max_mul * unique_speed_max_mul;
    return speed_max.into();
}


const RIVALS_CC_DAMAGE_ARMOR: f32 = 6.0;

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_Squat_Main)]
pub unsafe fn status_Squat_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // reworked CC armor in RoA mode
    if utils::game_modes::check_custom_mode(game_modes::CustomMode::RivalsOfAetherMode) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, RIVALS_CC_DAMAGE_ARMOR);
    }
    original!()(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_Squat)]
pub unsafe fn status_end_Squat(fighter: &mut L2CFighterCommon) -> L2CValue {
    // reworked CC armor in RoA mode
    if utils::game_modes::check_custom_mode(game_modes::CustomMode::RivalsOfAetherMode) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    original!()(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_SquatWait_Main)]
pub unsafe fn status_SquatWait_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // reworked CC armor in RoA mode
    if utils::game_modes::check_custom_mode(game_modes::CustomMode::RivalsOfAetherMode) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, RIVALS_CC_DAMAGE_ARMOR);
    }
    original!()(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_SquatWait)]
pub unsafe fn status_end_SquatWait(fighter: &mut L2CFighterCommon) -> L2CValue {
    // reworked CC armor in RoA mode
    if utils::game_modes::check_custom_mode(game_modes::CustomMode::RivalsOfAetherMode) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    original!()(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_SquatRv_Main)]
pub unsafe fn status_squatrv_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor) {
        // Dashback out of uncrouch
        let lr = PostureModule::lr(fighter.module_accessor);
        let dashback_stick_x = ParamModule::get_float(fighter.object(), ParamType::Common, "dashback_stick_x");
        let dash_flick_x = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("dash_flick_x"));
        let is_backdash_input =
            fighter.prev_stick_x() * lr > dashback_stick_x
            && fighter.global_table[STICK_X].get_f32() * lr <= dashback_stick_x
            && fighter.global_table[FLICK_X].get_i32() <= dash_flick_x;
        if is_backdash_input {
            VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_SMASH_TURN);
            interrupt!(fighter, FIGHTER_STATUS_KIND_TURN, true);
        }
    }

    call_original!(fighter)
}