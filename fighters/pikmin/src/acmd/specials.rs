use super::*;

unsafe extern "C" fn game_specials(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_PIKMIN_STATUS_SPECIAL_S_FLAG_THROW);
    }
}

unsafe extern "C" fn game_specialnstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_PIKMIN_GENERATE_ARTICLE_PIKMIN, false, -1);
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, WorkModule::get_float(boma, *FIGHTER_PIKMIN_STATUS_PULL_OUT_WORK_FLOAT_MOT_RATE));
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.0);
    }
}

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_PIKMIN_STATUS_SPECIAL_LW_FLAG_SORT);
        // damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, hash40("top"), 7.5, 0.0, 7.0, -8.5, 0.0, 7.0, 8.5, 1.5, 1.0, 50, false, 0.8, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        // damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
}

unsafe extern "C" fn effect_speciallw(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 0.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("pikmin_order"), Hash40::new("s_antenna4"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_FLW_POS(agent, Hash40::new("pikmin_seiretsu"), Hash40::new("top"), 0, 3, 0, 0, 0, 0, 0.55, true);
        LAST_EFFECT_SET_RATE(agent, 2.0);
    }
}

unsafe extern "C" fn game_specialnfailure(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 8.5, 80, 85, 0, 45, 4.0, 0.0, 2.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.5, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specials", game_specials);
    agent.acmd("game_specialairs", game_specials);
    agent.acmd("game_specialnstart", game_specialnstart);
    agent.acmd("game_specialnfailure", game_specialnfailure);
    agent.acmd("game_specialairnfailure", game_specialnfailure);
    agent.acmd("game_speciallw", game_speciallw);
    agent.acmd("game_specialairlw", game_speciallw);
    agent.acmd("effect_speciallw", effect_speciallw);
    agent.acmd("effect_specialairlw", effect_speciallw);
}