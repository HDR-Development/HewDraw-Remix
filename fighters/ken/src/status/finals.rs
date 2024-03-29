use super::*;

// FIGHTER_STATUS_KIND_FINAL

pub unsafe extern "C" fn final_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_FinalCommon();
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_FINAL | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        (*FIGHTER_STATUS_ATTR_DISABLE_ITEM_INTERRUPT | *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE | *FIGHTER_STATUS_ATTR_FINAL) as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_FINAL as u32,
        0
    );
    let meter_amount = MeterModule::meter(fighter.battle_object);
    MeterModule::drain_direct(fighter.battle_object, meter_amount);
    return 0.into();
}

// FIGHTER_RYU_STATUS_KIND_FINAL2

pub unsafe extern "C" fn final2_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Pre, fighter, *FIGHTER_RYU_STATUS_KIND_FINAL2)(fighter);
    let meter_amount = MeterModule::meter(fighter.battle_object);
    MeterModule::drain_direct(fighter.battle_object, meter_amount);
    ret
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_FINAL, final_pre);
    agent.status(Pre, *FIGHTER_RYU_STATUS_KIND_FINAL2, final2_pre);
}
