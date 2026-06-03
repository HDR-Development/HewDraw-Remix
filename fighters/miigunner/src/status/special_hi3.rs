use super::*;

pub unsafe extern "C" fn special_hi3_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_MOTION_CLIFF_MOVE,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return 0.into();
}

pub unsafe extern "C" fn special_hi3_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

unsafe extern "C" fn special_hi3_rush_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.off_flag(*FIGHTER_MIIGUNNER_STATUS_ARM_ROCKET_RUSH_FLAG_CONTINUE);
    fighter.set_int(0, *FIGHTER_MIIGUNNER_STATUS_ARM_ROCKET_RUSH_INT_RUSH_FRAME);
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(sub_special_hi3_rush as *const () as _));
    fighter.set_int(*FIGHTER_STATUS_KIND_FALL_SPECIAL, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_STATUS_KIND_END);
    
    fighter.main_shift(special_hi3_rush_main_loop)
}

unsafe extern "C" fn special_hi3_rush_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_changing(fighter.module_accessor)
    || StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.off_flag(*FIGHTER_MIIGUNNER_STATUS_ARM_ROCKET_RUSH_FLAG_AIR);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_BRAKE);
            if fighter.is_flag(*FIGHTER_MIIGUNNER_STATUS_ARM_ROCKET_RUSH_FLAG_CONTINUE) {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_hi3"), -1.0, 1.0, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi3"), 0.0, 1.0, false, 0.0, false, false);
                fighter.on_flag(*FIGHTER_MIIGUNNER_STATUS_ARM_ROCKET_RUSH_FLAG_CONTINUE);
            }
            fighter.sub_fighter_cliff_check(L2CValue::I32(*GROUND_CLIFF_CHECK_KIND_NONE));
        }
        else {
            fighter.on_flag(*FIGHTER_MIIGUNNER_STATUS_ARM_ROCKET_RUSH_FLAG_AIR);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_BRAKE);
            if fighter.is_flag(*FIGHTER_MIIGUNNER_STATUS_ARM_ROCKET_RUSH_FLAG_CONTINUE) {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_hi3"), -1.0, 1.0, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi3"), 0.0, 1.0, false, 0.0, false, false);
                fighter.on_flag(*FIGHTER_MIIGUNNER_STATUS_ARM_ROCKET_RUSH_FLAG_CONTINUE);
            }
            fighter.sub_fighter_cliff_check(L2CValue::I32(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES));
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    fighter.check_wall_jump_cancel();
    let rot = fighter.get_float(*FIGHTER_MIIGUNNER_STATUS_ARM_ROCKET_RUSH_FLOAT_ROT_X);
    VarModule::set_float(fighter.battle_object, vars::miigunner::instance::SPECIAL_HI3_ROT, rot);
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_HI3_RUSH_END.into(), false.into());
    }

    return 0.into();
}

unsafe extern "C" fn sub_special_hi3_rush(fighter: &mut L2CFighterCommon, param: bool) -> L2CValue {
    if param {
        fighter.inc_int(*FIGHTER_MIIGUNNER_STATUS_ARM_ROCKET_RUSH_INT_RUSH_FRAME);
    }

    return 0.into();
}

unsafe extern "C" fn special_hi3_rush_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

unsafe extern "C" fn special_hi3_rush_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_AIR_STOP,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_MIIGUNNER_ARM_ROCKET_RUSH_END_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_MIIGUNNER_ARM_ROCKET_RUSH_END_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_MIIGUNNER_ARM_ROCKET_RUSH_END_FLOAT,
        *FS_SUCCEEDS_KEEP_ATTACK
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_hi3_rush_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi3_end"), 0.0, 1.0, false, 0.0, false, false);
    fighter.set_int(*FIGHTER_STATUS_KIND_FALL_SPECIAL, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_STATUS_KIND_END);

    fighter.main_shift(special_hi3_rush_end_main_loop)
}

unsafe extern "C" fn special_hi3_rush_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.motion_frame() <= 12.0 {
        fighter.check_wall_jump_cancel();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, *FIGHTER_STATUS_KIND_FALL_SPECIAL, false);
    }

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_HI3_RUSH, special_hi3_rush_main);
    agent.status(Exit, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_HI3_RUSH, special_hi3_rush_exit);
    
    agent.status(Pre, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_HI3_RUSH_END, special_hi3_rush_end_pre);
    agent.status(Main, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_HI3_RUSH_END, special_hi3_rush_end_main);
}