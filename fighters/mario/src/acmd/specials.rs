
use super::*;





#[acmd_script( agent = "mario", script = "game_specialn" , category = ACMD_GAME , low_priority)]
unsafe fn mario_specialn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.149);
        VarModule::off_flag(fighter.battle_object, mario::FIREBRAND_ACTIVATED);
        VarModule::off_flag(fighter.battle_object, mario::DOUBLE_FIREBALL);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        if !ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
            ArticleModule::generate_article(boma, *FIGHTER_MARIO_GENERATE_ARTICLE_FIREBALL, false, 0);
            special_projectile_spawned[hdr::get_player_number(boma)] = true;
         }
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
            VarModule::on_flag(fighter.battle_object, mario::FIREBRAND_ACTIVATED);
            ATTACK(fighter, 0, 0, Hash40::new("arml"), 12.0, 50, 115, 0, 50, 7.5, 6.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("arml"), 12.0, 50, 115, 0, 50, 5.5, 2.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("shoulderl"), 12.0, 50, 115, 0, 50, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
            HIT_NODE(fighter, Hash40::new("handl"), *HIT_STATUS_XLU);
            HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
            FT_MOTION_RATE(fighter, 1.250);
        }
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)) && !ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) 
           && !VarModule::is_flag(fighter.battle_object, mario::FIREBRAND_ACTIVATED) {
            VarModule::on_flag(fighter.battle_object, mario::DOUBLE_FIREBALL);
            ArticleModule::generate_article(boma, *FIGHTER_MARIO_GENERATE_ARTICLE_FIREBALL, false, 0);
            FT_MOTION_RATE(fighter, 2.000);
        }
    }
    
}

#[acmd_script( agent = "mario", script = "game_specialairn" , category = ACMD_GAME , low_priority)]
unsafe fn mario_specialairn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.149);
        VarModule::off_flag(fighter.battle_object, mario::FIREBRAND_ACTIVATED);
        VarModule::off_flag(fighter.battle_object, mario::DOUBLE_FIREBALL);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        if !ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
            ArticleModule::generate_article(boma, *FIGHTER_MARIO_GENERATE_ARTICLE_FIREBALL, false, 0);
            special_projectile_spawned[hdr::get_player_number(boma)] = true;
         }
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
            VarModule::on_flag(fighter.battle_object, mario::FIREBRAND_ACTIVATED);
            ATTACK(fighter, 0, 0, Hash40::new("arml"), 12.0, 50, 115, 0, 50, 7.5, 6.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("arml"), 12.0, 50, 115, 0, 50, 5.5, 2.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("shoulderl"), 12.0, 50, 115, 0, 50, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
            HIT_NODE(fighter, Hash40::new("handl"), *HIT_STATUS_XLU);
            HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
            FT_MOTION_RATE(fighter, 1.250);
        }
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)) && !ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) 
           && !VarModule::is_flag(fighter.battle_object, mario::FIREBRAND_ACTIVATED) {
            VarModule::on_flag(fighter.battle_object, mario::DOUBLE_FIREBALL);
            ArticleModule::generate_article(boma, *FIGHTER_MARIO_GENERATE_ARTICLE_FIREBALL, false, 0);
            FT_MOTION_RATE(fighter, 2.000);
        }
    }
    
}

#[acmd_script( agent = "mario", script = "game_specials" , category = ACMD_GAME , low_priority)]
unsafe fn mario_specials(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_MARIO_GENERATE_ARTICLE_MANTLE, false, 0);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        SEARCH(fighter, 0, 0, Hash40::new("top"), 8.0, 0.0, 6.5, 2.5, Some(0.0), Some(6.5), Some(8.0), *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_NORMAL, 60, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false);
        WorkModule::set_float(boma, 9.0, *FIGHTER_MARIO_STATUS_SPECIAL_S_WORK_ID_FLOAT_REFLECT_MOTION_FRAME);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIO_REFLECTOR_KIND_MANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 110, 100, 80, 0, 7.5, 0.0, 6.7, 9.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_turn"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_MANT, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 110, 100, 80, 0, 5.0, 0.0, 6.7, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_turn"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_MANT, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIO_REFLECTOR_KIND_MANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        WorkModule::off_flag(boma, *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
    }
}

