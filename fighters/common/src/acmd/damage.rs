use super::*;

const DAMAGE_STATUSES: [smash::lib::LuaConst ; 8] = [
    FIGHTER_STATUS_KIND_DAMAGE,
    FIGHTER_STATUS_KIND_DAMAGE_AIR,
    FIGHTER_STATUS_KIND_DAMAGE_FLY,
    FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
    FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
    FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
    FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
    FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
];


// Fire

#[skyline::hook(replace = smash::lua2cpp::L2CFighterAnimcmdEffectCommon_effect_FireHitF)]
pub unsafe fn effect_FireHitF(fighter: &mut L2CFighterAnimcmdEffectCommon) -> L2CValue {
    let agent = &mut fighter.clone().agent;
    let boma = agent.boma();
    let excute = {
        agent.clear_lua_stack();
        is_excute(agent.lua_state_agent);
        agent.pop_lua_stack(1).get_bool()
    };

    for _ in 0..3 {
        if excute {
            if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
            || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
                return 0.into();
            }
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("sys_damage_fire"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, false);
            EFFECT_FOLLOW_NO_STOP(agent.lua_state_agent);
        }
        fighter.effect_FireHitEff();
    }
    if excute {
        if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
        || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
            return 0.into();
        }
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("sys_damage_fire_fly"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 1, false);
        EFFECT_FOLLOW_NO_STOP(agent.lua_state_agent);
    }
    fighter.effect_FireHitEff();
    if excute {
        if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
        || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
            return 0.into();
        }
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("sys_damage_fire"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, false);
        EFFECT_FOLLOW_NO_STOP(agent.lua_state_agent);
    }
    fighter.effect_FireHitEff();
    fighter.effect_FireHitEff();
    if excute {
        if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
        || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
            return 0.into();
        }
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("sys_damage_fire"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, false);
        EFFECT_FOLLOW_NO_STOP(agent.lua_state_agent);
    }
    fighter.effect_FireHitEff();
    fighter.effect_FireHitEff();
    if excute {
        if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
        || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
            return 0.into();
        }
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("sys_damage_fire"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, false);
        EFFECT_FOLLOW_NO_STOP(agent.lua_state_agent);
    }
    fighter.effect_FireHitEff();
    fighter.effect_FireHitEff();
    if excute {
        if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
        || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
            return 0.into();
        }
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("sys_damage_fire"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, false);
        EFFECT_FOLLOW_NO_STOP(agent.lua_state_agent);
    }
    fighter.effect_FireHitEff();
    fighter.effect_FireHitEff();
    for _ in 0..5 {
        if excute {
            if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
            || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
                return 0.into();
            }
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("sys_damage_fire"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1.2, false);
            EFFECT_FOLLOW_NO_STOP(agent.lua_state_agent);
        }
        if fighter.effect_FireHitEff2().get_bool() {
            return 0.into();
        }
    }

    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterAnimcmdEffectCommon_effect_FireHitF_L)]
pub unsafe fn effect_FireHitF_L(fighter: &mut L2CFighterAnimcmdEffectCommon) -> L2CValue {
    let agent = &mut fighter.clone().agent;
    let boma = agent.boma();
    let excute = {
        agent.clear_lua_stack();
        is_excute(agent.lua_state_agent);
        agent.pop_lua_stack(1).get_bool()
    };

    for _ in 0..3 {
        if excute {
            if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
            || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
                return 0.into();
            }
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("sys_damage_fire"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, false);
            EFFECT_FOLLOW_NO_STOP(agent.lua_state_agent);
        }
        fighter.effect_FireHitEff();
    }
    if excute {
        if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
        || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
            return 0.into();
        }
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("sys_damage_fire_fly"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 1, false);
        EFFECT_FOLLOW_NO_STOP(agent.lua_state_agent);
    }
    for _ in 0..9 {
        if excute {
            if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
            || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
                return 0.into();
            }
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("sys_damage_fire"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, false);
            EFFECT_FOLLOW_NO_STOP(agent.lua_state_agent);
        }
        fighter.effect_FireHitEff();
    }
    for _ in 0..5 {
        if excute {
            if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
            || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
                return 0.into();
            }
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("sys_damage_fire"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1.2, false);
            EFFECT_FOLLOW_NO_STOP(agent.lua_state_agent);
        }
        if fighter.effect_FireHitEff2().get_bool() {
            return 0.into();
        }
    }

    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterAnimcmdEffectCommon_effect_FireHitS)]
