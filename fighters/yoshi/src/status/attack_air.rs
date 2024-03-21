use super::*;
use globals::*;

// FIGHTER_STATUS_KIND_ATTACK_AIR //

pub unsafe extern "C" attack_air_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air_inherit_jump_aerial_motion_uniq_process_init();
    0.into()
}

pub unsafe extern "C" fn attack_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    common::djc::attack_air_main_status(fighter)
}

pub unsafe extern "C" fn attack_air_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec();
    0.into()
}

pub unsafe extern "C" fn attack_air_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air_inherit_jump_aerial_motion_uniq_process_exit();
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_init);
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_exec);
    agent.status(Exit, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_exit);
}
