use super::*;
use super::helper::*;

unsafe extern "C" fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_shift_status_main(L2CValue::Ptr(special_lw_main_loop as *const () as _))
}

unsafe extern "C" fn special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let sit = fighter.global_table[SITUATION_KIND].get_i32();
    if StatusModule::is_changing(fighter.module_accessor) || StatusModule::is_situation_changed(fighter.module_accessor) {
        special_motion_helper(
            fighter,
            hash40("special_lw").into(),
            hash40("special_air_lw").into(),
            FIGHTER_KINETIC_TYPE_GROUND_STOP.into(),
            FIGHTER_KINETIC_TYPE_FALL.into(),
            FIGHTER_ROCKMAN_STATUS_SPECIAL_LW_WORK_ID_FLAG_FIRST.into(),
            GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK.into()
        );
        if sit == *SITUATION_KIND_GROUND {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_CANCEL);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ATTACK_HI3_LANDING);
        }
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            fighter.change_status(FIGHTER_ROCKMAN_STATUS_KIND_SPECIAL_LW_SHOOT.into(), false.into());
        }
        else {
            let status = if sit == *SITUATION_KIND_GROUND {
                FIGHTER_STATUS_KIND_WAIT
            }
            else {
                FIGHTER_STATUS_KIND_FALL
            };
            fighter.change_status(status.into(), false.into());
        }
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(
            Main,
            *FIGHTER_STATUS_KIND_SPECIAL_LW,
            special_lw_main,
        );
}
