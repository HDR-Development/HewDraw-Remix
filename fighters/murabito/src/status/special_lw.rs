use super::*;

unsafe extern "C" fn special_lw_plant_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_GROUND_STOP,
        *GROUND_CORRECT_KIND_GROUND as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false,
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
        0,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_lw_plant_fail_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        0,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_lw_plant_fail_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    if situation == *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_plant_failure"), 0.0, 1.0, false, 0.0, false, false);
    } else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_plant_failure"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.main_shift(special_lw_plant_fail_main_loop)
}

unsafe extern "C" fn special_lw_plant_fail_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return true.into();
        }
    }

    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        let next_status = if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND {
            FIGHTER_STATUS_KIND_FALL
        } else if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING) {
            FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR
        } else {
            FIGHTER_STATUS_KIND_LANDING
        };
        fighter.change_status(next_status.into(), false.into());
        return true.into();
    }

    if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND {
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            SET_SPEED_EX(fighter, 0, 1.7, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        } else if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
            SET_SPEED_EX(fighter, 0, 1.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        && fighter.is_button_off(Buttons::SpecialAll) {
            MotionModule::set_rate(fighter.module_accessor, 2.0);
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        let next_status = if fighter.global_table[SITUATION_KIND] != *SITUATION_KIND_GROUND {
            FIGHTER_STATUS_KIND_FALL
        } else {
            FIGHTER_STATUS_KIND_WAIT
        };
        fighter.change_status(next_status.into(), false.into());
        return false.into();
    }

    return false.into();
}

unsafe extern "C" fn special_lw_water_air_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_MURABITO_SPECIAL_LW_WATER_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_MURABITO_SPECIAL_LW_WATER_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_MURABITO_SPECIAL_LW_WATER_FLOAT,
        (*FS_SUCCEEDS_KEEP_VISIBILITY | *FS_SUCCEEDS_KEEP_EFFECT) as i32
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_ENABLE,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_SHOOT | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_PLANT, special_lw_plant_pre);
    agent.status(Pre, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_PLANT_FAIL, special_lw_plant_fail_pre);
    agent.status(Main, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_PLANT_FAIL, special_lw_plant_fail_main);
    agent.status(Pre, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_LW_WATER_AIR, special_lw_water_air_pre);
}