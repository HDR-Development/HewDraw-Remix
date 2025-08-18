use super::*;

// FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_HI_FAILURE

unsafe extern "C" fn special_hi_failure_pre(fighter: &mut L2CFighterCommon) -> L2CValue{
    
    fighter.set_status_kind_interrupt(*FIGHTER_STATUS_KIND_FALL_SPECIAL);
    return 1.into();

}

pub fn install(agent: &mut Agent){
    agent.status(Pre, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_HI_FAILURE, special_hi_failure_pre);
}