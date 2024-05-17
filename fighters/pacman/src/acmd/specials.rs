use super::*;

unsafe extern "C" fn expression_specialairsreturn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
    if is_excute(agent) {
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
}

unsafe extern "C" fn game_specialhistart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        if !WorkModule::is_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_TRAMPOLINE_JUMP) {
            ArticleModule::generate_article(boma, *FIGHTER_PACMAN_GENERATE_ARTICLE_TRAMPOLINE, false, -1);
        }
    }
}

unsafe extern "C" fn game_specialairhiloop(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("pizzapacman"), 5.0, 90, 90, 0, 40, 5.0, -0.5, 2.0, 0.0, Some(0.5), Some(2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 9.0);
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("pizzapacman"), 6.0, 86, 95, 0, 40, 4.4, -0.5, 2.0, 0.0, Some(0.5), Some(2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 1, 0, Hash40::new("pizzapacman"), 7.0, 60, 70, 0, 40, 3.8, -0.5, 2.0, 0.0, Some(0.5), Some(2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 16.0);
    if is_excute(agent) {
        VarModule::on_flag(agent.battle_object, vars::pacman::status::TRAMPOLINE_AERIAL);
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
        VarModule::on_flag(agent.battle_object, vars::pacman::status::TRAMPOLINE_AERIAL);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        VarModule::off_flag(agent.battle_object, vars::pacman::status::TRAMPOLINE_AERIAL);
    }
}

unsafe extern "C" fn game_speciallwfailure(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 2.4);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
        ATTACK(agent, 0, 0, Hash40::new("top"), 7.5, 270, 70, 0, 20, 10.0, 0.0, 3.5, 0.0, None, None, None, 1.3, 1.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("expression_specialairsreturn", expression_specialairsreturn, Priority::Low);

    agent.acmd("game_specialhistart", game_specialhistart, Priority::Low);
    agent.acmd("game_specialairhistart", game_specialhistart, Priority::Low);
    agent.acmd("game_specialairhiloop", game_specialairhiloop, Priority::Low);
    agent.acmd("game_specialairhiend", game_specialairhiend, Priority::Low);

    agent.acmd("game_speciallwfailure", game_speciallwfailure, Priority::Low);
    agent.acmd("game_specialairlwfailure", game_speciallwfailure, Priority::Low);
}