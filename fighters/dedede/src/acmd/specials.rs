
use super::*;

#[acmd_script( agent = "dedede", script = "game_specialnstart" , category = ACMD_GAME , low_priority)]
unsafe fn dedede_special_n_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
        FT_MOTION_RATE(fighter, 0.6);
    frame(lua_state, 17.0);
        FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script( agent = "dedede", script = "game_specialnloop", category = ACMD_GAME, low_priority )]
unsafe fn dedede_special_n_loop_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_DEDEDE_STATUS_SPECIAL_N_FLAG_INHALE);
        CATCH(fighter, 0, Hash40::new("top"), 5.0, 0.0, 9.0, 5.0, Some(0.0), Some(9.0), Some(12.0), *FIGHTER_STATUS_KIND_SWALLOWED, *COLLISION_SITUATION_MASK_GA);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 0.0, 170, 40, 30, 0, 6.0, 0.0, 9.0, 16.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 0.0, 170, 40, 10, 0, 7.0, 0.0, 6.0, 23.0, Some(0.0), Some(12.0), Some(23.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 9.0, 0.0, 8.5, 10.0, Some(0.0), Some(8.5), Some(21.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        SEARCH(fighter, 0, 0, Hash40::new("top"), 4.0, 0.0, 13.0, 5.5, Some(0.0), Some(3.5), Some(5.5), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, true);
        search!(fighter, *MA_MSC_CMD_SEARCH_SET_OPPONENT, 0, 0, *COLLISION_TARGET_PROPERTY, *COLLISION_PROPERTY_MASK_REFLECT);
        SEARCH(fighter, 1, 0, Hash40::new("top"), 9.5, 0.0, 8.5, 11.0, Some(0.0), Some(8.5), Some(20.5), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, true);
        search!(fighter, *MA_MSC_CMD_SEARCH_SET_OPPONENT, 1, 0, *COLLISION_TARGET_PROPERTY, *COLLISION_PROPERTY_MASK_REFLECT);
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_DEDEDE_STATUS_SPECIAL_N_FLAG_INHALE);
            CATCH(fighter, 0, Hash40::new("top"), 5.0, 0.0, 5.5, 5.0, Some(0.0), Some(5.5), Some(12.0), *FIGHTER_STATUS_KIND_SWALLOWED, *COLLISION_SITUATION_MASK_GA);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 0.0, 170, 40, 30, 0, 6.0, 0.0, 6.5, 16.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 0.0, 170, 40, 10, 0, 7.0, 0.0, 3.5, 23.0, Some(0.0), Some(9.5), Some(23.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            ATTACK(fighter, 3, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 9.0, 0.0, 6.0, 10.0, Some(0.0), Some(6.0), Some(21.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            SEARCH(fighter, 0, 0, Hash40::new("top"), 4.0, 0.0, 9.5, 5.5, Some(0.0), Some(2.5), Some(5.5), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, true);
            search!(fighter, *MA_MSC_CMD_SEARCH_SET_OPPONENT, 0, 0, *COLLISION_TARGET_PROPERTY, *COLLISION_PROPERTY_MASK_REFLECT);
            SEARCH(fighter, 1, 0, Hash40::new("top"), 9.5, 0.0, 6.0, 11.0, Some(0.0),Some(6.0), Some(20.5), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, true);
            search!(fighter, *MA_MSC_CMD_SEARCH_SET_OPPONENT, 1, 0, *COLLISION_TARGET_PROPERTY, *COLLISION_PROPERTY_MASK_REFLECT);
        }
    }
}

#[acmd_script( agent = "dedede", script = "game_specialairnstart" , category = ACMD_GAME , low_priority)]
unsafe fn dedede_special_air_n_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
        FT_MOTION_RATE(fighter, 0.6);
    frame(lua_state, 17.0);
        FT_MOTION_RATE(fighter, 1.0);

}

