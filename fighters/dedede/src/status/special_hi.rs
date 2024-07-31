use super::*;

// FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_HI_FAILURE

unsafe extern "C" fn special_hi_failure_pre(fighter: &mut L2CFighterCommon) -> L2CValue{
    if !VarModule::is_flag(fighter.battle_object, vars::dedede::instance::JET_GROUND_BONK){
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_SPECIAL);
        return 1.into();
    }
    smashline::original_status(Pre, fighter, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_HI_FAILURE)(fighter)

}

pub fn install(agent: &mut Agent){
    agent.status(Pre, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_HI_FAILURE, special_hi_failure_pre);
}