use super::*;

unsafe extern "C" fn game_specialsmove(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if agent.is_situation(*SITUATION_KIND_GROUND) {
            VarModule::on_flag(agent.battle_object, vars::pacman::instance::SPECIAL_S_GROUND_START);
        }
        ATTACK(agent, 0, 0, Hash40::new("pizzapacman"), 1.0, 45, 90, 0, 30, 5.0, 0.0, 2.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(agent, 1, 0, Hash40::new("pizzapacman"), 10.0, 45, 90, 0, 30, 5.0, 0.0, 2.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
}

unsafe extern "C" fn game_specialsdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if WorkModule::is_flag(boma, *FIGHTER_PACMAN_STATUS_SPECIAL_S_WORK_FLAG_EAT_POWER_ESA) {
            ATTACK(agent, 0, 0, Hash40::new("pizzapacman"), 10.0, 361, 66, 0, 40, 6.0, 0.0, 2.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        }
        else {
            ATTACK(agent, 0, 0, Hash40::new("pizzapacman"), 1.0, 50, 108, 0, 50, 6.0, 0.0, 2.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        }
    }
}

unsafe extern "C" fn expression_specialairsreturn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
        ItemModule::set_have_item_visibility(boma, false, 0);
        ItemModule::set_attach_item_visibility(boma, false, *ATTACH_ITEM_GROUP_ALL as u8);
        VisibilityModule::set_int64(boma, hash40("body") as i64, hash40("body_none") as i64);
        VisibilityModule::set_int64(boma, hash40("pizza") as i64, hash40("pizza_normal") as i64);
        HIT_NODE(agent, Hash40::new("waist"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("handr"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("handl"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_OFF);
        HIT_NODE(agent, Hash40::new("pizzapacman"), *HIT_STATUS_NORMAL);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::pacman::instance::SPECIAL_S_GROUND_START) {
            ItemModule::set_have_item_visibility(boma, true, 0);
            ItemModule::set_attach_item_visibility(boma, true, *ATTACH_ITEM_GROUP_ALL as u8);
            VisibilityModule::set_int64(boma, hash40("body") as i64, hash40("body_normal") as i64);
            VisibilityModule::set_int64(boma, hash40("pizza") as i64, hash40("pizza_none") as i64);
            HIT_NODE(agent, Hash40::new("waist"), *HIT_STATUS_NORMAL);
            HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_NORMAL);
            HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
            HIT_NODE(agent, Hash40::new("handr"), *HIT_STATUS_NORMAL);
            HIT_NODE(agent, Hash40::new("handl"), *HIT_STATUS_NORMAL);
            HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_NORMAL);
            HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_NORMAL);
            HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
            HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
        }
    }
}

unsafe extern "C" fn game_specialhistart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if !WorkModule::is_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_TRAMPOLINE_JUMP) {
            ArticleModule::generate_article(boma, *FIGHTER_PACMAN_GENERATE_ARTICLE_TRAMPOLINE, false, -1);
        }
    }
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 3.0, 4.0);
    frame(lua_state, 3.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specialairhiloop(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("pizzapacman"), 5.0, 90, 90, 0, 40, 5.0, -0.5, 2.0, 0.0, Some(0.5), Some(2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 5.0);
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("pizzapacman"), 6.0, 82, 95, 0, 40, 4.4, -0.5, 2.0, 0.0, Some(0.5), Some(2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("pizzapacman"), 7.0, 60, 70, 0, 40, 3.8, -0.5, 2.0, 0.0, Some(0.5), Some(2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::pacman::status::SPECIAL_HI_AERIAL);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_specialairhiend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        boma.select_cliff_hangdata_from_name("special_hi");
        VarModule::on_flag(agent.battle_object, vars::pacman::status::SPECIAL_HI_AERIAL);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::pacman::status::SPECIAL_HI_AERIAL);
    }
}

unsafe extern "C" fn game_speciallwfailure(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 6.0, 13.0);
    frame(lua_state, 6.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 7.5, 270, 60, 0, 35, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KAMEHIT, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 7.5, 270, 70, 0, 20, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KAMEHIT, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn effect_speciallwfailure(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_RATE(agent, 1.2);
    }
}

unsafe extern "C" fn sound_speciallwfailure(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 6.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_punch_kick_swing_s"));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialsmove", game_specialsmove, Priority::Low);
    agent.acmd("game_specialsdash", game_specialsdash, Priority::Low);

    agent.acmd("expression_specialairsreturn", expression_specialairsreturn, Priority::Low);

    agent.acmd("game_specialhistart", game_specialhistart, Priority::Low);
    agent.acmd("game_specialairhistart", game_specialhistart, Priority::Low);
    agent.acmd("game_specialairhiloop", game_specialairhiloop, Priority::Low);
    agent.acmd("game_specialairhiend", game_specialairhiend, Priority::Low);

    agent.acmd("game_speciallwfailure", game_speciallwfailure, Priority::Low);
    agent.acmd("game_specialairlwfailure", game_speciallwfailure, Priority::Low);
    agent.acmd("effect_speciallwfailure", effect_speciallwfailure, Priority::Low);
    agent.acmd("effect_specialairlwfailure", effect_speciallwfailure, Priority::Low);
    agent.acmd("sound_speciallwfailure", sound_speciallwfailure, Priority::Low);
    agent.acmd("sound_specialairlwfailure", sound_speciallwfailure, Priority::Low);
}