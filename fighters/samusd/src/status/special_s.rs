use super::*;

unsafe extern "C" fn special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.on_flag(*FIGHTER_SAMUS_INSTANCE_WORK_ID_FLAG_ST_INIT);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), FIGHTER_LOG_ACTION_CATEGORY_KEEP, *FIGHTER_LOG_ATTACK_KIND_SPECIAL_S);
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1G);
    }
    else {
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A);
    }
    
    return 1.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_pre);
}