#[acmd_script( agent = "mario", script = "game_specialairs" , category = ACMD_GAME , low_priority)]
unsafe fn mario_specialairs(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        ArticleModule::generate_article(boma, *FIGHTER_MARIO_GENERATE_ARTICLE_MANTLE, false, 0);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        SEARCH(fighter, 0, 0, Hash40::new("top"), 8.0, 0.0, 6.5, 2.5, Some(0.0), Some(6.5), Some(8.0), *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_NORMAL, 60, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false);
        WorkModule::set_float(boma, 9.0, *FIGHTER_MARIO_STATUS_SPECIAL_S_WORK_ID_FLOAT_REFLECT_MOTION_FRAME);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIOD_REFLECTOR_KIND_DRMANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 110, 100, 80, 0, 7.5, 0.0, 6.7, 9.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_turn"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_MANT, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 110, 100, 80, 0, 5.0, 0.0, 6.7, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_turn"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_MANT, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIOD_REFLECTOR_KIND_DRMANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        WorkModule::off_flag(boma, *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
    }
}

#[acmd_script( agent = "mario", script = "game_specialhi" , category = ACMD_GAME , low_priority)]
unsafe fn mario_specialhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    if (WorkModule::is_flag(boma, *FIGHTER_MARIO_STATUS_SPECIAL_HI_FLAG_CAPPY)) {
        frame(lua_state, 3.0);
        if is_excute(fighter) {
            SA_SET(fighter, *SITUATION_KIND_AIR);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 60, 100, 160, 0, 2.5, 0.0, 6.5, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 86, 100, 150, 0, 4.0, 0.0, 6.5, 5.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 100, 100, 150, 0, 4.0, 0.0, 6.3, 9.2, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, *ATTACK_REGION_PUNCH);
            AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
        }
        wait(lua_state, 3.0);
        if is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 0.6, 60, 100, 180, 0, 3.0, 0.0, 6.5, 2.5, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 0.6, 92, 100, 170, 0, 3.8, 0.0, 6.5, 7.5, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 0.6, 60, 100, 110, 0, 3.0, 0.0, 10.0, 2.5, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 3, 0, Hash40::new("top"), 0.6, 92, 100, 110, 0, 3.8, 0.0, 10.0, 8.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, *ATTACK_REGION_PUNCH);
            AttackModule::set_no_finish_camera(boma, 0, true, false);
            AttackModule::set_no_finish_camera(boma, 1, true, false);
            AttackModule::set_no_finish_camera(boma, 2, true, false);
            AttackModule::set_no_finish_camera(boma, 3, true, false);
            AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
        }
        frame(lua_state, 13.0);
        if is_excute(fighter) {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
        frame(lua_state, 17.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 60, 155, 0, 50, 7.5, 0.0, 11.5, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN_LAST, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 60, 155, 0, 50, 7.5, 0.0, 11.5, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN_LAST, *ATTACK_REGION_PUNCH);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
    }
    else{
        frame(lua_state, 3.0);
        if is_excute(fighter) {
            SA_SET(fighter, *SITUATION_KIND_AIR);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 60, 100, 160, 0, 2.5, 0.0, 6.5, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 86, 100, 150, 0, 4.0, 0.0, 6.5, 5.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 100, 100, 150, 0, 4.0, 0.0, 6.3, 9.2, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
            AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
        }
        wait(lua_state, 3.0);
        if is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 0.6, 60, 100, 180, 0, 3.0, 0.0, 6.5, 2.5, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 0.6, 92, 100, 170, 0, 3.8, 0.0, 6.5, 7.5, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 0.6, 60, 100, 110, 0, 3.0, 0.0, 10.0, 2.5, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 3, 0, Hash40::new("top"), 0.6, 92, 100, 110, 0, 3.8, 0.0, 10.0, 8.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
            AttackModule::set_no_finish_camera(boma, 0, true, false);
            AttackModule::set_no_finish_camera(boma, 1, true, false);
            AttackModule::set_no_finish_camera(boma, 2, true, false);
            AttackModule::set_no_finish_camera(boma, 3, true, false);
            AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
        }
        frame(lua_state, 13.0);
        if is_excute(fighter) {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
        frame(lua_state, 17.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 60, 155, 0, 50, 9.0, 0.0, 11.5, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIO_COIN_LAST, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 60, 155, 0, 50, 9.0, 0.0, 11.5, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIO_COIN_LAST, *ATTACK_REGION_PUNCH);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
    }
    
}

