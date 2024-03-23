use super::*;

unsafe extern "C" fn game_specialsstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_MOVE_FRONT);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_MOVE_FRONT);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 2.0, 0.0, 12.0, 9.0, Some(0.0), Some(4.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_search"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        WorkModule::on_flag(boma, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_HIT_CHECK_ONOFF);
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_PIT_REFLECTOR_KIND_SPECIAL_S, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_S);
        WorkModule::on_flag(boma, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_CLIFF_FALL_ONOFF);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_GRAVITY_ONOFF);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_MTRANS_AIR_UNABLE);
        AttackModule::clear_all(boma);
        WorkModule::off_flag(boma, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_HIT_CHECK_ONOFF);
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_PIT_REFLECTOR_KIND_SPECIAL_S, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_S);
        // WorkModule::off_flag(boma, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_CLIFF_FALL_ONOFF);
    }
}

unsafe extern "C" fn game_specialsend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_PIT_REFLECTOR_KIND_SPECIAL_S, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_S);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 40, 80, 0, 63, 6.0, 0.0, 4.0, 9.0, Some(0.0), Some(10.0), Some(9.0), 1.15, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_PIT_REFLECTOR_KIND_SPECIAL_S, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_S);
    }
}

unsafe extern "C" fn game_specialairsstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        boma.select_cliff_hangdata_from_name("special_s");
        WorkModule::on_flag(boma, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_MOVE_FRONT);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 2.0, 0.0, 14.0, 9.0, Some(0.0), Some(4.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_search"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        WorkModule::on_flag(boma, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_HIT_CHECK_ONOFF);
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_PIT_REFLECTOR_KIND_SPECIAL_S, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_S);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_GRAVITY_ONOFF);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_HIT_CHECK_ONOFF);
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_PIT_REFLECTOR_KIND_SPECIAL_S, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_S);
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 0.667);
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("bust"), 0.5, 367, 100, 0, 35, 6.0, 0.0, -3.0, -2.0, None, None, None, 0.5, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("bust"), 7.0, 68, 110, 0, 60, 6.5, 0.0, -3.0, -2.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 45.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.000);
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLAG_FIX_ANGLE);
        WorkModule::on_flag(boma, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLAG_BACK_ANGLE);
        JostleModule::set_status(boma, true);
    }
    
}

unsafe extern "C" fn game_speciallwstartl(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 6.0, 4.0);
    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 69, 85, 0, 77, 7.5, 0.0, 7.0, -1.5, Some(0.0), Some(7.0), Some(1.5), 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_NONE);
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_LW);
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, 1, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_LW);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_speciallwstartl(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        let facing = PostureModule::lr(boma);
        let rot1 = if facing > 0.0 { 240 } else { 300 };
        let rot2 = if facing > 0.0 { 120 } else { 60 };
        EFFECT_FOLLOW(agent, Hash40::new("pitb_guardian_shield"), Hash40::new("virtualguardianf"), 2.0 * facing, 3, -2, 0, rot1, 0, 1.3, true);
        EFFECT_FOLLOW(agent, Hash40::new("pitb_guardian_shield"), Hash40::new("virtualguardianb"), 2.0 * facing, 3, 2, 0, rot2, 0, 1.3, true);
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

unsafe extern "C" fn game_speciallwhold(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_LW);
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, 1, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_LW);
    }
    
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialsstart", game_specialsstart);
    agent.acmd("game_specialsend", game_specialsend);
    agent.acmd("game_specialairsstart", game_specialairsstart);
    agent.acmd("game_specialhi", game_specialhi);
    agent.acmd("game_speciallwstartl", game_speciallwstartl);
    agent.acmd("game_speciallwstartr", game_speciallwstartl);
    agent.acmd("game_specialairlwstartl", game_speciallwstartl);
    agent.acmd("game_specialairlwstartr", game_speciallwstartl);
    agent.acmd("effect_speciallwstartl", effect_speciallwstartl);
    agent.acmd("effect_speciallwstartr", effect_speciallwstartl);
    agent.acmd("effect_specialairlwstartl", effect_speciallwstartl);
    agent.acmd("effect_specialairlwstartr", effect_speciallwstartl);
    agent.acmd("game_speciallwhold", game_speciallwhold);
    agent.acmd("game_specialairlwhold", game_speciallwhold);
}