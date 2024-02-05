use super::*;
use globals::*;
use smashline::*;



// FIGHTER_STATUS_KIND_FINAL //


pub unsafe extern "C" fn pre_final(fighter: &mut L2CFighterCommon) -> L2CValue {
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
    MeterModule::drain(fighter.object(), 6);
    return 0.into();
}

// FIGHTER_RYU_STATUS_KIND_FINAL2 //


pub unsafe extern "C" fn pre_final2(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Pre, fighter, *FIGHTER_RYU_STATUS_KIND_FINAL2)(fighter);
    MeterModule::drain(fighter.object(), 10);
    ret
}
pub fn install() {
    smashline::Agent::new("ken")
        .status(Pre, *FIGHTER_STATUS_KIND_FINAL, pre_final)
        .status(Pre, *FIGHTER_RYU_STATUS_KIND_FINAL2, pre_final2)
        .install();
}
