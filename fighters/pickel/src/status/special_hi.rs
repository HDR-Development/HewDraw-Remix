use super::*;

pub unsafe extern "C" fn special_hi_gliding_pre(fighter: &mut L2CFighterCommon) -> L2CValue{
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_GLIDE_START,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_PICKEL_STATUS_WORK_KEEP_FLAG_SPECIAL_HI_FLAG,
        *FIGHTER_PICKEL_STATUS_WORK_KEEP_FLAG_SPECIAL_HI_INT,
        *FIGHTER_PICKEL_STATUS_WORK_KEEP_FLAG_SPECIAL_HI_FLOAT,
        *FS_SUCCEEDS_KEEP_VISIBILITY as i32
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_HI_GLIDING, special_hi_gliding_pre);
}