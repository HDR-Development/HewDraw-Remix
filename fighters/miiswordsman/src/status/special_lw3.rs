use super::*;

pub unsafe extern "C" fn special_lw3_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_FALL,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
        ) as u64,
        (
            *FIGHTER_STATUS_ATTR_DISABLE_GROUND_FRICTION |
            *FIGHTER_STATUS_ATTR_START_TURN
        ) as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn special_lw3_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_INT_JET_STUB_START_SITUATION) != *SITUATION_KIND_GROUND {
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_INT_JET_STUB_START_SITUATION) == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw3_end_air"), 0.0, 1.0, false, 0.0, false, false);
            if StopModule::is_stop(fighter.module_accessor) {
                special_lw3_end_substatus(fighter, false.into());
            }
            fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(special_lw3_end_substatus as *const () as _));
            fighter.main_shift(special_lw3_end_main_loop_air)
        }
        else {
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                fighter.set_situation(SITUATION_KIND_GROUND.into());
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                let rate = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("lw3_end_air_to_ground_rate"));
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw3_end"), 0.0, rate, false, 0.0, false, false);
                fighter.main_shift(special_lw3_end_main_loop_ground)
            }
            else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw3_end_air"), 0.0, 1.0, false, 0.0, false, false);
                WorkModule::enable_transition_term(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_JET_STUB_AA_TRANSITION_TERM_ID_INHERIT_FALL);
                WorkModule::unable_transition_term(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_JET_STUB_AA_TRANSITION_TERM_ID_LANDING_WAIT);
                WorkModule::unable_transition_term(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_JET_STUB_AA_TRANSITION_TERM_ID_LANDING_FALL);
                fighter.main_shift(special_lw3_end_main_loop)
            }
        }
    }
    else {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            let rate = WorkModule::get_param_float(fighter.module_accessor,hash40("param_special_lw"),hash40("lw3_end_ground_to_ground_rate"));
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw3_end"), 0.0, rate, false, 0.0, false, false);
            fighter.main_shift(special_lw3_end_main_loop_ground)
        }
        else {
            // ending grounded thrust in the air
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            KineticModule::add_speed(fighter.module_accessor, &Vector3f::new(0.0, 0.5, 0.0));
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw3_end_air"), 0.0, 1.0, false, 0.0, false, false);
            fighter.main_shift(special_lw3_end_main_loop_air)
        }
    }
}

unsafe extern "C" fn special_lw3_end_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    if !param_1.get_bool() {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_FLAG_JET_STUB_FALL_ONOFF) {
            KineticModule::change_kinetic(fighter.module_accessor,*FIGHTER_KINETIC_TYPE_FALL);
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor,*FIGHTER_KINETIC_TYPE_MOTION_AIR);
        }
    }
}

unsafe extern "C" fn special_lw3_end_main_loop_ground(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !CancelModule::is_enable_cancel(fighter.module_accessor) || (CancelModule::is_enable_cancel(fighter.module_accessor) && !fighter.sub_wait_ground_check_common(L2CValue::Bool(false)).get_bool() && !fighter.sub_air_check_fall_common().get_bool()) {
        if !fighter.is_situation(*SITUATION_KIND_AIR) {
            if MotionModule::is_end(fighter.module_accessor) {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
                return 0.into();
            }
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 0.into();
        }
    }
    
    return 1.into();
}

unsafe extern "C" fn special_lw3_end_main_loop_air(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_INT_JET_STUB_START_SITUATION) == *SITUATION_KIND_GROUND {
        if fighter.status_frame() >= 20 {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
    }

    return 0.into();
}

unsafe extern "C" fn special_lw3_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_JET_STUB_AA_TRANSITION_TERM_ID_INHERIT_FALL) {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 0.into();
        }
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_FLAG_JET_STUB_LANDING_FALL_SPECIAL) {
                let landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("lw3_landing_frame"));
                WorkModule::set_float(fighter.module_accessor,landing_frame as f32, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
                fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
                return 1.into();
            }
            fighter.set_situation(SITUATION_KIND_GROUND.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw3_end"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::unable_transition_term(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_JET_STUB_AA_TRANSITION_TERM_ID_INHERIT_FALL);
            WorkModule::enable_transition_term(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_JET_STUB_AA_TRANSITION_TERM_ID_LANDING_WAIT);
            WorkModule::enable_transition_term(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_JET_STUB_AA_TRANSITION_TERM_ID_LANDING_FALL);
            return 0.into();
        }
    }
    else {
        if MotionModule::is_end(fighter.module_accessor) {
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
                return 0.into();
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                return 0.into();
            }
        }
    }

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_LW3_END, special_lw3_end_main);
}