pub unsafe fn effect_FireHitS(fighter: &mut L2CFighterAnimcmdEffectCommon) -> L2CValue {
    let agent = &mut fighter.clone().agent;
    let boma = agent.boma();
    let excute = {
        agent.clear_lua_stack();
        is_excute(agent.lua_state_agent);
        agent.pop_lua_stack(1).get_bool()
    };

    if excute {
        if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
        || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
            return 0.into();
        }
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("sys_damage_fire"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, false);
        EFFECT_FOLLOW_NO_STOP(agent.lua_state_agent);
    }
    fighter.effect_FireHitEff();
    if excute {
        if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
        || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
            return 0.into();
        }
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("sys_damage_fire"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, false);
        EFFECT_FOLLOW_NO_STOP(agent.lua_state_agent);
    }
    fighter.effect_FireHitEff();
    if excute {
        if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
        || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
            return 0.into();
        }
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("sys_damage_fire"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, false);
        EFFECT_FOLLOW_NO_STOP(agent.lua_state_agent);
    }
    if fighter.effect_FireHitEff2().get_bool() {
        return 0.into();
    }
    if excute {
        agent.clear_lua_stack();
        COL_NORMAL(agent.lua_state_agent);
    }

    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterAnimcmdEffectCommon_effect_FireHitS_L)]
pub unsafe fn effect_FireHitS_L(fighter: &mut L2CFighterAnimcmdEffectCommon) -> L2CValue {
    let agent = &mut fighter.clone().agent;
    let boma = agent.boma();
    let excute = {
        agent.clear_lua_stack();
        is_excute(agent.lua_state_agent);
        agent.pop_lua_stack(1).get_bool()
    };

    for _ in 0..2 {
        if excute {
            if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
            || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
                return 0.into();
            }
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("sys_damage_fire"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, false);
            EFFECT_FOLLOW_NO_STOP(agent.lua_state_agent);
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("sys_damage_fire"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, false);
            EFFECT_FOLLOW_NO_STOP(agent.lua_state_agent);
        }
        fighter.effect_FireHitEff();
    }
    if fighter.effect_FireHitEff2().get_bool() {
        return 0.into();
    }
    if excute {
        agent.clear_lua_stack();
        COL_NORMAL(agent.lua_state_agent);
    }

    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterAnimcmdEffectCommon_effect_FireHitM)]
pub unsafe fn effect_FireHitM(fighter: &mut L2CFighterAnimcmdEffectCommon) -> L2CValue {
    let agent = &mut fighter.clone().agent;
    let boma = agent.boma();
    let excute = {
        agent.clear_lua_stack();
        is_excute(agent.lua_state_agent);
        agent.pop_lua_stack(1).get_bool()
    };

    for _ in 0..4 {
        if excute {
            if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
            || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
                return 0.into();
            }
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("sys_damage_fire"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, false);
            EFFECT_FOLLOW_NO_STOP(agent.lua_state_agent);
        }
        fighter.effect_FireHitEff();
    }
    if fighter.effect_FireHitEff2().get_bool() {
        return 0.into();
    }
    if fighter.effect_FireHitEff2().get_bool() {
        return 0.into();
    }

    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterAnimcmdEffectCommon_effect_FireHitM_L)]
pub unsafe fn effect_FireHitM_L(fighter: &mut L2CFighterAnimcmdEffectCommon) -> L2CValue {
    let agent = &mut fighter.clone().agent;
    let boma = agent.boma();
    let excute = {
        agent.clear_lua_stack();
        is_excute(agent.lua_state_agent);
        agent.pop_lua_stack(1).get_bool()
    };

    for _ in 0..4 {
        if excute {
            if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
            || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
                return 0.into();
            }
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("sys_damage_fire"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, false);
            EFFECT_FOLLOW_NO_STOP(agent.lua_state_agent);
        }
        fighter.effect_FireHitEff();
    }
    if excute {
        if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
        || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
            return 0.into();
        }
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("sys_damage_fire"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, false);
        EFFECT_FOLLOW_NO_STOP(agent.lua_state_agent);
    }
    if fighter.effect_FireHitEff2().get_bool() {
        return 0.into();
    }
    if fighter.effect_FireHitEff2().get_bool() {
        return 0.into();
    }

    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterAnimcmdEffectCommon_effect_FireHitL)]
