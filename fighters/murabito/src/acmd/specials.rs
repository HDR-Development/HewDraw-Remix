use super::*;

unsafe extern "C" fn game_speciallw1(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_MURABITO_STATUS_SPECIAL_LW_PLANT_FLAG_CHECK_PLANT);
        FT_MOTION_RATE(agent, 0.661);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_SEED, false, 0);
    }
    frame(lua_state, 18.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 72, 20, 0, 80, 4.0, 0.0, 3.0, 12.0, Some(0.0), Some(3.0), Some(3.5), 1.25, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        WorkModule::on_flag(boma, *FIGHTER_MURABITO_STATUS_SPECIAL_LW_PLANT_FLAG_PLANT);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

unsafe extern "C" fn game_speciallw1failure(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(agent, 0.75);
    frame(lua_state, 2.0);
    if is_excute(agent) {
        ArticleModule::generate_article_enable(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_SEED, false, -1);
    }
    frame(lua_state, 55.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_MURABITO_GENERATE_ARTICLE_SEED, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn game_specialairlw1failure(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 14.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 0.4);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 78, 62, 0, 70, 5.0, 0.0, -6.0, 1.0, Some(0.0), Some(-1.5), Some(1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 63.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 64.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.0);
    }
}

unsafe extern "C" fn effect_specialairlw1failure(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), -2, 5, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 1, -7, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true, 0.6);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
}

unsafe extern "C" fn sound_specialairlw1failure(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 12.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_murabito_attackair_l01"));
    }
}

unsafe extern "C" fn expression_specialairlw1failure(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 2.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("item") as i64, hash40("item_shovel") as i64);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
    frame(lua_state, 69.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("item") as i64, hash40("item_none") as i64);
    }
}

unsafe extern "C" fn game_speciallw3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 14.0, 361, 100, 0, 36, 5.0, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 23, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 14.0, 361, 100, 0, 36, 3.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 23, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 2, 0, Hash40::new("top"), 14.0, 361, 100, 0, 36, 5.0, 0.0, 5.5, 11.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 23, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 3, 0, Hash40::new("top"), 14.0, 361, 100, 0, 36, 3.0, 0.0, 6.0, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 23, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn game_speciallw3hit(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if !ItemModule::is_have_item(boma, 0){
            ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_WOOD), 0, 0, false, false);
        }
    }
}

unsafe extern "C" fn game_specialairlw3hit(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    if is_excute(agent) {
        if !ItemModule::is_have_item(boma, 0){
            ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_WOOD), 0, 0, false, false);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_speciallw1", game_speciallw1, Priority::Low);

    agent.acmd("game_speciallw1failure", game_speciallw1failure, Priority::Low);
    agent.acmd("game_specialairlw1failure", game_specialairlw1failure, Priority::Low);
    agent.acmd("effect_specialairlw1failure", effect_specialairlw1failure, Priority::Low);
    agent.acmd("sound_specialairlw1failure", sound_specialairlw1failure, Priority::Low);
    agent.acmd("expression_specialairlw1failure", expression_specialairlw1failure, Priority::Low);
    
    agent.acmd("game_speciallw3", game_speciallw3, Priority::Low);
    agent.acmd("game_specialairlw3", game_speciallw3, Priority::Low);
    agent.acmd("game_speciallw3hit", game_speciallw3hit, Priority::Low);
    agent.acmd("game_specialairlw3hit", game_specialairlw3hit, Priority::Low);
}
