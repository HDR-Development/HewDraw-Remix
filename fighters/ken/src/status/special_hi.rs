use super::*;

// FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_JUMP

pub unsafe extern "C" fn special_hi_jump_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND] == FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_JUMP
    || fighter.global_table[STATUS_KIND] == FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_FALL
    || fighter.global_table[STATUS_KIND] == FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_LANDING {
        ItemModule::set_change_status_event(fighter.module_accessor, true);
        return 0.into();
    }

    MotionAnimcmdModule::flush(fighter.module_accessor, false);

	EffectModule::kill_kind(fighter.module_accessor, Hash40::new("ryu_syoryuken_line"), false, true);
    let id = VarModule::get_int(fighter.battle_object, vars::shotos::instance::SPECIAL_HI_FIRE_EFFECT_HANDLE) as u32;
    EffectModule::kill(fighter.module_accessor, id, true, true);
	EffectModule::kill_kind(fighter.module_accessor, Hash40::new("ken_syoryuken_firearc"), false, true);
	EffectModule::kill_kind(fighter.module_accessor, Hash40::new("ken_syoryuken_firearc2"), false, true);

    ItemModule::set_change_status_event(fighter.module_accessor, true);
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(End, *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_JUMP, special_hi_jump_end);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_jump_end);
    agent.status(End, *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND, special_hi_jump_end);
}