pub unsafe fn effect_FireHitL(fighter: &mut L2CFighterAnimcmdEffectCommon) -> L2CValue {
    let agent = &mut fighter.clone().agent;
    let boma = agent.boma();
    let excute = {
        agent.clear_lua_stack();
        is_excute(agent.lua_state_agent);
        agent.pop_lua_stack(1).get_bool()
    };

    for _ in 0..8 {
        if excute {
            if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
            || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
                return 0.into();
            }
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("sys_damage_fire"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, false);
            EFFECT_FOLLOW_NO_STOP(agent.lua_state_agent);
        }
        fighter.effect_FireHitEff();
    }
    if fighter.effect_FireHitEff2().get_bool() {
        return 0.into();
    }
    if fighter.effect_FireHitEff2().get_bool() {
        return 0.into();
    }
    if fighter.effect_FireHitEff2().get_bool() {
        return 0.into();
    }
    if fighter.effect_FireHitEff2().get_bool() {
        return 0.into();
    }

    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterAnimcmdEffectCommon_effect_FireHitL_L)]
pub unsafe fn effect_FireHitL_L(fighter: &mut L2CFighterAnimcmdEffectCommon) -> L2CValue {
    let agent = &mut fighter.clone().agent;
    let boma = agent.boma();
    let excute = {
        agent.clear_lua_stack();
        is_excute(agent.lua_state_agent);
        agent.pop_lua_stack(1).get_bool()
    };

    for _ in 0..8 {
        if excute {
            if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
            || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
                return 0.into();
            }
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("sys_damage_fire"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, false);
            EFFECT_FOLLOW_NO_STOP(agent.lua_state_agent);
        }
        fighter.effect_FireHitEff();
    }
    if excute {
        if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
        || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
            return 0.into();
        }
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("sys_damage_fire"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, false);
        EFFECT_FOLLOW_NO_STOP(agent.lua_state_agent);
    }
    if fighter.effect_FireHitEff2().get_bool() {
        return 0.into();
    }
    if fighter.effect_FireHitEff2().get_bool() {
        return 0.into();
    }
    if fighter.effect_FireHitEff2().get_bool() {
        return 0.into();
    }
    if fighter.effect_FireHitEff2().get_bool() {
        return 0.into();
    }

    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterAnimcmdEffectCommon_effect_FireHitEff)]
pub unsafe fn effect_FireHitEff(fighter: &mut L2CFighterAnimcmdEffectCommon) -> L2CValue {
    let agent = &mut fighter.clone().agent;
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let excute = {
        agent.clear_lua_stack();
        is_excute(agent.lua_state_agent);
        agent.pop_lua_stack(1).get_bool()
    };

    if excute {
        if DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
        && !boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
            agent.clear_lua_stack();
            lua_args!(agent, 2, 0.15, 0.02, 0.95);
            BURN_COLOR(agent.lua_state_agent);
        }
    }
    wait(lua_state, 2.0);
    if excute {
        agent.clear_lua_stack();
        BURN_COLOR_NORMAL(agent.lua_state_agent);
        if DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
        && !boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
            agent.clear_lua_stack();
            lua_args!(agent, 0.3, 0.01, 0, 0.5);
            FLASH(agent.lua_state_agent);
        }
    }
    wait(lua_state, 2.0);
    if excute {
        agent.clear_lua_stack();
        COL_NORMAL(agent.lua_state_agent);
    }

    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterAnimcmdEffectCommon_effect_FireHitEff2)]
