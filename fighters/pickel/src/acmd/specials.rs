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
        WorkModule::on_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        let mut pearl_active = false;
        if ArticleModule::is_exist(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_TROLLEY) {
            let article = ArticleModule::get_article(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_TROLLEY);
            let article_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
            let article_boma = sv_battle_object::module_accessor(article_id);
            if StatusModule::status_kind(article_boma) == WEAPON_PICKEL_TROLLEY_STATUS_KIND_PEARL_FLY {
                pearl_active = true;
            }
        }
        if !pearl_active
        && agent.get_int(*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_GOLD) >=1 {
            ArticleModule::generate_article(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_TROLLEY, false, -1);
            ArticleModule::change_status(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_TROLLEY, WEAPON_PICKEL_TROLLEY_STATUS_KIND_PEARL_FLY, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST));
            FighterSpecializer_Pickel::sub_material_num(boma, *FIGHTER_PICKEL_MATERIAL_KIND_GOLD, 1);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("sound_specialn1getgold", sound_specialn1getgold, Priority::Low);

    agent.acmd("game_specialsride", game_specialsride, Priority::Low);

    agent.acmd("game_specialsstart", game_specialsstart, Priority::Low);
    agent.acmd("game_specialairsstart", game_specialsstart, Priority::Low);
}
