use super::*;

unsafe extern "C" fn special_hi_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    // ArticleModule::set_flag(fighter.module_accessor, *FIGHTER_PIKMIN_GENERATE_ARTICLE_PIKMIN, true, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLAG_AUTONOMY);
    ArticleModule::set_flag(fighter.module_accessor, *FIGHTER_PIKMIN_GENERATE_ARTICLE_PIKMIN, false, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_HI_DISABLE_AIR_FOLLOW);
    ArticleModule::set_int(fighter.module_accessor,*FIGHTER_PIKMIN_GENERATE_ARTICLE_PIKMIN, *WEAPON_PIKMIN_PIKMIN_OWNER_CONDITION_AIR, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_OWNER_CONDITION_CURRENT);
    return false.into();
}

unsafe extern "C" fn special_hi_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_MOTION_FALL,
        *GROUND_CORRECT_KIND_AIR as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_PIKMIN_SPECIAL_HI_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_PIKMIN_SPECIAL_HI_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_PIKMIN_SPECIAL_HI_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI) as u64,
        *FIGHTER_STATUS_ATTR_INTO_DOOR as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_hi_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::mul_speed(fighter.module_accessor, &Vector3f{x: 1.0, y: 0.5, z:  1.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
    smashline::original_status(Main, fighter, *FIGHTER_PIKMIN_STATUS_KIND_SPECIAL_HI_END)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_exec);
    agent.status(Exec, *FIGHTER_PIKMIN_STATUS_KIND_SPECIAL_HI_WAIT, special_hi_exec);

    agent.status(Pre, *FIGHTER_PIKMIN_STATUS_KIND_SPECIAL_HI_END, special_hi_end_pre);
    agent.status(Main, *FIGHTER_PIKMIN_STATUS_KIND_SPECIAL_HI_END, special_hi_end_main);
    agent.status(Exec, *FIGHTER_PIKMIN_STATUS_KIND_SPECIAL_HI_END, special_hi_exec);
}