pub unsafe fn effect_FireHitEff2(fighter: &mut L2CFighterAnimcmdEffectCommon) -> L2CValue {
    let agent = &mut fighter.clone().agent;
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let excute = {
        agent.clear_lua_stack();
        is_excute(agent.lua_state_agent);
        agent.pop_lua_stack(1).get_bool()
    };

    if excute {
        if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
        || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
            return 1.into();
        }
        agent.clear_lua_stack();
        lua_args!(agent, 2, 0.15, 0.02, 0.8);
        BURN_COLOR(agent.lua_state_agent);
        agent.clear_lua_stack();
        lua_args!(agent, 0.3, 0.01, 0, 0.2);
        FLASH(agent.lua_state_agent);
    }
    wait(lua_state, 1.0);
    if excute {
        if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
        || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
            return 1.into();
        }
        agent.clear_lua_stack();
        lua_args!(agent, 3, 0x10, 1.2, 0.16, 0);
        BURN_COLOR_FRAME(agent.lua_state_agent);
        agent.clear_lua_stack();
        lua_args!(agent, 3, 0.3, 0.01, 0, 0);
        FLASH_FRM(agent.lua_state_agent);
    }
    wait(lua_state, 4.0);
    if excute {
        agent.clear_lua_stack();
        BURN_COLOR_NORMAL(agent.lua_state_agent);
        agent.clear_lua_stack();
        COL_NORMAL(agent.lua_state_agent);
    }
    wait(lua_state, 1.0);

    0.into()
}


// Purple

#[skyline::hook(replace = smash::lua2cpp::L2CFighterAnimcmdEffectCommon_effect_PurpleFireHitF)]
pub unsafe fn effect_PurpleFireHitF(fighter: &mut L2CFighterAnimcmdEffectCommon) -> L2CValue {
    let agent = &mut fighter.clone().agent;
    let boma = agent.boma();
    let excute = {
        agent.clear_lua_stack();
        is_excute(agent.lua_state_agent);
        agent.pop_lua_stack(1).get_bool()
    };

    for _ in 0..12 {
        if excute {
            if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
            || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
                return 0.into();
            }
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new_raw(0x116c693a44), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, false);
            EFFECT_FOLLOW_NO_STOP(agent.lua_state_agent);
        }
        fighter.effect_PurpleFireHitEff();
    }
    if fighter.effect_PurpleFireHitEff2().get_bool() {
        return 0.into();
    }
    if fighter.effect_PurpleFireHitEff2().get_bool() {
        return 0.into();
    }
    if fighter.effect_PurpleFireHitEff2().get_bool() {
        return 0.into();
    }
    if fighter.effect_PurpleFireHitEff2().get_bool() {
        return 0.into();
    }
    if fighter.effect_PurpleFireHitEff2().get_bool() {
        return 0.into();
    }
    if fighter.effect_PurpleFireHitEff2().get_bool() {
        return 0.into();
    }

    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterAnimcmdEffectCommon_effect_PurpleFireHitS)]
pub unsafe fn effect_PurpleFireHitS(fighter: &mut L2CFighterAnimcmdEffectCommon) -> L2CValue {
    let agent = &mut fighter.clone().agent;
    let boma = agent.boma();
    let excute = {
        agent.clear_lua_stack();
        is_excute(agent.lua_state_agent);
        agent.pop_lua_stack(1).get_bool()
    };

    if excute {
        if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
        || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
            return 0.into();
        }
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new_raw(0x116c693a44), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent.lua_state_agent);
    }
    fighter.effect_PurpleFireHitEff();
    if excute {
        if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
        || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
            return 0.into();
        }
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new_raw(0x116c693a44), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent.lua_state_agent);
    }
    fighter.effect_PurpleFireHitEff();
    if fighter.effect_PurpleFireHitEff2().get_bool() {
        return 0.into();
    }

    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterAnimcmdEffectCommon_effect_PurpleFireHitM)]
pub unsafe fn effect_PurpleFireHitM(fighter: &mut L2CFighterAnimcmdEffectCommon) -> L2CValue {
    let agent = &mut fighter.clone().agent;
    let boma = agent.boma();
    let excute = {
        agent.clear_lua_stack();
        is_excute(agent.lua_state_agent);
        agent.pop_lua_stack(1).get_bool()
    };

    for _ in 0..4 {
        if excute {
            if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
            || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
                return 0.into();
            }
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new_raw(0x116c693a44), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            EFFECT(agent.lua_state_agent);
        }
        fighter.effect_PurpleFireHitEff();
    }
    if fighter.effect_PurpleFireHitEff2().get_bool() {
        return 0.into();
    }
    if fighter.effect_PurpleFireHitEff2().get_bool() {
        return 0.into();
    }

    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterAnimcmdEffectCommon_effect_PurpleFireHitL)]
