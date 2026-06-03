use super::*;


// FIGHTER_STATUS_KIND_SPECIAL_HI

unsafe extern "C" fn special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return 0.into();
}

// FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_CHARGE

unsafe extern "C" fn special_hi_charge_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return 0.into();
}

// FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_CHARGE_DAMAGE

pub unsafe extern "C" fn special_hi_charge_damage_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_CHARGE_DAMAGE)(fighter);
    MotionModule::set_rate(fighter.module_accessor, 1.0);
    ret
}

// FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_UPPER_DAMAGE

pub unsafe extern "C" fn special_hi_upper_damage_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_UPPER_DAMAGE)(fighter);
    MotionModule::set_rate(fighter.module_accessor, 1.0);
    ret
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_pre);
    agent.status(Pre, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_CHARGE, special_hi_charge_pre);
    agent.status(Main, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_CHARGE_DAMAGE, special_hi_charge_damage_main);
    agent.status(Main, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_UPPER_DAMAGE, special_hi_upper_damage_main);
}