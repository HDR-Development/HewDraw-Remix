use super::*;

// FIGHTER_STATUS_KIND_ATTACK

unsafe extern "C" fn attack_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_pre_Attack();
}

unsafe extern "C" fn attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_AttackCommon();
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.check_attack_mtrans();
    }
    fighter.global_table[SUB_STATUS3].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_check_attack_mtrans as *const () as _));
    fighter.sub_status_AttackComboCommon();
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_Attack_Main as *const () as _))
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_ATTACK, attack_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK, attack_main);
}
