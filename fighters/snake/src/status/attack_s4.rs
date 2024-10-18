use super::*;

// FIGHTER_STATUS_KIND_ATTACK_S4

////changed rpg7 side-smash to multi-hit knife

unsafe extern "C" fn attack_s4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::reset_trigger(fighter.module_accessor);
    smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_ATTACK_S4)(fighter)
}

unsafe extern "C" fn attack_s4_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    VarModule::off_flag(fighter.object(), vars::snake::instance::ATTACK_S4_ENABLE_COMBO); 
    VarModule::off_flag(fighter.object(), vars::snake::instance::ATTACK_S4_COMBO_BUFFER); 
    VarModule::set_int(fighter.object(), vars::snake::instance::ATTACK_S4_COMBO_COUNT, 0); 
    smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_ATTACK_S4)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_S4, attack_s4_main);
    agent.status(End, *FIGHTER_STATUS_KIND_ATTACK_S4, attack_s4_end);
}