#[acmd_script( agent = "dedede", script = "game_specialairnloop", category = ACMD_GAME, low_priority )]
unsafe fn dedede_special_air_n_loop_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_DEDEDE_STATUS_SPECIAL_N_FLAG_INHALE);
        CATCH(fighter, 0, Hash40::new("top"), 5.0, 0.0, 12.0, 5.0, Some(0.0), Some(12.0), Some(9.0), *FIGHTER_STATUS_KIND_SWALLOWED, *COLLISION_SITUATION_MASK_GA);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 0.0, 170, 40, 30, 0, 6.0, 0.0, 12.0, 16.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 0.0, 170, 40, 10, 0, 7.0, 0.0, 9.0, 23.0, Some(0.0), Some(15.0), Some(23.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 9.0, 0.0, 11.5, 10.0, Some(0.0), Some(11.5), Some(21.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        SEARCH(fighter, 0, 0, Hash40::new("top"), 4.0, 0.0, 16.0, 5.5, Some(0.0), Some(6.5), Some(5.5), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, true);
        search!(fighter, *MA_MSC_CMD_SEARCH_SET_OPPONENT, 0, 0, *COLLISION_TARGET_PROPERTY, *COLLISION_PROPERTY_MASK_REFLECT);
        SEARCH(fighter, 1, 0, Hash40::new("top"), 9.5, 0.0, 11.5, 11.0, Some(0.0), Some(11.5), Some(20.5), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, true);
        search!(fighter, *MA_MSC_CMD_SEARCH_SET_OPPONENT, 1, 0, *COLLISION_TARGET_PROPERTY, *COLLISION_PROPERTY_MASK_REFLECT);
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_DEDEDE_STATUS_SPECIAL_N_FLAG_INHALE);
            CATCH(fighter, 0, Hash40::new("top"), 5.0, 0.0, 5.5, 5.0, Some(0.0), Some(5.5), Some(9.0), *FIGHTER_STATUS_KIND_SWALLOWED, *COLLISION_SITUATION_MASK_GA);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 0.0, 170, 40, 30, 0, 6.0, 0.0, 6.5, 16.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 0.0, 170, 40, 10, 0, 7.0, 0.0, 3.5, 23.0, Some(0.0), Some(9.5), Some(23.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            ATTACK(fighter, 3, 0, Hash40::new("top"), 0.0, 0, 0, 0, 0, 9.0, 0.0, 6.0, 10.0, Some(0.0), Some(6.0), Some(21.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            SEARCH(fighter, 0, 0, Hash40::new("top"), 4.0, 0.0, 9.5, 5.5, Some(0.0), Some(2.5), Some(5.5), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, true);
            search!(fighter, *MA_MSC_CMD_SEARCH_SET_OPPONENT, 0, 0, *COLLISION_TARGET_PROPERTY, *COLLISION_PROPERTY_MASK_REFLECT);
            SEARCH(fighter, 1, 0, Hash40::new("top"), 9.5, 0.0, 6.0, 11.0, Some(0.0), Some(6.0), Some(20.5), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, true);
            search!(fighter, *MA_MSC_CMD_SEARCH_SET_OPPONENT, 1, 0, *COLLISION_TARGET_PROPERTY, *COLLISION_PROPERTY_MASK_REFLECT);
        }
    }
}

#[acmd_script( agent = "dedede", script = "game_specialnend" , category = ACMD_GAME , low_priority)]
unsafe fn dedede_special_n_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 25.0 / (19.0-1.0));
}

#[acmd_script( agent = "dedede", script = "game_specialairnend" , category = ACMD_GAME , low_priority)]
unsafe fn dedede_special_air_n_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 25.0 / (19.0-1.0));
}

#[acmd_script( agent = "dedede", script = "game_specialsstart" , category = ACMD_GAME , low_priority)]
unsafe fn dedede_special_s_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_GENERATE);
        VarModule::set_int(fighter.battle_object, vars::dedede::instance::INHALE_COUNTER, 0);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter){
        WorkModule::on_flag(boma, *FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_PUTOUT);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_THROW);
    }
    frame(lua_state, 25.0);
    FT_MOTION_RATE(fighter, 35.0 / (65.0-26.0));    
}

#[acmd_script( agent = "dedede", script = "effect_specialsstart" , category = ACMD_EFFECT , low_priority)]
unsafe fn dedede_special_s_start_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        if (PostureModule::lr(boma) == -1.0){
            EFFECT_FOLLOW(fighter, Hash40::new("dedede_hammer_arc_wind"), Hash40::new("throw"), 2.0, -2.0, -2.0, 90.0, 0.0, 0.0 , 0.6, true);
        }
        else{
            EFFECT_FOLLOW(fighter, Hash40::new("dedede_hammer_arc_wind"), Hash40::new("throw"), -2.0, -2.0, -2.0, 90.0, 180.0, 0.0 , 0.6, true);
        }
        LAST_EFFECT_SET_COLOR(fighter, 0.96, 0.6, 0.26); 
        
    }
    frame(lua_state, 18.0);
    if is_excute(fighter){     
        FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state,21.0);
    if is_excute(fighter){
        EFFECT_OFF_KIND(fighter, Hash40::new("dedede_hammer_arc_wind"), true, true);
    }
}

