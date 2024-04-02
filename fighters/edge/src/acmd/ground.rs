use super::*;

unsafe extern "C" fn game_attack11(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 2.0, 3.0);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("legl"), 2.5, 60, 100, 30, 1, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneel"), 2.5, 82, 100, 30, 1, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("kneel"), 2.5, 82, 100, 30, 1, 3.5, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 3, 0, Hash40::new("top"), 2.5, 361, 15, 0, 25, 3.0, 0.0, 3.0, 4.0, Some(0.0), Some(3.0), Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_down_only(boma, 3, false);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}

unsafe extern "C" fn expression_attack11(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_R, 4);
    }
    frame(lua_state, 4.5);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 6.5);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(lua_state, 35.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 13);
    }
}

unsafe extern "C" fn game_attack12(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 4.0, 3.0);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("legr"), 3.0, 60, 100, 30, 1, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneer"), 3.0, 75, 100, 30, 1, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("kneer"), 3.0, 75, 100, 30, 1, 3.5, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 3, 0, Hash40::new("top"), 3.0, 361, 15, 0, 25, 3.0, 0.0, 3.0, 4.0, Some(0.0), Some(3.0), Some(14.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_down_only(boma, 3, false);
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}

unsafe extern "C" fn game_attack13(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 5.0/(6.5-1.0));
    }
    frame(lua_state, 6.5);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0/(7.0-6.5));
        ATTACK(agent, 0, 0, Hash40::new("swordl1"), 5.0, 361, 85, 0, 55, 4.0, 1.0, 0.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("swordl1"), 5.0, 361, 85, 0, 55, 4.0, 1.5, 0.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        //ATTACK(agent, 2, 0, Hash40::new("swordl1"), 6.0, 75, 50, 0, 70, 4.0, -3.0, 0.0, -1.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("swordl1"), 7.0, 361, 80, 0, 55, 4.0, -3.0, 0.0, -1.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        // Blade sourspot
        ATTACK(agent, 1, 0, Hash40::new("swordl1"), 5.0, 361, 85, 0, 55, 4.0, 6.0, 0.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        FT_MOTION_RATE(agent, 1.0);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 16.0/(36.0-16.0));
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
    }
}

unsafe extern "C" fn effect_attack13(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("edge_attack_dash"), Hash40::new("swordl1"), -3.0, 0, -1.0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_RATE(agent, 1.25);
        EFFECT_FOLLOW(agent, Hash40::new("edge_attack_dash_aura"), Hash40::new("swordl1"), -3.0, 0, -1.0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_RATE(agent, 1.25);
    }
    frame(lua_state, 6.5);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(agent, 1.4);
        if PostureModule::lr(boma) > 1.0{
            EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 4, 10, -11, 2, -12, 0, 0.45, true);
            LAST_EFFECT_SET_ALPHA(agent, 0.5);
        }
        else{
            EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 4, 10, -12, 2, -12, 0, 0.55, true);
            LAST_EFFECT_SET_ALPHA(agent, 0.8);
            LAST_EFFECT_SET_RATE(agent, 1.2);
        }
        EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 3, 8, -2, 1.2, -12, 0, 0.7, true);
        EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), -0.5, 9, 13, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 1.8);
        EFFECT_FOLLOW(agent, Hash40::new("edge_attack_line"), Hash40::new("top"), 6.2, 10.5, -20.8, 2, -12, 0, 1.03, true);
        LAST_EFFECT_SET_ALPHA(agent, 1.5);

        EFFECT_FOLLOW(agent, Hash40::new("edge_attack_dash2"), Hash40::new("swordl1"), -3.0, 0, -1.0, 0, 0, 0, 0.65, true);
        LAST_EFFECT_SET_RATE(agent, 0.75);
    }
}

unsafe extern "C" fn game_attackdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.8);
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.5);
    frame(lua_state, 11.0);
    FT_MOTION_RATE(agent, 1);
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 10.0, 361, 70, 0, 67, 3.5, -1.5, 0.0, 0.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        ATTACK(agent, 1, 0, Hash40::new("armr"), 10.0, 361, 70, 0, 67, 4.0, -1.0, 0.0, 0.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        ATTACK(agent, 2, 0, Hash40::new("armr"), 13.0, 53, 82, 0, 67, 6.0, 5.0, 0.0, 0.0, None, None, None, 1.15, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 7.0, 361, 82, 0, 66, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        ATTACK(agent, 1, 0, Hash40::new("armr"), 7.0, 361, 82, 0, 66, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        ATTACK(agent, 2, 0, Hash40::new("armr"), 7.0, 361, 82, 0, 66, 5.0, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attack11", game_attack11);
    agent.acmd("expression_attack11", expression_attack11);
    agent.acmd("game_attack12", game_attack12);
    agent.acmd("game_attack13", game_attack13);
    agent.acmd("effect_attack13", effect_attack13);
    
    agent.acmd("game_attackdash", game_attackdash);
}
