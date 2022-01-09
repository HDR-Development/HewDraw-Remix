
use super::*;





#[acmd_script( agent = "mariod", script = "game_specialn" , category = ACMD_GAME , low_priority)]
unsafe fn mariod_special_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        VarModule::off_flag(fighter.battle_object, mario::FIREBRAND_ACTIVATED);
        FT_MOTION_RATE(fighter, 1.149);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        if !ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
            if (!special_projectile_spawned[hdr::get_player_number(boma)]) {
                ArticleModule::generate_article(boma, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRCAPSULE, false, 0);
            }
        }
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK)  {
            VarModule::on_flag(fighter.battle_object, mario::FIREBRAND_ACTIVATED);
            ATTACK(fighter, 0, 0, Hash40::new("arml"), 10.0, 65, 100, 0, 30, 7.5, 6.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("arml"), 10.0, 65, 100, 0, 30, 5.5, 2.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("shoulderl"), 10.0, 65, 100, 0, 30, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_PUNCH);
            HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
            HIT_NODE(fighter, Hash40::new("handl"), *HIT_STATUS_XLU);
            HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "mariod", script = "game_specialairn" , category = ACMD_GAME , low_priority)]
unsafe fn mariod_special_air_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        VarModule::off_flag(fighter.battle_object, mario::FIREBRAND_ACTIVATED);
        FT_MOTION_RATE(fighter, 1.149);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        if !ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
            if (!special_projectile_spawned[hdr::get_player_number(boma)]) {
                ArticleModule::generate_article(boma, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRCAPSULE, false, 0);
            }
        }
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK)  {
            VarModule::on_flag(fighter.battle_object, mario::FIREBRAND_ACTIVATED);
            ATTACK(fighter, 0, 0, Hash40::new("arml"), 10.0, 65, 100, 0, 30, 7.5, 6.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("arml"), 10.0, 65, 100, 0, 30, 5.5, 2.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("shoulderl"), 10.0, 65, 100, 0, 30, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_PUNCH);
            HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
            HIT_NODE(fighter, Hash40::new("handl"), *HIT_STATUS_XLU);
            HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "mariod", script = "game_specials" , category = ACMD_GAME , low_priority)]
unsafe fn mariod_special_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRMANTLE, false, 0);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        SEARCH(fighter, 0, 0, Hash40::new("top"), 8.0, 0.0, 6.5, 2.5, Some(0.0), Some(6.5), Some(8.0), *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_NORMAL, 60, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false);
        WorkModule::set_float(boma, 9.0, *FIGHTER_MARIOD_STATUS_SPECIAL_S_WORK_ID_FLOAT_REFLECT_MOTION_FRAME);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIOD_REFLECTOR_KIND_DRMANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
            FT_MOTION_RATE(fighter, 1.667);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 9.5, 361, 87, 0, 45, 4.5, 0.0, 13.0, 8.0, Some(0.0), Some(1.0), Some(8.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 9.5, 361, 87, 0, 45, 5.0, 0.0, 6.7, 5.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_OBJECT);
        }
        if !ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 110, 100, 80, 0, 4.5, 0.0, 13.0, 8.0, Some(0.0), Some(1.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13c64f9fca), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIOD_MANT, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0, 110, 100, 80, 0, 5.0, 0.0, 6.7, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13c64f9fca), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIOD_MANT, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 0.0, 35, 100, 45, 0, 10, 0.0, 8.0, 8.0, Some(0.0), Some(8.0), Some(13.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIOD_REFLECTOR_KIND_DRMANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRMANTLE, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    
}

#[acmd_script( agent = "mariod", script = "game_specialairs" , category = ACMD_GAME , low_priority)]
unsafe fn mariod_special_air_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRMANTLE, false, 0);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        SEARCH(fighter, 0, 0, Hash40::new("top"), 8.0, 0.0, 6.5, 2.5, Some(0.0), Some(6.5), Some(8.0), *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_NORMAL, 60, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false);
        WorkModule::set_float(boma, 9.0, *FIGHTER_MARIOD_STATUS_SPECIAL_S_WORK_ID_FLOAT_REFLECT_MOTION_FRAME);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIOD_REFLECTOR_KIND_DRMANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
            FT_MOTION_RATE(fighter, 1.667);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 9.5, 361, 87, 0, 45, 4.5, 0.0, 13.0, 8.0, Some(0.0), Some(1.0), Some(8.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 9.5, 361, 87, 0, 45, 5.0, 0.0, 6.7, 5.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_OBJECT);
        }
        if !ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 110, 100, 80, 0, 4.5, 0.0, 13.0, 8.0, Some(0.0), Some(1.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13c64f9fca), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIOD_MANT, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0, 110, 100, 80, 0, 5.0, 0.0, 6.7, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13c64f9fca), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIOD_MANT, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 0.0, 35, 100, 45, 0, 10, 0.0, 8.0, 8.0, Some(0.0), Some(8.0), Some(13.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIOD_REFLECTOR_KIND_DRMANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRMANTLE, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    
}

#[acmd_script( agent = "mariod", script = "game_specialhi" , category = ACMD_GAME , low_priority)]
unsafe fn mariod_special_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        SA_SET(fighter, *SITUATION_KIND_AIR);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 50, 102, 0, 30, 6.0, 0.0, 6.0, 9.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 74, 66, 0, 64, 6.0, 0.0, 9.5, 4.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    
}

#[acmd_script( agent = "mariod", script = "game_specialairhi" , category = ACMD_GAME , low_priority)]
unsafe fn mariod_special_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        SA_SET(fighter, *SITUATION_KIND_AIR);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 50, 102, 0, 30, 6.0, 0.0, 6.0, 9.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 74, 66, 0, 64, 6.0, 0.0, 9.5, 4.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        mariod_special_n_game,
        mariod_special_air_n_game,
        mariod_special_s_game,
        mariod_special_air_s_game,
        mariod_special_hi_game,
        mariod_special_air_hi_game,
    );
}

