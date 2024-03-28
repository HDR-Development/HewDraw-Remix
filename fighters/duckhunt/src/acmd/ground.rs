use super::*;

unsafe extern "C" fn game_attack11(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();

    frame(lua_state, 4.0);
    if is_excute(agent) {
        ATTACK(agent,  0,  0,  Hash40::new("top"),  2.0,  70,  30,  0,  20,  3.0,  0.0,  4.0,  6.0,  None,  None,  None,  1.0,  1.0,  *ATTACK_SETOFF_KIND_ON,  *ATTACK_LR_CHECK_F,  false,  0,  0.0,  0,  false,  false,  false,  false,  true,  *COLLISION_SITUATION_MASK_GA_d,  *COLLISION_CATEGORY_MASK_ALL,  *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"),  *ATTACK_SOUND_LEVEL_S,  *COLLISION_SOUND_ATTR_PUNCH,  *ATTACK_REGION_PUNCH); 
        ATTACK(agent,  1,  0,  Hash40::new("top"),  2.0,  70,  30,  0,  20,  3.0,  0.0,  4.0,  8.5,  None,  None,  None,  1.0,  1.0,  *ATTACK_SETOFF_KIND_ON,  *ATTACK_LR_CHECK_F,  false,  0,  0.0,  0,  false,  false,  false,  false,  true,  *COLLISION_SITUATION_MASK_GA_d,  *COLLISION_CATEGORY_MASK_ALL,  *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"),  *ATTACK_SOUND_LEVEL_S,  *COLLISION_SOUND_ATTR_PUNCH,  *ATTACK_REGION_PUNCH); 
        ATTACK(agent,  2,  0,  Hash40::new("top"),  2.0,  70,  30,  0,  20,  3.0,  0.0,  4.0,  11.5,  None,  None,  None,  1.0,  1.0,  *ATTACK_SETOFF_KIND_ON,  *ATTACK_LR_CHECK_F,  false,  0,  0.0,  0,  false,  false,  false,  false,  true,  *COLLISION_SITUATION_MASK_GA_d,  *COLLISION_CATEGORY_MASK_FIGHTER,  *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"),  *ATTACK_SOUND_LEVEL_S,  *COLLISION_SOUND_ATTR_PUNCH,  *ATTACK_REGION_PUNCH);
        //Locking hitboxes
        ATTACK(agent, 3, 0, Hash40::new("top"), 2.0, 361, 30, 0, 20, 3.0,  0.0,  4.0,  6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 4, 0, Hash40::new("top"), 2.0, 361, 20, 0, 15, 3.0,  0.0,  4.0,  8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 5, 0, Hash40::new("top"), 2.0, 361, 20, 0, 15, 3.0,  0.0,  4.0,  11.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);

    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma,  *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    } 
}

unsafe extern "C" fn game_attack12(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        ATTACK(agent,  0,  0,  Hash40::new("top"),  2.0,  70,  30,  0,  30,  3.0,  0.0,  3.5,  2.0,  None,  None,  None,  1.0,  1.0,  *ATTACK_SETOFF_KIND_ON,  *ATTACK_LR_CHECK_F,  false,  0,  0.0,  0,  false,  false,  false,  false,  true,  *COLLISION_SITUATION_MASK_GA_d,  *COLLISION_CATEGORY_MASK_ALL,  *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"),  *ATTACK_SOUND_LEVEL_S,  *COLLISION_SOUND_ATTR_PUNCH,  *ATTACK_REGION_HEAD);
        ATTACK(agent,  1,  0,  Hash40::new("top"),  2.0,  70,  25,  0,  25,  5.0,  0.0,  7.5,  5.5,  None,  None,  None,  1.2,  1.0,  *ATTACK_SETOFF_KIND_ON,  *ATTACK_LR_CHECK_F,  false,  0,  0.0,  0,  false,  false,  false,  false,  true,  *COLLISION_SITUATION_MASK_GA_d,  *COLLISION_CATEGORY_MASK_ALL,  *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"),  *ATTACK_SOUND_LEVEL_S,  *COLLISION_SOUND_ATTR_PUNCH,  *ATTACK_REGION_HEAD);
        //Locking hitboxes
        ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 361, 30, 0, 20, 3.0,  0.0,  3.5,  2.0,  None,  None,  None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 3, 0, Hash40::new("top"), 2.0, 361, 20, 0, 15, 5.0,  0.0,  7.5,  5.5,  None,  None,  None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
    }
}

unsafe extern "C" fn game_attack13(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ATTACK(agent,  0,  0,  Hash40::new("top"),  5.0,  55,  70,  0,  55,  4.0,  0.0,  4.5,  4.5,  None, None, None,  1.0,  1.0,  *ATTACK_SETOFF_KIND_ON,  *ATTACK_LR_CHECK_F,  false,  0,  0.0,  0,  false,  false,  false,  false,  true,  *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL,  *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"),  *ATTACK_SOUND_LEVEL_M,  *COLLISION_SOUND_ATTR_KICK,  *ATTACK_REGION_KICK);
        ATTACK(agent,  1,  0,  Hash40::new("top"),  5.0,  55,  70,  0,  55,  4.0,  0.0,  4.5,  9.0,  None, None, None,  1.0,  1.0,  *ATTACK_SETOFF_KIND_ON,  *ATTACK_LR_CHECK_F,  false,  0,  0.0,  0,  false,  false,  false,  false,  true,  *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL,  *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"),  *ATTACK_SOUND_LEVEL_M,  *COLLISION_SOUND_ATTR_KICK,  *ATTACK_REGION_KICK);
    }
    wait(lua_state, 3.0);
    FT_MOTION_RATE_RANGE(agent, 6.0, 28.0, 30.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_attack100sub(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 4.5, 0.5);
        ATTACK(agent,  0,  0,  Hash40::new("top"),  0.4,  361,  15,  0,  16,  4.5,  0.0,  7.5,  4.5,  None,  None,  None,  0.5,  0.4,  *ATTACK_SETOFF_KIND_OFF,  *ATTACK_LR_CHECK_F,  false,  0,  0.0,  0,  false,  false,  false,  false,  true,  *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL,  *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"),  *ATTACK_SOUND_LEVEL_S,  *COLLISION_SOUND_ATTR_CUTUP,  *ATTACK_REGION_HEAD);
        ATTACK(agent,  1,  0,  Hash40::new("top"),  0.4,  361,  10,  0,  12,  6.5,  0.0,  7.5,  14.0,  Some(0.0),  Some(7.5),  Some(6.5),  0.5,  0.4,  *ATTACK_SETOFF_KIND_OFF,  *ATTACK_LR_CHECK_F,  false,  0,  0.0,  0,  false,  false,  false,  false,  true,  *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL,  *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_cutup"),  *ATTACK_SOUND_LEVEL_S,  *COLLISION_SOUND_ATTR_CUTUP,  *ATTACK_REGION_HEAD);
        AttackModule::set_add_reaction_frame(boma,  0,  2.0,  false);
        AttackModule::set_add_reaction_frame(boma,  1,  2.0,  false);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1, 5.0);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma,  *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
    }
}

unsafe extern "C" fn game_attack100end(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 4.5, 5.0);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        ATTACK(agent,  0,  0,  Hash40::new("top"),  3.0,  80,  140,  0,  70,  5.5,  0.0,  6.5,  7.0,  None,  None,  None,  2.0,  1.0,  *ATTACK_SETOFF_KIND_OFF,  *ATTACK_LR_CHECK_F,  false,  0,  0.0,  0,  false,  false,  false,  false,  true,  *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL,  *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"),  *ATTACK_SOUND_LEVEL_L,  *COLLISION_SOUND_ATTR_PUNCH,  *ATTACK_REGION_HEAD);
        ATTACK(agent,  1,  0,  Hash40::new("top"),  3.0,  80,  140,  0,  70,  5.5,  0.0,  6.5,  12.0,  None,  None,  None,  2.0,  1.0,  *ATTACK_SETOFF_KIND_OFF,  *ATTACK_LR_CHECK_F,  false,  0,  0.0,  0,  false,  false,  false,  false,  true,  *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL,  *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"),  *ATTACK_SOUND_LEVEL_L,  *COLLISION_SOUND_ATTR_PUNCH,  *ATTACK_REGION_HEAD);
        ATTACK(agent,  2,  0,  Hash40::new("top"),  3.0,  80,  140,  0,  70,  5.5,  0.0,  6.5,  17.0,  None,  None,  None,  2.0,  1.0,  *ATTACK_SETOFF_KIND_OFF,  *ATTACK_LR_CHECK_F,  false,  0,  0.0,  0,  false,  false,  false,  false,  true,  *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL,  *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"),  *ATTACK_SOUND_LEVEL_L,  *COLLISION_SOUND_ATTR_PUNCH,  *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent,  36.3/(39.0-6.0));
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_attackdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.8);
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 6.0/(10.0-1.0));
    frame(lua_state, 10.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 60, 40, 0, 100, 4.0, 0.0, 7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 60, 40, 0, 100, 4.0, 0.0, 7.0, 4.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 10.0, 60, 40, 0, 100, 4.0, 0.0, 7.0, 9.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 105, 40, 0, 90, 3.5, 0.0, 7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 105, 40, 0, 90, 3.5, 0.0, 7.0, 4.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(agent, 2, 0, Hash40::new("top"), 7.0, 105, 40, 0, 90, 3.5, 0.0, 7.0, 9.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attack11", game_attack11);
    agent.acmd("game_attack12", game_attack12);
    agent.acmd("game_attack13", game_attack13);
    agent.acmd("game_attack100sub", game_attack100sub);
    agent.acmd("game_attack100end", game_attack100end);

    agent.acmd("game_attackdash", game_attackdash);
}
