use super::*;

// FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_LOOP

unsafe extern "C" fn special_s_loop_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_LOOP)(fighter);
    DamageModule::set_damage_lock(fighter.module_accessor, false);
    ret
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_LOOP, special_s_loop_main);
}
