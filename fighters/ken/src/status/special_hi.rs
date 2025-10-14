use super::*;


// FIGHTER_STATUS_KIND_SPECIAL_HI

unsafe extern "C" fn special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut mask_flags = (*FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64;

    if fighter.global_table[STATUS_KIND_INTERRUPT] != FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND {
        mask_flags |= *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI as u64;
    }
    else {
        mask_flags |= *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI_COMMAND as u64;
    }

    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        mask_flags,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return 0.into();
}

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
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_pre);
    agent.status(End, *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_JUMP, special_hi_jump_end);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_jump_end);
    agent.status(Pre, *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND, special_hi_pre);
    agent.status(End, *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND, special_hi_jump_end);
}
