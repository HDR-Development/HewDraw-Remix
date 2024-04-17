use super::*;

// FIGHTER_STATUS_KIND_WAIT

pub unsafe extern "C" fn wait_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_Wait()
}

// vanilla script

pub unsafe extern "C" fn wait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_wait_common();
    fighter.sub_wait_motion_mtrans();
    fighter.sub_shift_status_main(L2CValue::Ptr(fgc_wait_main_loop as *const () as _))
}

pub unsafe extern "C" fn fgc_wait_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.status_Wait_Main().get_bool() {
        return 0.into();
    }
    let lr = WorkModule::get_float(
        fighter.module_accessor,
        *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1,
    );
    if lr != 0.0 && PostureModule::lr(fighter.module_accessor) != lr {
        let stick_x_corrected = fighter.global_table[STICK_X].get_f32()
            * (PostureModule::lr(fighter.module_accessor) * -1.0);
        let stick_y = fighter.global_table[STICK_Y].get_f32();
        let walk_stick_x = WorkModule::get_param_float(
            fighter.module_accessor,
            hash40("common"),
            hash40("walk_stick_x"),
        );
        let squat_stick_y = WorkModule::get_param_float(
            fighter.module_accessor,
            hash40("common"),
            hash40("squat_stick_y"),
        );

        if WorkModule::is_enable_transition_term(
            fighter.module_accessor,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK,
        ) {
            if walk_stick_x <= stick_x_corrected {
                if squat_stick_y < stick_y {
                    fighter.change_status(FIGHTER_RYU_STATUS_KIND_WALK_BACK.into(), true.into());
                    return 0.into();
                }
            }
        }
        fighter.change_status(FIGHTER_RYU_STATUS_KIND_TURN_AUTO.into(), false.into());
        return 0.into();
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_WAIT, wait_pre);
    //agent.status(Main, *FIGHTER_STATUS_KIND_WAIT, wait_main);
}