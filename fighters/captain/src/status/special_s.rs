use super::*;

unsafe extern "C" fn special_s_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let start_situation = WorkModule::get_int(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_KNUCKLE_START_SITUATION);
    if start_situation == *SITUATION_KIND_AIR {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_end"), 0.0, 1.0, false, 0.0, false, false);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_end"), 0.0, 1.0, false, 0.0, false, false);
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }

    fighter.main_shift(special_s_end_main_loop)
}

unsafe extern "C" fn special_s_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into()
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    let start_situation = WorkModule::get_int(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_KNUCKLE_START_SITUATION);
    if start_situation == *SITUATION_KIND_AIR {
        if MotionModule::is_end(fighter.module_accessor) {
            let status = if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) { FIGHTER_STATUS_KIND_FALL } else { FIGHTER_STATUS_KIND_FALL_SPECIAL };
            fighter.change_status(status.into(), false.into());
            return 1.into();
        }
        if StatusModule::is_situation_changed(fighter.module_accessor)
        && fighter.is_situation(*SITUATION_KIND_GROUND) {
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING) {
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
                return 1.into();
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
                return 1.into();
            }
        }
    }
    else {
        if MotionModule::is_end(fighter.module_accessor) {
            let status = if fighter.is_situation(*SITUATION_KIND_GROUND) { FIGHTER_STATUS_KIND_WAIT } else { FIGHTER_STATUS_KIND_FALL };
            fighter.change_status(status.into(), false.into());
            return 1.into();
        }
    }

    return 0.into();
}

unsafe extern "C" fn special_s_end_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    let start_situation = WorkModule::get_int(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_KNUCKLE_START_SITUATION);
    if start_situation == *SITUATION_KIND_AIR {
        let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("landing_frame"));
        WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    }

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_S_END, special_s_end_main);
    agent.status(Exit, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_S_END, special_s_end_exit);
}