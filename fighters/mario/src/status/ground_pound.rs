use super::*;

unsafe extern "C" fn mario_ground_pound_start_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
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
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        (*FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u32,
        0
    );
    return false.into();
}

unsafe extern "C" fn mario_ground_pound_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // VarModule::on_flag(fighter.battle_object, vars::mario::instance::SPECIAL_LW_DISABLE);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_air_lw_start"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(mario_ground_pound_start_main_loop as *const () as _))
}

unsafe extern "C" fn mario_ground_pound_start_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(statuses::mario::GROUND_POUND_FALL.into(), true.into());
    }
    return false.into();
}

unsafe extern "C" fn mario_ground_pound_start_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return false.into();
}

unsafe extern "C" fn mario_ground_pound_fall_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_FALL,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    return false.into();
}

unsafe extern "C" fn mario_ground_pound_fall_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_air_lw_fall"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(mario_ground_pound_fall_main_loop as *const () as _))
}

unsafe extern "C" fn mario_ground_pound_fall_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return true.into();
    }
    let duration = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "ground_pound.fall_duration");
    if fighter.global_table[CURRENT_FRAME].get_i32() >= duration
    || fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        fighter.change_status(statuses::mario::GROUND_POUND_END.into(), false.into());
    }
    return false.into();
}

unsafe extern "C" fn mario_ground_pound_fall_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    let gravity = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    lua_bind::FighterKineticEnergyGravity::set_speed(
        gravity as *mut smash::app::FighterKineticEnergyGravity,
        -ParamModule::get_float(fighter.battle_object, ParamType::Agent, "ground_pound.fall_speed")
    );
    lua_bind::FighterKineticEnergyGravity::set_accel(gravity as *mut smash::app::FighterKineticEnergyGravity, 0.0);
    return false.into();
}

unsafe extern "C" fn mario_ground_pound_fall_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return false.into();
}

unsafe extern "C" fn mario_ground_pound_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    return false.into();
}

unsafe extern "C" fn mario_ground_pound_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_air_lw_landing"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    } else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion( 
            fighter.module_accessor,
            Hash40::new("special_air_lw_cancel"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        // TODO: replace lol
        SET_SPEED_EX(fighter, 0.0, -1.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(mario_ground_pound_end_main_loop as *const () as _))
}

unsafe extern "C" fn mario_ground_pound_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
                fighter.set_float(10.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            } else {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
            return false.into();
        }
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND
        && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return false.into();
        }
        if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND
        && fighter.sub_air_check_fall_common().get_bool() {
            return false.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        } else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return false.into();
    }
    return false.into();
}

unsafe extern "C" fn mario_ground_pound_end_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return false.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, statuses::mario::GROUND_POUND_START, mario_ground_pound_start_pre);
    agent.status(Main, statuses::mario::GROUND_POUND_START, mario_ground_pound_start_main);
    agent.status(End, statuses::mario::GROUND_POUND_START, mario_ground_pound_start_end);

    agent.status(Pre, statuses::mario::GROUND_POUND_FALL, mario_ground_pound_fall_pre);
    agent.status(Main, statuses::mario::GROUND_POUND_FALL, mario_ground_pound_fall_main);
    agent.status(Exec, statuses::mario::GROUND_POUND_FALL, mario_ground_pound_fall_exec);
    agent.status(End, statuses::mario::GROUND_POUND_FALL, mario_ground_pound_fall_end);

    agent.status(Pre, statuses::mario::GROUND_POUND_END, mario_ground_pound_end_pre);
    agent.status(Main, statuses::mario::GROUND_POUND_END, mario_ground_pound_end_main);
    agent.status(End, statuses::mario::GROUND_POUND_END, mario_ground_pound_end_end);
}