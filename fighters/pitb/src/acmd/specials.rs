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
        ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 40, 80, 0, 57, 6.0, 0.0, 4.0, 9.0, Some(0.0), Some(10.0), Some(9.0), 1.15, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 26, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
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
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
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
    frame(lua_state, 42.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP);
    }
}

unsafe extern "C" fn game_specialhistart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        boma.select_cliff_hangdata_from_name("special_hi_start");
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
    }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("bust"), 14.0, 75, 83, 0, 43, 5.0, 0.0, -3.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("bust"), 8.0, 75, 73, 0, 58, 5.0, 0.0, -3.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 43.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLAG_FIX_ANGLE);
        WorkModule::on_flag(boma, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLAG_BACK_ANGLE);
    }
    frame(lua_state, 48.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_speciallwstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 6.0, 3.0);
    frame(lua_state, 6.0);
    FT_MOTION_RATE_RANGE(agent, 6.0, 6.9, 1.0);
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_LW);
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, 1, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_LW);
    }
    frame(lua_state, 6.9);
    FT_MOTION_RATE_RANGE(agent, 6.9, 7.0, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 70, 85, 0, 77, 7.5, 0.0, 7.0, -1.5, Some(0.0), Some(7.0), Some(1.5), 0.72, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 6.933);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 7.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_speciallwstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        let facing = PostureModule::lr(boma);
        let rot1 = if facing > 0.0 { 240 } else { 300 };
        let rot2 = if facing > 0.0 { 120 } else { 60 };
        EFFECT_FOLLOW(agent, Hash40::new("pitb_guardian_shield"), Hash40::new("virtualguardianf"), 2.0 * facing, 3, -2, 0, rot1, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(agent, 5.0, 1.0, 1.0);
        EFFECT_FOLLOW(agent, Hash40::new("pitb_guardian_shield"), Hash40::new("virtualguardianb"), 2.0 * facing, 3, 2, 0, rot2, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(agent, 5.0, 1.0, 1.0);
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

unsafe extern "C" fn sound_speciallwstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_pitb_special_l01"));
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

unsafe extern "C" fn effect_speciallwend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        let facing = PostureModule::lr(boma);
        let rot1 = if facing > 0.0 { 240 } else { 300 };
        let rot2 = if facing > 0.0 { 120 } else { 60 };
        EFFECT(agent, Hash40::new("pitb_guardian_shield_end"), Hash40::new("virtualguardianf"), 2.0 * facing, 3, -2, 0, rot1, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("pitb_guardian_shield_end"), Hash40::new("virtualguardianb"), 2.0 * facing, 3, 2, 0, rot2, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("pitb_guardian_shield"), false, false);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialsstart", game_specialsstart, Priority::Low);
    agent.acmd("game_specialsend", game_specialsend, Priority::Low);
    agent.acmd("game_specialairsstart", game_specialairsstart, Priority::Low);

    agent.acmd("game_specialhistart", game_specialhistart, Priority::Low);
    agent.acmd("game_specialairhistart", game_specialhistart, Priority::Low);
    agent.acmd("game_specialhi", game_specialhi, Priority::Low);
    
    agent.acmd("game_speciallwstartl", game_speciallwstart, Priority::Low);
    agent.acmd("game_speciallwstartr", game_speciallwstart, Priority::Low);
    agent.acmd("game_specialairlwstartl", game_speciallwstart, Priority::Low);
    agent.acmd("game_specialairlwstartr", game_speciallwstart, Priority::Low);
    agent.acmd("effect_speciallwstartl", effect_speciallwstart, Priority::Low);
    agent.acmd("effect_speciallwstartr", effect_speciallwstart, Priority::Low);
    agent.acmd("effect_specialairlwstartl", effect_speciallwstart, Priority::Low);
    agent.acmd("effect_specialairlwstartr", effect_speciallwstart, Priority::Low);
    agent.acmd("sound_speciallwstartl", sound_speciallwstart, Priority::Low);
    agent.acmd("sound_speciallwstartr", sound_speciallwstart, Priority::Low);
    agent.acmd("sound_specialairlwstartl", sound_speciallwstart, Priority::Low);
    agent.acmd("sound_specialairlwstartr", sound_speciallwstart, Priority::Low);

    agent.acmd("game_speciallwhold", game_speciallwhold, Priority::Low);
    agent.acmd("game_specialairlwhold", game_speciallwhold, Priority::Low);

    agent.acmd("effect_speciallwendl", effect_speciallwend, Priority::Low);
    agent.acmd("effect_speciallwendr", effect_speciallwend, Priority::Low);
    agent.acmd("effect_specialairlwendl", effect_speciallwend, Priority::Low);
    agent.acmd("effect_specialairlwendr", effect_speciallwend, Priority::Low);
}