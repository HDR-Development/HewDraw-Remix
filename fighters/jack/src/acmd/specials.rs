use super::*;

unsafe extern "C" fn game_specialairndown(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    for _ in 0..20 {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 50, 80, 0, 25, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-20.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 1, 0, Hash40::new("top"), 1.2, 45, 0, 0, 29, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-45.0), Some(-2.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 2, 0, Hash40::new("top"), 1.2, 55, 0, 0, 29, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-45.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            ATTACK(agent, 3, 0, Hash40::new("top"), 1.2, 45, 0, 0, 29, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-45.0), Some(2.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 5, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 5, 4);
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 3, 5, 4);
        }
        if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE){
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 50, 80, 0, 25, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-20.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                ATTACK(agent, 1, 0, Hash40::new("top"), 1.2, 45, 0, 0, 29, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-45.0), Some(-2.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                ATTACK(agent, 2, 0, Hash40::new("top"), 1.2, 55, 0, 0, 29, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-45.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                ATTACK(agent, 3, 0, Hash40::new("top"), 1.2, 45, 0, 0, 29, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-45.0), Some(2.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 5, 4);
                FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 5, 4);
                FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
                FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 3, 5, 4);
            }
        }
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_63_bullet"), 2, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 6.0);
        if WorkModule::is_flag(boma, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE){
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 1.6, 50, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-22.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                ATTACK(agent, 1, 0, Hash40::new("top"), 0.8, 45, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(-2.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                ATTACK(agent, 2, 0, Hash40::new("top"), 0.8, 55, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                ATTACK(agent, 3, 0, Hash40::new("top"), 0.8, 45, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(2.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 0, 5, 4);
                FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 1, 5, 4);
                FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(agent, 2, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
                FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(agent, 3, 5, 4);
            }
        }
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_63_bullet"), 2, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 6.0);
    }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, Hash40::new("special_hi"), false, 0.0);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, /*Flag*/ *FIGHTER_JACK_STATUS_SPECIAL_HI_FLAG_REVERSE_LR);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        ArticleModule::change_status(boma, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, *WEAPON_JACK_WIREROPE_STATUS_KIND_EXTEND, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        AreaModule::reset_area(boma, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH);
        ENABLE_AREA(agent, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH);
        AreaModule::reset_area(boma, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH_ADD);
        ENABLE_AREA(agent, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH_ADD);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        SEARCH(agent, 0, 0, Hash40::new("throw"), 3.0, 1.5, 1.5, 0.7, Some(1.5), Some(-15.0), Some(-6.4), *COLLISION_KIND_MASK_HSR, *HIT_STATUS_MASK_NORMAL, 0, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_BODY, false);
        SEARCH(agent, 1, 0, Hash40::new("throw"), 5.5, 1.5, 1.5, 0.7, None, None, None, *COLLISION_KIND_MASK_HSR, *HIT_STATUS_MASK_NORMAL, 0, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_BODY, false);
        SEARCH(agent, 2, 0, Hash40::new("throw"), 5.5, 1.2, 1.5, 0.7, None, None, None, *COLLISION_KIND_MASK_HSR, *HIT_STATUS_MASK_NORMAL, 0, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_BODY, false);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        AttackModule::clear(boma, 1, false);
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR, 0);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
    SET_SEARCH_SIZE_EXIST(agent, 2, 8);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        UNABLE_AREA(agent, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH_ADD);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        ArticleModule::change_status(boma, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, *WEAPON_JACK_WIREROPE_STATUS_KIND_BACK, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
        UNABLE_AREA(agent, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH);
    }
}

unsafe extern "C" fn game_specialairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, false, 0);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_HI_FLAG_REVERSE_LR);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_AIR_LASSO_FLAG_HANG_IMMIDIATE);
    }
    frame(lua_state, 13.0);
    if is_excute(agent) {
        GroundModule::select_cliff_hangdata(boma, *FIGHTER_JACK_CLIFF_HANG_DATA_AIR_LASSO as u32);
        WorkModule::off_flag(boma, *FIGHTER_STATUS_AIR_LASSO_FLAG_HANG_IMMIDIATE);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        ArticleModule::change_status(boma, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, *WEAPON_JACK_WIREROPE_STATUS_KIND_EXTEND, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        // AreaModule::reset_area(boma, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH);
        // ENABLE_AREA(fighter, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH);
        // AreaModule::reset_area(boma, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH_ADD);
        // ENABLE_AREA(fighter, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH_ADD);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("throw"), 10.0, 70, 60, 0, 60, 6.5, 1.5, 2.0, 2.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 70,60, 0, 60, 6.5, 0.0, 22.0, 3.2, Some(0.0), Some(38.0), Some(10.6), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);

    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        AttackModule::clear(boma, 1, false);
        ATTACK(agent, 0, 0, Hash40::new("throw"), 10.0, 255, 15, 0, 70, 6.5, 1.5, 2.0, 2.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR, 0);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        SET_SEARCH_SIZE_EXIST(agent, 2, 8);
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        // UNABLE_AREA(fighter, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH_ADD);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        ArticleModule::change_status(boma, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, *WEAPON_JACK_WIREROPE_STATUS_KIND_BACK, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
        // UNABLE_AREA(fighter, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH);
    }
}

unsafe extern "C" fn effect_specialairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 20.0);
    if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("jack_wire_line"), Hash40::new("throw"), 0, 0, 0, 115.5, 0, -1, 0.6, true);
        }
    }
    else {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("jack_wire_line"), Hash40::new("throw"), 0, 0, 0, 115.5, 0, 1, 0.6, true);
        }
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("sys_smash_flash"), -1);
        EFFECT_OFF_KIND(agent, Hash40::new("jack_wire_line"), false, true);
    }
}

unsafe extern "C" fn game_specialairhif(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        KineticModule::mul_speed(boma, &Vector3f::new(0.8, 1.0, 1.0), *FIGHTER_KINETIC_ENERGY_ID_STOP);
        ArticleModule::change_motion(boma, *FIGHTER_JACK_GENERATE_ARTICLE_WING, Hash40::new("special_hi2_f"), false, 0.0);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 34.0);
    frame(lua_state, 37.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_HI2_FLAG_APPEAR_DOYLE);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialairndown", game_specialairndown, Priority::Low);
    
    agent.acmd("game_specialhi", game_specialhi, Priority::Low);
    agent.acmd("game_specialairhi", game_specialairhi, Priority::Low);
    agent.acmd("effect_specialairhi", effect_specialairhi, Priority::Low);
    agent.acmd("game_specialairhif", game_specialairhif, Priority::Low);
}
