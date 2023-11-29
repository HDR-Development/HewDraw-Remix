use super::*;
use globals::*;


// FIGHTER_STATUS_KIND_SPECIAL_S //

#[status_script(agent = "diddy", status = FIGHTER_DIDDY_STATUS_KIND_SPECIAL_S_JUMP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_s_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let start_speed_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("monkey_flip_jump_start_spd_y"));
    let lr = PostureModule::lr(fighter.module_accessor);
    let start_speed_x = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DIDDY_STATUS_MONKEY_FLIP_FLAG_SMASH) {
        WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("monkey_flip_smash_jump_start_spd_x"))
    }
    else {
        WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("monkey_flip_jump_start_spd_x"))
    };
    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
        0.0,
        start_speed_y,
        0.0,
        0.0,
        0.0
    );
    sv_kinetic_energy!(
        enable,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY
    );
    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        ENERGY_STOP_RESET_TYPE_AIR,
        start_speed_x * lr,
        0.0,
        0.0,
        0.0,
        0.0
    );
    sv_kinetic_energy!(
        enable,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP
    );
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_air_s_jump"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    if !StopModule::is_stop(fighter.module_accessor) {
        special_s_jump_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(special_s_jump_substatus as *const () as _));
    fighter.main_shift(special_s_jump_main_loop)
}

unsafe extern "C" fn special_s_jump_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_DIDDY_STATUS_MONKEY_FLIP_WORK_INT_KICK_FRAME);
    }
    0.into()
}

unsafe extern "C" fn special_s_jump_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }

    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_DIDDY_STATUS_KIND_SPECIAL_S_FLIP_LANDING.into(), false.into());
        return 0.into();
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }

    let current_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_DIDDY_STATUS_MONKEY_FLIP_WORK_INT_KICK_FRAME);
    let kick_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("monkey_flip_kick_frame"));
    if kick_frame <= current_frame
    && fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER != 0 { // Normally also checks *FIGHTER_PAD_FLAG_ATTACK_TRIGGER
        fighter.change_status(FIGHTER_DIDDY_STATUS_KIND_SPECIAL_S_KICK.into(), true.into());
        return 0.into();
    }

    0.into()
}

pub fn install() {
    install_status_scripts!(
        special_s_jump_main
    );
}