pub unsafe fn effect_PurpleFireHitL(fighter: &mut L2CFighterAnimcmdEffectCommon) -> L2CValue {
    let agent = &mut fighter.clone().agent;
    let boma = agent.boma();
    let excute = {
        agent.clear_lua_stack();
        is_excute(agent.lua_state_agent);
        agent.pop_lua_stack(1).get_bool()
    };

    for _ in 0..8 {
        if excute {
            if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
            || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
                return 0.into();
            }
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new_raw(0x116c693a44), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            EFFECT(agent.lua_state_agent);
        }
        fighter.effect_PurpleFireHitEff();
    }
    if fighter.effect_PurpleFireHitEff2().get_bool() {
        return 0.into();
    }
    if fighter.effect_PurpleFireHitEff2().get_bool() {
        return 0.into();
    }
    if fighter.effect_PurpleFireHitEff2().get_bool() {
        return 0.into();
    }
    if fighter.effect_PurpleFireHitEff2().get_bool() {
        return 0.into();
    }

    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterAnimcmdEffectCommon_effect_PurpleFireHitEff)]
pub unsafe fn effect_PurpleFireHitEff(fighter: &mut L2CFighterAnimcmdEffectCommon) -> L2CValue {
    let agent = &mut fighter.clone().agent;
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let excute = {
        agent.clear_lua_stack();
        is_excute(agent.lua_state_agent);
        agent.pop_lua_stack(1).get_bool()
    };

    if excute {
        if DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
        && !boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
            agent.clear_lua_stack();
            lua_args!(agent, 0.8, 0.4, 2, 0.2);
            FLASH(agent.lua_state_agent);
            agent.clear_lua_stack();
            lua_args!(agent, 2, 1, 7, 0.4);
            BURN_COLOR(agent.lua_state_agent);
        }
    }
    wait(lua_state, 1.0);
    if excute {
        if DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
        && !boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
            agent.clear_lua_stack();
            lua_args!(agent, 2, 0.8, 0.4, 2, 0);
            FLASH_FRM(agent.lua_state_agent);
        }
    }
    wait(lua_state, 1.0);
    if excute {
        agent.clear_lua_stack();
        BURN_COLOR_NORMAL(agent.lua_state_agent);
        agent.clear_lua_stack();
        COL_NORMAL(agent.lua_state_agent);
        if DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
        && !boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
            agent.clear_lua_stack();
            lua_args!(agent, 0.2, 0, 1, 0.3);
            FLASH(agent.lua_state_agent);
        }
    }
    wait(lua_state, 1.0);
    if excute {
        if DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
        && !boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
            agent.clear_lua_stack();
            lua_args!(agent, 2, 0.2, 0, 1, 0);
            FLASH_FRM(agent.lua_state_agent);
        }
    }
    wait(lua_state, 2.0);
    if excute {
        agent.clear_lua_stack();
        COL_NORMAL(agent.lua_state_agent);
    }
    wait(lua_state, 2.0);

    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterAnimcmdEffectCommon_effect_PurpleFireHitEff2)]
pub unsafe fn effect_PurpleFireHitEff2(fighter: &mut L2CFighterAnimcmdEffectCommon) -> L2CValue {
    let agent = &mut fighter.clone().agent;
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let excute = {
        agent.clear_lua_stack();
        is_excute(agent.lua_state_agent);
        agent.pop_lua_stack(1).get_bool()
    };

    if excute {
        if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
        || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
            return 1.into();
        }
        agent.clear_lua_stack();
        lua_args!(agent, 0.196, 0, 0.392, 0.157);
        FLASH(agent.lua_state_agent);
    }
    wait(lua_state, 2.0);
    if excute {
        if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
        || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
            return 1.into();
        }
        agent.clear_lua_stack();
        lua_args!(agent, 0.118, 0, 0.235, 0.078);
        FLASH(agent.lua_state_agent);
    }
    wait(lua_state, 2.0);
    if excute {
        agent.clear_lua_stack();
        COL_NORMAL(agent.lua_state_agent);
    }
    wait(lua_state, 2.0);

    0.into()
}


