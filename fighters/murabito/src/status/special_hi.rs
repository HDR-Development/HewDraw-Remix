use super::*;

unsafe extern "C" fn special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_MURABITO_SPECIAL_HI,
        *GROUND_CORRECT_KIND_AIR as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_MURABITO_SPECIAL_HI_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_MURABITO_SPECIAL_HI_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_MURABITO_SPECIAL_HI_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI as u64,
        *FIGHTER_STATUS_ATTR_INTO_DOOR as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_WAIT, special_hi_pre);
    agent.status(Pre, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_TURN, special_hi_pre);
    agent.status(Pre, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_FLAP, special_hi_pre);
}