#[acmd_script( agent = "mario", script = "game_specialairhi" , category = ACMD_GAME , low_priority)]
unsafe fn mario_specialairhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    if (WorkModule::is_flag(boma, *FIGHTER_MARIO_STATUS_SPECIAL_HI_FLAG_CAPPY)) {
        frame(lua_state, 3.0);
        if is_excute(fighter) {
            SA_SET(fighter, *SITUATION_KIND_AIR);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 60, 100, 160, 0, 2.5, 0.0, 6.5, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 86, 100, 150, 0, 4.0, 0.0, 6.5, 5.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 100, 100, 150, 0, 4.0, 0.0, 6.3, 9.2, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, *ATTACK_REGION_PUNCH);
            AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
        }
        wait(lua_state, 3.0);
        if is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 0.6, 60, 100, 180, 0, 3.0, 0.0, 6.5, 2.5, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 0.6, 92, 100, 170, 0, 3.8, 0.0, 6.5, 7.5, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 0.6, 60, 100, 110, 0, 3.0, 0.0, 10.0, 2.5, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 3, 0, Hash40::new("top"), 0.6, 92, 100, 110, 0, 3.8, 0.0, 10.0, 8.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN, *ATTACK_REGION_PUNCH);
            AttackModule::set_no_finish_camera(boma, 0, true, false);
            AttackModule::set_no_finish_camera(boma, 1, true, false);
            AttackModule::set_no_finish_camera(boma, 2, true, false);
            AttackModule::set_no_finish_camera(boma, 3, true, false);
            AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
        }
        frame(lua_state, 13.0);
        if is_excute(fighter) {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
        frame(lua_state, 17.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 60, 152, 0, 50, 7.5, 0.0, 11.5, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN_LAST, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 60, 152, 0, 50, 7.5, 0.0, 11.5, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_mario_local_coin"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIO_LOCAL_COIN_LAST, *ATTACK_REGION_PUNCH);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
    }
    else {
        frame(lua_state, 3.0);
        if is_excute(fighter) {
            SA_SET(fighter, *SITUATION_KIND_AIR);
            WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 60, 100, 160, 0, 2.5, 0.0, 6.5, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 86, 100, 150, 0, 4.0, 0.0, 6.5, 5.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 100, 100, 150, 0, 4.0, 0.0, 6.3, 9.2, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
            AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
        }
        wait(lua_state, 3.0);
        if is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 0.6, 60, 100, 180, 0, 3.0, 0.0, 6.5, 2.5, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 0.6, 92, 100, 170, 0, 3.8, 0.0, 6.5, 7.5, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 0.6, 60, 100, 110, 0, 3.0, 0.0, 10.0, 2.5, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 3, 0, Hash40::new("top"), 0.6, 92, 100, 110, 0, 3.8, 0.0, 10.0, 8.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
            AttackModule::set_no_finish_camera(boma, 0, true, false);
            AttackModule::set_no_finish_camera(boma, 1, true, false);
            AttackModule::set_no_finish_camera(boma, 2, true, false);
            AttackModule::set_no_finish_camera(boma, 3, true, false);
            AttackModule::set_no_damage_fly_smoke_all(boma, true, false);
        }
        frame(lua_state, 13.0);
        if is_excute(fighter) {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
        frame(lua_state, 17.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 60, 155, 0, 50, 9.0, 0.0, 11.5, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIO_COIN_LAST, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 60, 155, 0, 50, 9.0, 0.0, 11.5, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIO_COIN_LAST, *ATTACK_REGION_PUNCH);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        mario_specialn,
        mario_specialairn,
        mario_specials,
        mario_specialairs,
        mario_specialhi,
        mario_specialairhi,
    );
}