// Elec

#[skyline::hook(replace = smash::lua2cpp::L2CFighterAnimcmdEffectCommon_effect_ElecHitF)]
pub unsafe fn effect_ElecHitF(fighter: &mut L2CFighterAnimcmdEffectCommon) -> L2CValue {
    let agent = &mut fighter.clone().agent;
    let boma = agent.boma();
    let excute = {
        agent.clear_lua_stack();
        is_excute(agent.lua_state_agent);
        agent.pop_lua_stack(1).get_bool()
    };

    for _ in 0..15 {
        if excute {
            if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
            || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
                return 0.into();
            }
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("sys_damage_elec"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            EFFECT(agent.lua_state_agent);
            
        }
        fighter.effect_ElecHitEff();
    }
    if excute {
        if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
        || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
            return 0.into();
        }
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("sys_damage_elec"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent.lua_state_agent);
        
    }

    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterAnimcmdEffectCommon_effect_ElecHitS)]
pub unsafe fn effect_ElecHitS(fighter: &mut L2CFighterAnimcmdEffectCommon) -> L2CValue {
    let agent = &mut fighter.clone().agent;
    let boma = agent.boma();
    let excute = {
        agent.clear_lua_stack();
        is_excute(agent.lua_state_agent);
        agent.pop_lua_stack(1).get_bool()
    };

    for _ in 0..4 {
        if excute {
            if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
            || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
                return 0.into();
            }
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("sys_damage_elec"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, true);
            EFFECT(agent.lua_state_agent);
            
        }
        fighter.effect_ElecHitEff();
    }
    if excute {
        if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
        || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
            return 0.into();
        }
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("sys_damage_elec"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent.lua_state_agent);
        
    }

    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterAnimcmdEffectCommon_effect_ElecHitM)]
pub unsafe fn effect_ElecHitM(fighter: &mut L2CFighterAnimcmdEffectCommon) -> L2CValue {
    let agent = &mut fighter.clone().agent;
    let boma = agent.boma();
    let excute = {
        agent.clear_lua_stack();
        is_excute(agent.lua_state_agent);
        agent.pop_lua_stack(1).get_bool()
    };

    for _ in 0..7 {
        if excute {
            if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
            || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
                return 0.into();
            }
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("sys_damage_elec"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            EFFECT(agent.lua_state_agent);
            
        }
        fighter.effect_ElecHitEff();
    }
    if excute {
        if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
        || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
            return 0.into();
        }
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("sys_damage_elec"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent.lua_state_agent);
        
    }

    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterAnimcmdEffectCommon_effect_ElecHitL)]
pub unsafe fn effect_ElecHitL(fighter: &mut L2CFighterAnimcmdEffectCommon) -> L2CValue {
    let agent = &mut fighter.clone().agent;
    let boma = agent.boma();
    let excute = {
        agent.clear_lua_stack();
        is_excute(agent.lua_state_agent);
        agent.pop_lua_stack(1).get_bool()
    };

    for _ in 0..10 {
        if excute {
            if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
            || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
                return 0.into();
            }
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("sys_damage_elec"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            EFFECT(agent.lua_state_agent);
            
        }
        fighter.effect_ElecHitEff();
    }
    if excute {
        if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
        || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
            return 0.into();
        }
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("sys_damage_elec"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent.lua_state_agent);
        
    }

    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterAnimcmdEffectCommon_effect_ElecHitEff)]
