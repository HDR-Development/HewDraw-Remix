use super::*;
use globals::*;

// FIGHTER_WARIO_STATUS_KIND_SPECIAL_HI_JUMP


pub unsafe extern "C" fn special_hi_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_hi_jump"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.main_shift(special_hi_jump_main_loop)
}

unsafe extern "C" fn special_hi_jump_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }
    if !CancelModule::is_enable_cancel(fighter.module_accessor)
    || (!fighter.sub_wait_ground_check_common(L2CValue::Bool(false)).get_bool() && !fighter.sub_air_check_fall_common().get_bool()) {
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND
        && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_WARIO_STATUS_SPECIAL_HI_FLAG_DISABLE_MOTION_ANGLE) {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            return 0.into();
        }
        else {
            if MotionModule::is_end(fighter.module_accessor) {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                return 0.into();
            }
        }
        return 0.into();
    }
    1.into()
}


pub unsafe extern "C" fn fall_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND] == FIGHTER_WARIO_STATUS_KIND_SPECIAL_HI_JUMP {
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_SPECIAL);
        return 1.into();
    }
    fighter.status_pre_Fall()
}


pub fn install() {
    smashline::Agent::new("wario")
        .status(
            Main,
            *FIGHTER_WARIO_STATUS_KIND_SPECIAL_HI_JUMP,
            special_hi_jump_main,
        )
        .status(Pre, *FIGHTER_STATUS_KIND_FALL, fall_pre)
        .install();
}
