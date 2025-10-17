use super::*;

unsafe extern "C" fn game_specialsheading(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_NONE);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        // 0 is COLLISION_CATEGORY_MASK_NO_FIGHTER
        ATTACK(agent, 0, 0, Hash40::new("head"), 12.0, 295, 87, 0, 26, 7.0, 2.0, 1.5, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
        ATTACK(agent, 1, 0, Hash40::new("head"), 12.0, 295, 64, 0, 32, 5.7, 2.0, 1.5, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        // 0 is COLLISION_CATEGORY_MASK_NO_FIGHTER
        ATTACK(agent, 0, 0, Hash40::new("head"), 10.0, 295, 71, 0, 30, 5.0, 2.0, 1.5, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
        ATTACK(agent, 1, 0, Hash40::new("head"), 10.0, 295, 64, 0, 32, 4.0, 2.0, 1.5, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ON_DROP);
    }
}

unsafe extern "C" fn game_specialhistart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        boma.select_cliff_hangdata_from_name("special_hi");
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_WIIFIT_GENERATE_ARTICLE_HULAHOOP, false, -1);
    }
}

unsafe extern "C" fn game_specialhijump(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        boma.select_cliff_hangdata_from_name("special_hi");
    }
}

unsafe extern "C" fn game_specialhiend(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        boma.select_cliff_hangdata_from_name("special_hi");
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        ArticleModule::shoot_exist(boma, *FIGHTER_WIIFIT_GENERATE_ARTICLE_HULAHOOP, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialsheading", game_specialsheading, Priority::Low);

    agent.acmd("game_specialhistart", game_specialhistart, Priority::Low);
    agent.acmd("game_specialairhistart", game_specialhistart, Priority::Low);
    agent.acmd("game_specialhijump", game_specialhijump, Priority::Low);
    agent.acmd("game_specialhiend", game_specialhiend, Priority::Low);
}