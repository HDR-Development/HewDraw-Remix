use super::*;

unsafe extern "C" fn game_speciallwstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 17.0, 11.0);
    frame(lua_state, 17.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_speciallwbiteattack(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = StanceInfo::from(VarModule::get_int(agent.battle_object, vars::packun::instance::CURRENT_STANCE));
    if WorkModule::get_float(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_LW_WORK_FLOAT_CHARGE_RATE) > 1.0 {
        // Max charge
        if is_excute(agent) {
            if stance.label == STANCE_PUTRID {
                ATTACK(agent, 0, 0, Hash40::new("mouth"), 20.0 * stance.damage_bite, 60, 70, 0, 60, 7.9, 2.4, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 20, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
                ATTACK(agent, 1, 1, Hash40::new("mouth"), 0.0, 0, 0, 0, 0, 7.9, 2.4, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BITE);
                AttackModule::set_poison_param(boma, 0, 241, 60, 1.5, false);
            }
            else {
                ATTACK(agent, 0, 0, Hash40::new("mouth"), 20.0 * stance.damage_head, 60, 70, 0, 60, 7.9, 2.4, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 20, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
            }
        }
    }
    else {
        // Below max charge
        if is_excute(agent) {
            let charge = agent.get_int(*FIGHTER_PACKUN_STATUS_SPECIAL_LW_WORK_INT_CHARGE_COUNT);
            let angle = (81.0 - charge as f32 * 0.5) as u64;
            let kbg = (55.0 + charge as f32 * 0.2) as i32;
            if stance.label == STANCE_PUTRID {
                ATTACK(agent, 0, 0, Hash40::new("mouth"), 0.0, angle, kbg, 0, 60, 7.9, 2.4, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_poison"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
                ATTACK(agent, 1, 1, Hash40::new("mouth"), 0.0, 0, 0, 0, 0, 7.9, 2.4, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BITE);
                AttackModule::set_poison_param(boma, 0, 241, 60, 1.5, false);
            }
            else {
                ATTACK(agent, 0, 0, Hash40::new("mouth"), 0.0 * stance.damage_head, angle, kbg, 0, 60, 7.9, 2.4, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
            }
            WorkModule::on_flag(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_LW_FLAG_ATTACK_LERP);
        }
    }
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_LW_FLAG_BITE_ATTACK);
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
}

unsafe extern "C" fn effect_speciallwbite(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = StanceInfo::from(VarModule::get_int(agent.battle_object, vars::packun::instance::CURRENT_STANCE));
    frame(lua_state, 1.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("packun_longrange_start"), Hash40::new("mouth"), 0, 0, 0, 180, 0, 0, 0.85, true);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("packun_longrange_bite_line"), Hash40::new("mouth"), 6, 0, 0, 0, 90, 0, 0.8, true);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        if stance.label == STANCE_PUTRID {
            EFFECT_FOLLOW(agent, Hash40::new("packun_poison_max"), Hash40::new("mouth"), 6, -0.6, 0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_COLOR(agent, 0.5, 0.5, 0.5);
            EFFECT_FOLLOW(agent, Hash40::new("packun_poison_max"), Hash40::new("mouth"), 6, -0.6, 0, 0, 0, 0, 0.9, true);
            EFFECT_FOLLOW(agent, Hash40::new("packun_poison_mouth"), Hash40::new("mouth"), 6, -0.6, 0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_RATE(agent, 2.0);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        let size = if stance.label != STANCE_PRICKLY { 0.85 } else { 1.0 };
        EFFECT_FOLLOW(agent, Hash40::new("packun_longrange_bite"), Hash40::new("mouth"), 0, 0, 0, 180, 0, 0, size, true);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("packun_longrange_bite"), -1);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        let size = if stance.label != STANCE_PRICKLY { 0.9 } else { 1.0 };
        EFFECT_FOLLOW(agent, Hash40::new("packun_longrange_bite_line2"), Hash40::new("mouth"), 5, 0, 0, 0, 90, 0, size, true);
    }
}

unsafe extern "C" fn effect_specialairlwbite(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = StanceInfo::from(VarModule::get_int(agent.battle_object, vars::packun::instance::CURRENT_STANCE));
    frame(lua_state, 5.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("packun_longrange_start"), Hash40::new("mouth"), 0, 0, 0, 180, 0, 0, 0.85, true);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        if stance.label == STANCE_PUTRID {
            EFFECT_FOLLOW(agent, Hash40::new("packun_poison_max"), Hash40::new("mouth"), 6, -0.6, 0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_COLOR(agent, 0.5, 0.5, 0.5);
            EFFECT_FOLLOW(agent, Hash40::new("packun_poison_max"), Hash40::new("mouth"), 6, -0.6, 0, 0, 0, 0, 0.9, true);
            EFFECT_FOLLOW(agent, Hash40::new("packun_poison_mouth"), Hash40::new("mouth"), 6, -0.6, 0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_RATE(agent, 2.0);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        let size = if stance.label != STANCE_PRICKLY { 0.85 } else { 1.0 };
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("packun_longrange_bite_line"), Hash40::new("mouth"), 6, 0, 0, 0, 90, 0, 0.9, true);
        EFFECT_FOLLOW(agent, Hash40::new("packun_longrange_bite"), Hash40::new("mouth"), 0, 0, 0, 180, 0, 0, size, true);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("packun_longrange_bite"), -1);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        let size = if stance.label != STANCE_PRICKLY { 0.9 } else { 1.0 };
        EFFECT_FOLLOW(agent, Hash40::new("packun_longrange_bite_line2"), Hash40::new("mouth"), 5, 0, 0, 0, 90, 0, size, true);
    }
}

unsafe extern "C" fn game_speciallwend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
        VarModule::off_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
        FT_MOTION_RATE_RANGE(agent, 1.0, 19.0, 9.0);
    }
    wait(lua_state, 15.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_LW_FLAG_END_CHANGE_KINETIC);
    }
    frame(lua_state, 19.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_speciallwfallend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_AIR) {
            KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 0.7, z: 0.0});
        }
    }
    frame(lua_state, 1.0);
    if VarModule::is_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
        VarModule::off_flag(agent.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
        FT_MOTION_RATE_RANGE(agent, 1.0, 19.0, 9.0);
    }
    wait(lua_state, 15.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_LW_FLAG_END_CHANGE_KINETIC);
    }
    frame(lua_state, 19.0);
    FT_MOTION_RATE(agent, 1.0);
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_speciallwstart", game_speciallwstart, Priority::Low);
    agent.acmd("game_specialairlwstart", game_speciallwstart, Priority::Low);

    agent.acmd("game_speciallwbiteattack", game_speciallwbiteattack, Priority::Low);
    agent.acmd("game_speciallwbite_attack", game_speciallwbiteattack, Priority::Low);
    agent.acmd("effect_speciallwbite", effect_speciallwbite, Priority::Low);
    agent.acmd("effect_specialairlwbite", effect_specialairlwbite, Priority::Low);

    agent.acmd("game_speciallwend", game_speciallwend, Priority::Low);
    agent.acmd("game_specialairlwend", game_speciallwend, Priority::Low);
    agent.acmd("game_speciallwfallend", game_speciallwfallend, Priority::Low);
    agent.acmd("game_specialairlwfallend", game_speciallwfallend, Priority::Low);
}