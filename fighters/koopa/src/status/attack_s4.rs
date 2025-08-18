use super::*;

unsafe extern "C" fn attack_s4_hold_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    EFFECT_OFF_KIND(fighter, Hash40::new("sys_explosion_sign"), false, false);
    return smashline::original_status(Exit, fighter, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD)(fighter);
}

unsafe extern "C" fn attack_s4_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    SlowModule::clear_whole(fighter.module_accessor);
    CameraModule::reset_all(fighter.module_accessor);
    EffectModule::remove_screen(fighter.module_accessor, Hash40::new("bg_criticalhit"), 0);

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Exit, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, attack_s4_hold_exit);
    agent.status(Exit, *FIGHTER_STATUS_KIND_ATTACK_S4, attack_s4_exit);
}