#[acmd_script( agent = "dedede", script = "sound_specialsstart", category = ACMD_SOUND, low_priority )]
unsafe fn dedede_special_s_start_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dedede_wad_throw"));
    }
}

#[acmd_script( agent = "dedede", script = "expression_specialsstart", category = ACMD_EXPRESSION, low_priority )]
unsafe fn dedede_special_s_start_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        //RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script( agent = "dedede", script = "effect_specialairsstart" , category = ACMD_EFFECT , low_priority)]
unsafe fn dedede_special_air_s_start_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        if (PostureModule::lr(boma) == -1.0){
            EFFECT_FOLLOW(fighter, Hash40::new("dedede_hammer_arc_wind"), Hash40::new("throw"), 2.0, -2.0, -2.0, 90.0, 0.0, 0.0 , 0.6, true);
        }
        else{
            EFFECT_FOLLOW(fighter, Hash40::new("dedede_hammer_arc_wind"), Hash40::new("throw"), -2.0, -2.0, -2.0, 90.0, 180.0, 0.0 , 0.6, true);
        }
        LAST_EFFECT_SET_COLOR(fighter, 0.96, 0.6, 0.26);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter){     
        FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state,21.0);
    if is_excute(fighter){
        EFFECT_OFF_KIND(fighter, Hash40::new("dedede_hammer_arc_wind"), true, true);
    }

}

#[acmd_script( agent = "dedede", script = "game_specialairsstart" , category = ACMD_GAME , low_priority)]
unsafe fn dedede_special_air_s_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_GENERATE);
        VarModule::set_int(fighter.battle_object, vars::dedede::instance::INHALE_COUNTER, 0);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter){
        WorkModule::on_flag(boma, *FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_PUTOUT);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_THROW);
    }
    frame(lua_state, 25.0);
    FT_MOTION_RATE(fighter, 35.0 / (65.0-26.0));    

}

#[acmd_script( agent = "dedede", script = "sound_specialairsstart", category = ACMD_SOUND, low_priority )]
unsafe fn dedede_special_air_s_start_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dedede_wad_throw"));
    }
}

#[acmd_script( agent = "dedede", script = "expression_specialairsstart", category = ACMD_EXPRESSION, low_priority )]
unsafe fn dedede_special_air_s_start_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        //RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_attackl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script( agent = "dedede", script = "game_specialsmiss", category = ACMD_GAME, low_priority )]
unsafe fn dedede_special_s_miss_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_MISS_SEARCH);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_MISS_SEARCH);
    }
}

#[acmd_script( agent = "dedede", script = "game_specialairsmiss", category = ACMD_GAME, low_priority )]
unsafe fn dedede_special_air_s_miss_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_MISS_SEARCH);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_MISS_SEARCH);
    }
}

#[acmd_script( agent = "dedede", script = "game_specialsget", category = ACMD_GAME, low_priority )]
unsafe fn dedede_special_s_get_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_THROW);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 32.0 / (65.0 - 26.0));
    }
}

#[acmd_script( agent = "dedede", script = "game_specialairsget", category = ACMD_GAME, low_priority )]
unsafe fn dedede_special_air_s_get_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_THROW);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 32.0 / (65.0 - 26.0));
    }
}

#[acmd_script( agent = "dedede", script = "effect_specialsget", category = ACMD_EFFECT, low_priority )]
unsafe fn dedede_special_s_get_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter){
        EFFECT(fighter, Hash40::new("sys_item_get"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter){
        if VarModule::is_flag(fighter.battle_object, vars::dedede::instance::IS_DASH_GORDO){
           FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -4.0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        }   
    }
}

#[acmd_script( agent = "dedede", script = "effect_specialairsget", category = ACMD_EFFECT, low_priority )]
unsafe fn dedede_special_air_s_get_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter){
        EFFECT(fighter, Hash40::new("sys_item_get"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter){
        if VarModule::is_flag(fighter.battle_object, vars::dedede::instance::IS_DASH_GORDO){
            FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -4.0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);

        }
    }
}

#[acmd_script( agent = "dedede", scripts = ["game_speciallwstart", "game_specialairlwstart"] , category = ACMD_GAME , low_priority)]
unsafe fn dedede_special_lw_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    if is_excute(fighter){
        FT_MOTION_RATE(fighter, 13.0/(18.0-1.0));
        ArticleModule::generate_article(boma, *FIGHTER_DEDEDE_GENERATE_ARTICLE_JETHAMMER, false, -1);
    }
}

