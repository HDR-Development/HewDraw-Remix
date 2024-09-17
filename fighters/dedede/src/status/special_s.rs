use super::*;

unsafe extern "C" fn special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_flag(*FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLAG_GORDO_GET) {
        if ArticleModule::is_generatable(fighter.module_accessor, *FIGHTER_DEDEDE_GENERATE_ARTICLE_GORDO) {
            StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_S_MISS);
            return 1.into();
        }
    }
    let keep_flag = if fighter.global_table[0x10].get_i32() == *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_S_GET {
        *FIGHTER_STATUS_WORK_KEEP_FLAG_DEDEDE_SPECIAL_S_FLAG
    } else { 0 };
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        keep_flag,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_DEDEDE_SPECIAL_S_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_DEDEDE_SPECIAL_S_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON |
            *FIGHTER_LOG_MASK_FLAG_SHOOT) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_pre);
}