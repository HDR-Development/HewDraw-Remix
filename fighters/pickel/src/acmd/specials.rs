use super::*;

unsafe extern "C" fn sound_specialn1getgold(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_pickel_special_n02_iron"));
        PLAY_SE(agent, Hash40::new("se_pickel_special_n_item"));
        PLAY_SE(agent, Hash40::new("se_result_coin_silver"));
    }
}

unsafe extern "C" fn game_specialsride(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 0.5);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 350, 100, 30, 10, 3.0, 0.0, 8.0, 4.5, Some(0.0), Some(4.0), Some(4.5), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
}

unsafe extern "C" fn game_specialsstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        //if !ArticleModule::is_exist(boma, FIGHTER_PICKEL_GENERATE_ARTICLE_ENDERPEARL) {
            ArticleModule::generate_article(boma, FIGHTER_PICKEL_GENERATE_ARTICLE_ENDERPEARL, false, -1);
        //}
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("sound_specialn1getgold", sound_specialn1getgold, Priority::Low);

    agent.acmd("game_specialsride", game_specialsride, Priority::Low);

    agent.acmd("game_specialsstart", game_specialsstart, Priority::Low);
    agent.acmd("game_specialairsstart", game_specialsstart, Priority::Low);
}
