use super::*;

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, false, 0);
        ArticleModule::change_motion(boma, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, Hash40::new("special_hi"), false, 0.0);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_HI_FLAG_REVERSE_LR);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        ArticleModule::change_status(boma, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, *WEAPON_JACK_WIREROPE_STATUS_KIND_EXTEND, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        // AreaModule::reset_area(boma, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH);
        // ENABLE_AREA(agent, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH);
        // AreaModule::reset_area(boma, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH_ADD);
        // ENABLE_AREA(agent, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH_ADD);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("throw"), 10.0, 70, 60, 0, 60, 4.5, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 70, 60, 0, 60, 4.5, 0.0, 22.0, 1.7, Some(0.0), Some(38.0), Some(9.1), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        AttackModule::clear(boma, 1, false);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            SEARCH(agent, 0, 0, Hash40::new("throw"), 5.5, 1.5, 1.5, 0.7, None, None, None, *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_NORMAL, 0, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_BODY, false);
            SEARCH(agent, 1, 0, Hash40::new("top"), 5.0, 0.0, 60.0, 19.5, Some(0.0), Some(77.0), Some(27.25), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_NORMAL, 0, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_BODY, false);
        }
    }
    frame(lua_state, 23.0);
    if is_excute(agent) {
        //UNABLE_AREA(agent, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH_ADD);
        SearchModule::clear(boma, 1);
    }
    frame(lua_state, 27.0);
    if is_excute(agent) {
        ArticleModule::change_status(boma, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, *WEAPON_JACK_WIREROPE_STATUS_KIND_BACK, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
        //UNABLE_AREA(agent, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH);
    }
}

unsafe extern "C" fn game_specialairhi(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        boma.off_flag(*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_SPECIAL_AIR_HI_HOP);
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
        ATTACK(agent, 0, 0, Hash40::new("throw"), 10.0, 70, 60, 0, 60, 4.5, 1.5, 2.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 70, 60, 0, 60, 4.5, 0.0, 22.0, 3.2, Some(0.0), Some(38.0), Some(10.6), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 21.0);
    if is_excute(agent) {
        AttackModule::clear(boma, 1, false);
        ATTACK(agent, 0, 0, Hash40::new("throw"), 10.0, 262, 28, 0, 45, 5.5, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR, 0);
    }
    frame(lua_state, 22.0);
    if is_excute(agent) {
        //SET_SEARCH_SIZE_EXIST(agent, 2, 8);
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
    frame(lua_state, 45.0);
    if is_excute(agent) {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            CancelModule::enable_cancel(boma);
        }
        else {
            VarModule::on_flag(agent.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
        }
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
        EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.7, true);
    }
    frame(lua_state, 26.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("sys_smash_flash"), -1);
        EFFECT_OFF_KIND(agent, Hash40::new("jack_wire_line"), false, true);
    }
}

unsafe extern "C" fn game_specialhithrow(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, false, -1);
        ArticleModule::change_motion(boma, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, Hash40::new("special_hi_throw"), false, -1.0);
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.0, 80, 40, 0, 75, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        CHECK_FINISH_CAMERA(agent, -20, 10);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        let target = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(lua_state, 20.0);
    FT_MOTION_RATE_RANGE(agent, 20.0, 25.0, 7.0);
    frame(lua_state, 25.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specialairhif(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 37.0, 16.0);
    if is_excute(agent) {
        ArticleModule::change_motion(boma, *FIGHTER_JACK_GENERATE_ARTICLE_WING, Hash40::new("special_hi2_f"), false, 0.0);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
        ATTACK(agent, 0, 0, Hash40::new("hip"), 6.0, 75, 10, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 37.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_JACK_STATUS_SPECIAL_HI2_FLAG_APPEAR_DOYLE);
    }
}

unsafe extern "C" fn game_specialairhiendf(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 1.0);
    if is_excute(agent) {
        KineticModule::suspend_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        KineticModule::mul_speed(boma, &Vector3f::new(0.75, 0.5, 1.0), *FIGHTER_KINETIC_ENERGY_ID_STOP);
    }
    wait(lua_state, 6.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        KineticModule::resume_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialhi", game_specialhi, Priority::Low);
    agent.acmd("game_specialairhi", game_specialairhi, Priority::Low);
    agent.acmd("effect_specialairhi", effect_specialairhi, Priority::Low);

    agent.acmd("game_specialhithrow", game_specialhithrow, Priority::Low);

    agent.acmd("game_specialairhif", game_specialairhif, Priority::Low);
    agent.acmd("game_specialairhib", game_specialairhif, Priority::Low);
    agent.acmd("game_specialairhiendf", game_specialairhiendf, Priority::Low);
    agent.acmd("game_specialairhiendb", game_specialairhiendf, Priority::Low);
}