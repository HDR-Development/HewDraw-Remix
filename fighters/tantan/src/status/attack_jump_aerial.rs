use super::*;

// FIGHTER_TANTAN_STATUS_KIND_ATTACK_JUMP_AERIAL

unsafe extern "C" fn attack_jump_aerial_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    EFFECT_OFF_KIND(fighter, Hash40::new("tantan_jump_line_s"), false, true);
    EFFECT_OFF_KIND(fighter, Hash40::new("tantan_jump_line_l"), false, true);
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_jump_aerial"), true, true);

    let ret = smashline::original_status(Main, fighter, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_JUMP_AERIAL)(fighter);

    if StatusModule::is_changing(fighter.module_accessor) {
        EFFECT(fighter, Hash40::new("sys_jump_aerial"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    
    return ret;
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_JUMP_AERIAL, attack_jump_aerial_main);
}
