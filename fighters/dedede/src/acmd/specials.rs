
use super::*;

#[acmd_script( agent = "dedede", script = "game_specialnstart" , category = ACMD_GAME , low_priority)]
unsafe fn dedede_special_n_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
        FT_MOTION_RATE(fighter, 0.6);
    frame(lua_state, 17.0);
        FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
            CATCH(fighter, 0, Hash40::new("top"), 7.5, 0.0, 7.0, 4.0, Some(0.0), Some(7.0), Some(7.0), *FIGHTER_STATUS_KIND_SWALLOWED, *COLLISION_SITUATION_MASK_GA);
            ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            SEARCH(fighter, 0, 0, Hash40::new("top"), 4.0, 0.0, 9.5, 5.5, Some(0.0), Some(2.5), Some(5.5), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, true);
            search!(fighter, *MA_MSC_CMD_SEARCH_SET_OPPONENT, 0, 0, *COLLISION_TARGET_PROPERTY, *COLLISION_PROPERTY_MASK_REFLECT);
        }
        else{
            CATCH(fighter, 0, Hash40::new("top"), 9.0, 0.0, 9.0, 4.0, Some(0.0), Some(9.0), Some(7.0), *FIGHTER_STATUS_KIND_SWALLOWED, *COLLISION_SITUATION_MASK_GA);
            ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            SEARCH(fighter, 0, 0, Hash40::new("top"), 4.0, 0.0, 13.0, 5.5, Some(0.0), Some(3.5), Some(5.5), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, true);
            search!(fighter, *MA_MSC_CMD_SEARCH_SET_OPPONENT, 0, 0, *COLLISION_TARGET_PROPERTY, *COLLISION_PROPERTY_MASK_REFLECT);
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
    if is_excute(fighter) {
        if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
            CATCH(fighter, 0, Hash40::new("top"), 7.5, 0.0, 7.0, 4.0, None, None, None, *FIGHTER_STATUS_KIND_SWALLOWED, *COLLISION_SITUATION_MASK_GA);
            ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            SEARCH(fighter, 0, 0, Hash40::new("top"), 4.0, 0.0, 9.5, 5.5, Some(0.0), Some(2.5), Some(5.5), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, true);
            search!(fighter, *MA_MSC_CMD_SEARCH_SET_OPPONENT, 0, 0, *COLLISION_TARGET_PROPERTY, *COLLISION_PROPERTY_MASK_REFLECT);
        }
        else{
            CATCH(fighter, 0, Hash40::new("top"), 9.0, 0.0, 9.0, 4.0, None, None, None, *FIGHTER_STATUS_KIND_SWALLOWED, *COLLISION_SITUATION_MASK_GA);
            ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            SEARCH(fighter, 0, 0, Hash40::new("top"), 4.0, 0.0, 13.0, 5.5, Some(0.0), Some(3.5), Some(5.5), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, true);
            search!(fighter, *MA_MSC_CMD_SEARCH_SET_OPPONENT, 0, 0, *COLLISION_TARGET_PROPERTY, *COLLISION_PROPERTY_MASK_REFLECT);
        }
    }
}

#[acmd_script( agent = "dedede", script = "game_specialsstart" , category = ACMD_GAME , low_priority)]
unsafe fn dedede_special_s_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_GENERATE);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_PUTOUT);
        FT_MOTION_RATE(fighter, 18.0/(29.0-6.0));
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        WorkModule::on_flag(boma, *FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_THROW);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 100, 0, 30, 7.5, 0.0, 8.0, 14.0, Some(0.0), Some(8.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "dedede", script = "game_specialairsstart" , category = ACMD_GAME , low_priority)]
unsafe fn dedede_special_air_s_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_GENERATE);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_PUTOUT);
        FT_MOTION_RATE(fighter, 18.0/(29.0-6.0));
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        WorkModule::on_flag(boma, *FIGHTER_DEDEDE_STATUS_GORDO_THROW_FLAG_THROW);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 100, 0, 30, 7.5, 0.0, 8.0, 14.0, Some(0.0), Some(8.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "dedede", script = "game_speciallw" , category = ACMD_GAME , low_priority)]
unsafe fn dedede_special_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let rush_speed = 0.9 + 0.01*WorkModule::get_float(boma, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_WORK_FLOAT_HOLD_COUNT);
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
    let rush_speed = 0.9 + 0.01*WorkModule::get_float(boma, *FIGHTER_DEDEDE_STATUS_JET_HAMMER_WORK_FLOAT_HOLD_COUNT);
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
    frame(lua_state, 52.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
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
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 90, 82 , 0, 95, 10.0, 0.0, 8.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 70, 82 , 0, 95, 10.0, 0.0, 8.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 70, 82 , 0, 95, 10.0, 0.0, 8.0, -10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_BODY);
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
        dedede_special_air_n_start_game,
        dedede_special_s_start_game,
        dedede_special_air_s_start_game,
        dedede_special_lw_game,
        dedede_special_lw_max_game,
        dedede_special_air_lw_game,
        dedede_special_air_lw_max_game,
        dedede_special_hi_jump_game,
        dedede_special_hi_landing_game,
    );
}