#[acmd_script( agent = "dedede", script = "game_specialairlwstart" , category = ACMD_GAME , low_priority)]
unsafe fn dedede_special_air_lw_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    if is_excute(fighter){
        FT_MOTION_RATE(fighter, 13.0/(18.0-1.0));
        ArticleModule::generate_article(boma, *FIGHTER_DEDEDE_GENERATE_ARTICLE_JETHAMMER, false, -1);
    }
}

#[acmd_script( agent = "dedede", script = "game_speciallw" , category = ACMD_GAME , low_priority)]
unsafe fn dedede_special_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let rush_speed = 0.6 + 0.01*WorkModule::get_float(boma, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_WORK_FLOAT_HOLD_COUNT);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 7.0/(10.0-1.0));
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        KineticModule::add_speed(boma, &Vector3f::new(rush_speed, 0.0, 0.0));
        FT_MOTION_RATE(fighter, 1.0);
        ATTACK(fighter, 0, 0, Hash40::new("hammer1"), 11.0, 40, 80, 0, 40, 9.0, 16.0, 0.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
        ATTACK(fighter, 1, 0, Hash40::new("hammer1"), 11.0, 40, 80, 0, 40, 6.0, 10.0, 0.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
        ATTACK(fighter, 2, 0, Hash40::new("hammer1"), 11.0, 40, 80, 0, 40, 6.0, 5.0, 0.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
        ATTACK(fighter, 3, 0, Hash40::new("hammer1"), 11.0, 40, 80, 0, 40, 6.0, 0.0, 0.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 43.0/(60.0-15.0));
    }
    frame(lua_state, 60.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
}

#[acmd_script( agent = "dedede", script = "game_speciallwmax" , category = ACMD_GAME , low_priority)]
unsafe fn dedede_special_lw_max_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let rush_speed = 0.9 + 0.01*WorkModule::get_float(boma, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_WORK_FLOAT_HOLD_COUNT);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 14);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        KineticModule::add_speed(boma, &Vector3f::new(rush_speed, 0.0, 0.0));
        ATTACK(fighter, 0, 0, Hash40::new("hammer1"), 40.0, 361, 46, 0, 60, 9.0, 16.0, 0.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 30.0, 361, 46, 0, 60, 5.0, 0.0, 7.0, 2.0, Some(0.0), Some(7.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(lua_state, 16.0);
    for _ in 0.. 20 {
        if is_excute(fighter) {
            KineticModule::add_speed(boma, &Vector3f::new(-1.0 * (rush_speed/30.0), 0.0, 0.0));
        }
        wait(lua_state, 1.0);
    }
}

#[acmd_script( agent = "dedede", script = "game_specialairlw" , category = ACMD_GAME , low_priority)]
unsafe fn dedede_special_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let rush_speed = 0.6 + 0.01*WorkModule::get_float(boma, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_WORK_FLOAT_HOLD_COUNT);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 7.0/(10.0-1.0));
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        KineticModule::add_speed(boma, &Vector3f::new(rush_speed, 0.5, 0.0));
        FT_MOTION_RATE(fighter, 1.0);
        ATTACK(fighter, 0, 0, Hash40::new("hammer1"), 11.0, 40, 80, 0, 40, 9.0, 16.0, 0.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
        ATTACK(fighter, 1, 0, Hash40::new("hammer1"), 11.0, 40, 80, 0, 40, 6.0, 10.0, 0.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
        ATTACK(fighter, 2, 0, Hash40::new("hammer1"), 11.0, 40, 80, 0, 40, 6.0, 5.0, 0.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
        ATTACK(fighter, 3, 0, Hash40::new("hammer1"), 11.0, 40, 80, 0, 40, 6.0, 0.0, 0.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 43.0/(60.0-15.0));
    }
    frame(lua_state, 60.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
}

#[acmd_script( agent = "dedede", script = "game_specialairlwmax" , category = ACMD_GAME , low_priority)]
unsafe fn dedede_special_air_lw_max_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let rush_speed = 0.9 + 0.01*WorkModule::get_float(boma, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_WORK_FLOAT_HOLD_COUNT);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 14);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        KineticModule::add_speed(boma, &Vector3f::new(rush_speed, 0.0, 0.0));
        ATTACK(fighter, 0, 0, Hash40::new("hammer1"), 40.0, 361, 46, 0, 60, 9.0, 16.0, 0.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 30.0, 361, 46, 0, 60, 5.0, 0.0, 7.0, 2.0, Some(0.0), Some(7.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(lua_state, 16.0);
    for _ in 0.. 20 {
        if is_excute(fighter) {
            KineticModule::add_speed(boma, &Vector3f::new(-1.0 * (rush_speed/30.0), 0.0, 0.0));
        }
        wait(lua_state, 1.0);
    }
}

#[acmd_script( agent = "dedede", script = "game_speciallwjumpsquat" , category = ACMD_GAME , low_priority)]
unsafe fn dedede_special_lw_jump_squat_game(fighter: &mut L2CAgentBase){
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    FT_MOTION_RATE(fighter, 5.0 / (8.0-1.0));
}

#[acmd_script( agent = "dedede", script = "game_specialhijump" , category = ACMD_GAME , low_priority)]
unsafe fn dedede_special_hi_jump_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        ATTACK(fighter, 0, 0, Hash40::new("head"), 4.0, 65, 100, 0, 50, 5.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 65, 100, 0, 50, 6.0, 0.0, 7.0, -5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 4.0, 65, 100, 0, 50, 6.0, 0.0, 7.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_REACTION_VALUE, 140);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_DEDEDE_STATUS_SUPER_JUMP_WORK_FLAG_TURN);
    }
    frame(lua_state, 39.0);
    if is_excute(fighter){
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
    }
    frame(lua_state, 52.0);
    if is_excute(fighter) {
        KineticModule::add_speed(boma, &Vector3f::new(0.0, -4.0, 0.0));
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_REACTION_VALUE, 140);
        HIT_NODE(fighter, Hash40::new("footr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("footl"), *HIT_STATUS_XLU);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 270, 90, 0, 60, 9.8, 0.0, 7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0, 270, 56, 0, 60, 8.5, 0.0, 7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
}


#[acmd_script( agent = "dedede", scripts = ["game_specialhilandingr", "game_specialhilandingl"] , category = ACMD_GAME , low_priority)]
unsafe fn dedede_special_hi_landing_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    if is_excute(fighter){
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    if is_excute(fighter){
        HIT_NODE(fighter, Hash40::new("footr"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("footl"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("virtualwaist"), *HIT_STATUS_NORMAL); 
    }
    frame(lua_state, 3.0);
    if is_excute(fighter){
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 90, 82, 0, 95, 6.0, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 70, 82, 0, 95, 6.0, 0.0, 6.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 70, 82, 0, 95, 6.0, 0.0, 6.0, -10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter){
        WorkModule::on_flag(boma, *FIGHTER_DEDEDE_STATUS_SUPER_JUMP_WORK_FLAG_STAR_RIGHT);

        ArticleModule::generate_article(boma, *FIGHTER_DEDEDE_GENERATE_ARTICLE_STAR, false, 0);
        WorkModule::on_flag(boma, *FIGHTER_DEDEDE_STATUS_SUPER_JUMP_WORK_FLAG_STAR_LEFT);

        ArticleModule::generate_article(boma, *FIGHTER_DEDEDE_GENERATE_ARTICLE_STAR, false, 0);
        AttackModule::clear_all(boma);
    }

}


pub fn install() {
    install_acmd_scripts!(
        dedede_special_n_start_game,
        dedede_special_n_loop_game,
        dedede_special_air_n_start_game,
        dedede_special_air_n_loop_game,
        dedede_special_n_end_game,
        dedede_special_air_n_end_game,
        dedede_special_s_start_game,
        dedede_special_s_start_effect,
        dedede_special_s_start_sound,
        dedede_special_s_start_expression,
        dedede_special_air_s_start_effect,
        dedede_special_air_s_start_game,
        dedede_special_air_s_start_sound,
        dedede_special_air_s_start_expression,
        dedede_special_s_miss_game,
        dedede_special_air_s_miss_game,
        dedede_special_s_get_game,
        dedede_special_air_s_get_game,
        dedede_special_s_get_effect,
        dedede_special_air_s_get_effect,
        dedede_special_lw_start_game,
        dedede_special_air_lw_start_game,
        dedede_special_lw_game,
        dedede_special_lw_max_game,
        dedede_special_air_lw_game,
        dedede_special_air_lw_max_game,
        dedede_special_lw_jump_squat_game,
        dedede_special_hi_jump_game,
        dedede_special_hi_landing_game,
    );
}

