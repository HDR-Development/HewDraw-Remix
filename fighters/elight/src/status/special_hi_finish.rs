use super::*;

unsafe extern "C" fn special_hi_finish_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !VarModule::is_flag(fighter.battle_object, vars::elight::status::SPECIAL_HI_FREEFALL) {
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, statuses::elight::SPECIAL_HI_FINISH2);
        return 1.into();
    }
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_MOTION_AIR,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
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
        (
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK
        ) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_HI_FINISH, special_hi_finish_pre);
}