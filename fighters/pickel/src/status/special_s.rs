use super::*;
 
// FIGHTER_STATUS_KIND_SPECIAL_S

pub unsafe extern "C" fn special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue{
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        (*FIGHTER_STATUS_ATTR_DISABLE_INTERRUPT_WARP | *FIGHTER_STATUS_ATTR_START_TURN) as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    
    return 0.into();
}

pub unsafe extern "C" fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue{
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_RESET);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
     }
    fighter.sub_change_motion_by_situation(
        Hash40::new("special_air_s_start").into(), 
        Hash40::new("special_air_s_start").into(), 
        false.into()
    );

    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_main_loop as *const () as _))
}

pub unsafe extern "C" fn special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue{
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        } else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }

        return 1.into();
    }

    if StatusModule::is_situation_changed(fighter.module_accessor)
    && !StatusModule::is_changing(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s_start"), -1.0, 1.0, 0.0, false, false);
        } else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1.into();
        }
    }

    return 0.into();
}

pub unsafe extern "C" fn special_s_end(fighter: &mut L2CFighterCommon) -> L2CValue{
    return 0.into();
}

// FIGHTER_PICKEL_STATUS_KIND_SPECIAL_S_FAILED

pub unsafe extern "C" fn special_s_failed_pre(fighter: &mut L2CFighterCommon) -> L2CValue{
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_ATTACK_DASH | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_DASH as u32,
        0
    );
    
    return 0.into();
}

pub unsafe extern "C" fn special_s_failed_main(fighter: &mut L2CFighterCommon) -> L2CValue{
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
     }
    fighter.sub_change_motion_by_situation(
        Hash40::new("special_s_failed").into(), 
        Hash40::new("special_s_failed").into(), 
        false.into()
    );

    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_failed_main_loop as *const () as _))
}

pub unsafe extern "C" fn special_s_failed_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue{
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.change_status(FIGHTER_STATUS_KIND_DOWN_WAIT.into(), false.into());
        } else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }

        return 1.into();
    }

    return 0.into();
}


pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_main);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_end);

    agent.status(Pre, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_S_FAILED, special_s_failed_pre);
    agent.status(Main, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_S_FAILED, special_s_failed_main);
}