pub unsafe fn effect_ElecHitEff(fighter: &mut L2CFighterAnimcmdEffectCommon) -> L2CValue {
    let agent = &mut fighter.clone().agent;
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let excute = {
        agent.clear_lua_stack();
        is_excute(agent.lua_state_agent);
        agent.pop_lua_stack(1).get_bool()
    };

    if excute {
        if DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
        && !boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
            agent.clear_lua_stack();
            lua_args!(agent, 0.8, 0.8, 2, 0.2);
            FLASH(agent.lua_state_agent);
            agent.clear_lua_stack();
            lua_args!(agent, 4, 1.6, 8, 0.8);
            BURN_COLOR(agent.lua_state_agent);
        }
    }
    wait(lua_state, 1.0);
    if excute {
        if DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
        && !boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
            agent.clear_lua_stack();
            lua_args!(agent, 2, 0.6, 0.6, 2, 0);
            FLASH_FRM(agent.lua_state_agent);
        }
    }
    wait(lua_state, 2.0);
    if excute {
        agent.clear_lua_stack();
        BURN_COLOR_NORMAL(agent.lua_state_agent);
        agent.clear_lua_stack();
        COL_NORMAL(agent.lua_state_agent);
        if DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
        && !boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
            agent.clear_lua_stack();
            lua_args!(agent, 0, 0, 0.1, 0.8);
            FLASH(agent.lua_state_agent);
        }
    }
    wait(lua_state, 1.0);
    if excute {
        if DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
        && !boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
            agent.clear_lua_stack();
            lua_args!(agent, 2, 0, 0, 0.1, 0);
            FLASH_FRM(agent.lua_state_agent);
        }
    }
    wait(lua_state, 2.0);
    if excute {
        agent.clear_lua_stack();
        COL_NORMAL(agent.lua_state_agent);
    }
    wait(lua_state, 2.0);

    0.into()
}


// Aura

#[skyline::hook(replace = smash::lua2cpp::L2CFighterAnimcmdEffectCommon_effect_AuraHitF)]
pub unsafe fn effect_AuraHitF(fighter: &mut L2CFighterAnimcmdEffectCommon) -> L2CValue {
    let agent = &mut fighter.clone().agent;
    let boma = agent.boma();
    let excute = {
        agent.clear_lua_stack();
        is_excute(agent.lua_state_agent);
        agent.pop_lua_stack(1).get_bool()
    };

    for _ in 0..12 {
        if excute {
            if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
            || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
                return 0.into();
            }
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("sys_damage_aura"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, false);
            EFFECT_FOLLOW_NO_STOP(agent.lua_state_agent);
        }
        fighter.effect_AuraHitEff();
    }
    if fighter.effect_AuraHitEff2().get_bool() {
        return 0.into();
    }
    if fighter.effect_AuraHitEff2().get_bool() {
        return 0.into();
    }
    if fighter.effect_AuraHitEff2().get_bool() {
        return 0.into();
    }
    if fighter.effect_AuraHitEff2().get_bool() {
        return 0.into();
    }
    if fighter.effect_AuraHitEff2().get_bool() {
        return 0.into();
    }
    if fighter.effect_AuraHitEff2().get_bool() {
        return 0.into();
    }

    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterAnimcmdEffectCommon_effect_AuraHitS)]
pub unsafe fn effect_AuraHitS(fighter: &mut L2CFighterAnimcmdEffectCommon) -> L2CValue {
    let agent = &mut fighter.clone().agent;
    let boma = agent.boma();
    let excute = {
        agent.clear_lua_stack();
        is_excute(agent.lua_state_agent);
        agent.pop_lua_stack(1).get_bool()
    };

    if excute {
        if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
        || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
            return 0.into();
        }
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("sys_damage_aura"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent.lua_state_agent);
    }
    fighter.effect_AuraHitEff();
    if excute {
        if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
        || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
            return 0.into();
        }
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("sys_damage_aura"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent.lua_state_agent);
    }
    fighter.effect_AuraHitEff();
    if fighter.effect_AuraHitEff2().get_bool() {
        return 0.into();
    }

    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterAnimcmdEffectCommon_effect_AuraHitM)]
pub unsafe fn effect_AuraHitM(fighter: &mut L2CFighterAnimcmdEffectCommon) -> L2CValue {
    let agent = &mut fighter.clone().agent;
    let boma = agent.boma();
    let excute = {
        agent.clear_lua_stack();
        is_excute(agent.lua_state_agent);
        agent.pop_lua_stack(1).get_bool()
    };

    for _ in 0..4 {
        if excute {
            if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
            || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
                return 0.into();
            }
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("sys_damage_aura"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            EFFECT(agent.lua_state_agent);
        }
        fighter.effect_AuraHitEff();
    }
    if fighter.effect_AuraHitEff2().get_bool() {
        return 0.into();
    }
    if fighter.effect_AuraHitEff2().get_bool() {
        return 0.into();
    }

    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterAnimcmdEffectCommon_effect_AuraHitL)]
