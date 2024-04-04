use super::*;

unsafe extern "C" fn game_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("shoulderl"), 15.0, 50, 120, 0, 20, 3.8, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("shoulderl"), 15.0, 50, 120, 0, 20, 4.3, 2.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("shoulderr"), 15.0, 50, 120, 0, 20, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_attacks4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 7, 8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FLIP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0.5, 8, 0, 0, -5, 0, 0.7, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_NONE);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("luigi_smash_thrust"), Hash40::new("luigi_smash_thrust"), Hash40::new("top"), 0.5, 8, 0, 0, -5, 0, 1.1, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW(agent, Hash40::new("sys_hit_elec_s"), Hash40::new("handl"), 3.0, 0.0, 0.0, 0, 90, 90, 0.175, true);
        //EFFECT_FOLLOW(agent, Hash40::new("sys_thunder"), Hash40::new("handl"), 0.0, 0.0, 0.0, 0, 90, 90, 1.0, true);
        //LAST_EFFECT_SET_RATE(agent, 1.25);
        //EffectModule::set_scale_last(boma, &Vector3f::new(0.5, 1.0, 0.5));
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_hit_elec_s"), true, true);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_thunder"), false, false);
    }
}

unsafe extern "C" fn game_attacks4hi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("shoulderl"), 15.0, 50, 120, 0, 20, 3.8, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("shoulderl"), 15.0, 50, 120, 0, 20, 4.3, 2.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("shoulderr"), 15.0, 50, 120, 0, 20, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_attacks4hi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 7, 8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FLIP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0.5, 6.5, 1, -25, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_NONE);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("luigi_smash_thrust"), Hash40::new("luigi_smash_thrust"), Hash40::new("top"), 0.5, 6.5, 1, -25, 0, 0, 1.1, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW(agent, Hash40::new("sys_hit_elec_s"), Hash40::new("handl"), 3.0, 0.0, 0.0, 0, 90, 90, 0.175, true);
        //EFFECT_FOLLOW(agent, Hash40::new("sys_thunder"), Hash40::new("handl"), 0.0, 0.0, 0.0, 0, 90, 90, 1.0, true);
        //LAST_EFFECT_SET_RATE(agent, 1.25);
        //EffectModule::set_scale_last(boma, &Vector3f::new(0.5, 1.0, 0.5));
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_hit_elec_s"), true, true);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_thunder"), false, false);
    }
}

unsafe extern "C" fn game_attacks4lw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("shoulderl"), 15.0, 50, 120, 0, 20, 3.8, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("shoulderl"), 15.0, 50, 120, 0, 20, 4.3, 2.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("shoulderr"), 15.0, 50, 120, 0, 20, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_attacks4lw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 7, 8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FLIP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 8.7, 1, 24, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_NONE);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("luigi_smash_thrust"), Hash40::new("luigi_smash_thrust"), Hash40::new("top"), 0, 8.7, 1, 24, 0, 0, 1.1, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW(agent, Hash40::new("sys_hit_elec_s"), Hash40::new("handl"), 3.0, 0.0, 0.0, 0, 90, 90, 0.175, true);
        //EFFECT_FOLLOW(agent, Hash40::new("sys_thunder"), Hash40::new("handl"), 0.0, 0.0, 0.0, 0, 90, 90, 1.0, true);
        //LAST_EFFECT_SET_RATE(agent, 1.25);
        //EffectModule::set_scale_last(boma, &Vector3f::new(0.5, 1.0, 0.5));
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_hit_elec_s"), true, true);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sys_thunder"), false, false);
    }
}

unsafe extern "C" fn game_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("snout"), *HIT_STATUS_XLU);
        ATTACK(agent, 0, 0, Hash40::new("head"), 17.0, 110, 98, 0, 35, 4.8, 2.8, -1.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
        ATTACK(agent, 1, 0, Hash40::new("hip"), 17.0, 110, 98, 0, 35, 4.0, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
    }
    wait(lua_state, 6.0);
    if is_excute(agent) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_attacklw4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("top"), 18.0, 90, 80, 0, 40, 5.0, 0.0, 3.6, 12.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 90, 80, 0, 40, 4.5, 0.0, 3.6, 6.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("top"), 18.0, 90, 80, 0, 40, 5.0, 0.0, 3.6, -12.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 90, 80, 0, 40, 4.5, 0.0, 3.6, -6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attacks4", game_attacks4);
    agent.acmd("effect_attacks4", effect_attacks4);
    agent.acmd("game_attacks4hi", game_attacks4hi);
    agent.acmd("effect_attacks4hi", effect_attacks4hi);
    agent.acmd("game_attacks4lw", game_attacks4lw);
    agent.acmd("effect_attacks4lw", effect_attacks4lw);

    agent.acmd("game_attackhi4", game_attackhi4);
    
    agent.acmd("game_attacklw4", game_attacklw4);
}
