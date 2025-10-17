use super::*;

// statuses::kirby::PIKMIN_SPECIAL_N_FAILURE

unsafe extern "C" fn special_n_failure_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_SpecialNCommon();

    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn special_n_failure_main(fighter: &mut L2CFighterCommon) -> L2CValue {

    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("pikmin_special_n_failure"), 0.0, 1.0, false, 0.0, false, false);
    }
    else { // Aerial vewrsion now handled here
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("pikmin_special_air_n"), 0.0, 1.0, false, 0.0, false, false);
    }

    fighter.main_shift(special_n_failure_main_loop)
}

unsafe extern "C" fn special_n_failure_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    fighter.sub_air_check_dive();

    if fighter.is_situation(*SITUATION_KIND_GROUND) && fighter.is_prev_situation(*SITUATION_KIND_AIR) {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        return 0.into();
    }

    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    return 0.into()
}

unsafe extern "C" fn special_n_failure_end(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, statuses::kirby::PIKMIN_SPECIAL_N_FAILURE, special_n_failure_pre);
    agent.status(Main, statuses::kirby::PIKMIN_SPECIAL_N_FAILURE, special_n_failure_main);
    agent.status(End, statuses::kirby::PIKMIN_SPECIAL_N_FAILURE, special_n_failure_end);
}