pub unsafe fn effect_AuraHitL(fighter: &mut L2CFighterAnimcmdEffectCommon) -> L2CValue {
    let agent = &mut fighter.clone().agent;
    let boma = agent.boma();
    let excute = {
        agent.clear_lua_stack();
        is_excute(agent.lua_state_agent);
        agent.pop_lua_stack(1).get_bool()
    };

    for _ in 0..8 {
        if excute {
            if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
            || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
                return 0.into();
            }
            agent.clear_lua_stack();
            lua_args!(agent, Hash40::new("sys_damage_aura"), Hash40::new("emit"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            EFFECT(agent.lua_state_agent);
        }
        fighter.effect_AuraHitEff();
    }
    if fighter.effect_AuraHitEff2().get_bool() {
        return 0.into();
    }
    if fighter.effect_AuraHitEff2().get_bool() {
        return 0.into();
    }
    if fighter.effect_AuraHitEff2().get_bool() {
        return 0.into();
    }
    if fighter.effect_AuraHitEff2().get_bool() {
        return 0.into();
    }

    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterAnimcmdEffectCommon_effect_AuraHitEff)]
pub unsafe fn effect_AuraHitEff(fighter: &mut L2CFighterAnimcmdEffectCommon) -> L2CValue {
    let agent = &mut fighter.clone().agent;
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let excute = {
        agent.clear_lua_stack();
        is_excute(agent.lua_state_agent);
        agent.pop_lua_stack(1).get_bool()
    };

    if excute {
        if DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
        && !boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
            agent.clear_lua_stack();
            lua_args!(agent, 0.502, 0.627, 1, 0.314);
            FLASH(agent.lua_state_agent);
        }
    }
    wait(lua_state, 1.0);
    if excute {
        if DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
        && !boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
            agent.clear_lua_stack();
            lua_args!(agent, 4, 0, 0, 0.235, 0.039);
            FLASH_FRM(agent.lua_state_agent);
        }
    }
    wait(lua_state, 4.0);
    if excute {
        agent.clear_lua_stack();
        COL_NORMAL(agent.lua_state_agent);
    }
    wait(lua_state, 1.0);

    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterAnimcmdEffectCommon_effect_AuraHitEff2)]
pub unsafe fn effect_AuraHitEff2(fighter: &mut L2CFighterAnimcmdEffectCommon) -> L2CValue {
    let agent = &mut fighter.clone().agent;
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let excute = {
        agent.clear_lua_stack();
        is_excute(agent.lua_state_agent);
        agent.pop_lua_stack(1).get_bool()
    };

    if excute {
        if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
        || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
            return 1.into();
        }
        agent.clear_lua_stack();
        lua_args!(agent, 0.392, 0.627, 0.784, 0.157);
        FLASH(agent.lua_state_agent);
    }
    wait(lua_state, 2.0);
    if excute {
        if !DAMAGE_STATUSES.iter().any(|x| **x == StatusModule::status_kind(boma) )
        || boma.is_flag(*FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION) {
            return 1.into();
        }
        agent.clear_lua_stack();
        lua_args!(agent, 0, 0, 0.235, 0.078);
        FLASH(agent.lua_state_agent);
    }
    wait(lua_state, 2.0);
    if excute {
        agent.clear_lua_stack();
        COL_NORMAL(agent.lua_state_agent);
    }
    wait(lua_state, 2.0);

    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            effect_FireHitF,
            effect_FireHitF_L,
            effect_FireHitS,
            effect_FireHitS_L,
            effect_FireHitM,
            effect_FireHitM_L,
            effect_FireHitL,
            effect_FireHitL_L,
            effect_FireHitEff,
            effect_FireHitEff2,

            effect_PurpleFireHitF,
            effect_PurpleFireHitS,
            effect_PurpleFireHitM,
            effect_PurpleFireHitL,
            effect_PurpleFireHitEff,
            effect_PurpleFireHitEff2,

            effect_ElecHitF,
            effect_ElecHitS,
            effect_ElecHitM,
            effect_ElecHitL,
            effect_ElecHitEff,

            effect_AuraHitF,
            effect_AuraHitS,
            effect_AuraHitM,
            effect_AuraHitL,
            effect_AuraHitEff,
            effect_AuraHitEff2
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}