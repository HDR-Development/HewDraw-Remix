use super::*;

use vars::pickel::{
    instance::*,
    status::*
};

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
    FT_MOTION_RATE(agent, 0.75);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 350, 100, 30, 10, 3.0, 0.0, 8.0, 4.5, Some(0.0), Some(4.0), Some(4.5), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
}

unsafe extern "C" fn game_specialsstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    frame(lua_state, 7.0);
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
        && agent.get_int(*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_GOLD) >=1
        && VarModule::get_int(boma.object(), PEARL_COOLDOWN) == 0 {
            VarModule::on_flag(boma.object(), SPECIAL_S_THROW_PEARL);
        }
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), SPECIAL_S_THROW_PEARL) {
            ArticleModule::generate_article(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_TROLLEY, false, -1);
            ArticleModule::change_status(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_TROLLEY, WEAPON_PICKEL_TROLLEY_STATUS_KIND_PEARL_FLY, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST));
            // re-imburse steve the 1 iron it costs to generate the trolley article
            FighterSpecializer_Pickel::add_material_num(boma, *FIGHTER_PICKEL_MATERIAL_KIND_IRON, 1);
            // set cooldown
            VarModule::set_int(boma.object(), PEARL_COOLDOWN, 90);
        }
    }
}

unsafe extern "C" fn effect_specialsstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        if VarModule::is_flag(boma.object(), SPECIAL_S_THROW_PEARL) {
            EFFECT_FLIP(agent, Hash40::new("sys_flash"), Hash40::new("sys_flash"), Hash40::new("top"), -4.0, 9, 5, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
            LAST_EFFECT_SET_RATE(agent, 1.2);
        } else {
            EFFECT(agent, Hash40::new("pickel_erace_smoke"), Hash40::new("top"), -3, 8, 3.5, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
        }
    }
}

unsafe extern "C" fn sound_specialsstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_pickel_swing_s02"));
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        if !VarModule::is_flag(boma.object(), SPECIAL_S_THROW_PEARL) {
            PLAY_SE(agent, Hash40::new("se_pickel_special_s11"));
        }
    }
}

unsafe extern "C" fn expression_specialsstart(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        if !VarModule::is_flag(boma.object(), SPECIAL_S_THROW_PEARL) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
}

unsafe extern "C" fn game_specialairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("head"), 5.0, 70, 100, 0, 30, 2.6, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("head"), 3.0, 70, 100, 0, 30, 1.7, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 31.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 40.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 60.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_PICKEL_STATUS_SPECIAL_HI_FLAG_ENABLE_CANCEL);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("sound_specialn1getgold", sound_specialn1getgold, Priority::Low);

    agent.acmd("game_specialsride", game_specialsride, Priority::Low);

    agent.acmd("game_specialsstart", game_specialsstart, Priority::Low);
    agent.acmd("game_specialairsstart", game_specialsstart, Priority::Low);
    agent.acmd("effect_specialsstart", effect_specialsstart, Priority::Low);
    agent.acmd("effect_specialairsstart", effect_specialsstart, Priority::Low);
    agent.acmd("sound_specialsstart", sound_specialsstart, Priority::Low);
    agent.acmd("sound_specialairsstart", sound_specialsstart, Priority::Low);
    agent.acmd("expression_specialsstart", expression_specialsstart, Priority::Low);
    agent.acmd("expression_specialairsstart", expression_specialsstart, Priority::Low);

    agent.acmd("game_specialairhi", game_specialairhi, Priority::Low);
}