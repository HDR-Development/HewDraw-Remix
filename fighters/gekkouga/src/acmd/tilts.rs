
use super::*;


#[acmd_script( agent = "gekkouga", script = "game_attacks3hi" , category = ACMD_GAME , low_priority)]
unsafe fn gekkouga_attack_s3_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneel"), 8.0, 361, 56, 0, 50, 4.0, -0.6, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneel"), 9.0, 361, 77, 0, 50, 4.0, -7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("footl"), 9.0, 361, 88, 0, 50, 4.5, 0.0, -2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_KICK);
        ATTACK(fighter, 3, 0, Hash40::new("footl"), 9.0, 361, 88, 0, 50, 4.5, 0.0, -6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        FT_MOTION_RATE(fighter, 0.750);
    }
    frame(lua_state, 38.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
    }
    
}

#[acmd_script( agent = "gekkouga", script = "effect_attacks3hi" , category = ACMD_EFFECT , low_priority)]
unsafe fn gekkouga_attack_s3_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        EFFECT_FLIP(fighter, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 9, 5, -40, -40, 55, 1.1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 1.7);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("gekkouga_water_impact"), Hash40::new("top"), 0.0, 13.5, 20.0, 0, 0, 0, 1.0, true);
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    
}

#[acmd_script( agent = "gekkouga", script = "game_attacks3" , category = ACMD_GAME , low_priority)]
unsafe fn gekkouga_attack_s3_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneel"), 8.0, 361, 56, 0, 50, 4.0, -0.6, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneel"), 9.0, 361, 77, 0, 50, 4.0, -7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("footl"), 9.0, 361, 88, 0, 50, 4.5, 0.0, -2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_KICK);
        ATTACK(fighter, 3, 0, Hash40::new("footl"), 9.0, 361, 88, 0, 50, 4.5, 0.0, -6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        FT_MOTION_RATE(fighter, 0.750);
    }
    frame(lua_state, 38.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
    }
    
}

#[acmd_script( agent = "gekkouga", script = "effect_attacks3" , category = ACMD_EFFECT , low_priority)]
unsafe fn gekkouga_attack_s3_s_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        EFFECT_FLIP(fighter, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 7.8, 7.5, -10, -40, 20, 1.1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 1.7);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("gekkouga_water_impact"), Hash40::new("top"), 0.0, 8.5, 22.5, 0, 0, 0, 1.0, true);
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    
}

#[acmd_script( agent = "gekkouga", script = "game_attacks3lw" , category = ACMD_GAME , low_priority)]
unsafe fn gekkouga_attack_s3_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {        
        ATTACK(fighter, 0, 0, Hash40::new("kneel"), 8.0, 361, 56, 0, 50, 4.0, -0.6, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneel"), 9.0, 361, 77, 0, 50, 4.0, -7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("footl"), 9.0, 361, 88, 0, 50, 4.5, 0.0, -2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_KICK);
        ATTACK(fighter, 3, 0, Hash40::new("footl"), 9.0, 361, 88, 0, 50, 4.5, 0.0, -4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        FT_MOTION_RATE(fighter, 0.750);
    }
    frame(lua_state, 38.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.150);
    }
    
}

#[acmd_script( agent = "gekkouga", script = "effect_attacks3lw" , category = ACMD_EFFECT , low_priority)]
unsafe fn gekkouga_attack_s3_lw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        EFFECT_FLIP(fighter, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 7.5, 8, 30, -60, -10, 1.1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 1.7);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("gekkouga_water_impact"), Hash40::new("top"), 0.0, 3.5, 18.5, 0, 0, 0, 1.0, true);
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    
}

#[acmd_script( agent = "gekkouga", script = "game_attackhi3" , category = ACMD_GAME , low_priority)]
unsafe fn gekkouga_attack_hi3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        //Only for hitting non-fighter things (such as substitute doll)
        ATTACK(fighter, 0, 0, Hash40::new_raw(0x09de54b48f), 5.0, 80, 83, 0, 110, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HARISEN, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 1, 0, Hash40::new_raw(0x09305ad5a3), 5.0, 70, 83, 0, 110, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HARISEN, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 2, 0, Hash40::new_raw(0x09d7e2f8a4), 5.0, 60, 83, 0, 110, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HARISEN, *ATTACK_REGION_HEAD);
        //Hits fighters
        ATTACK(fighter, 3, 0, Hash40::new_raw(0x09de54b48f), 5.0, 80, 123, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HARISEN, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 4, 0, Hash40::new_raw(0x09305ad5a3), 5.0, 70, 123, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HARISEN, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 5, 0, Hash40::new_raw(0x09d7e2f8a4), 5.0, 70, 123, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HARISEN, *ATTACK_REGION_HEAD);

    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "gekkouga", script = "effect_attackhi3" , category = ACMD_EFFECT , low_priority)]
unsafe fn gekkouga_attack_hi3_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("gekkouga_attack_hi"), Hash40::new("gekkouga_attack_hi"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true, *EF_FLIP_XY);
        EFFECT_FLW_POS(fighter, Hash40::new("gekkouga_splash"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 1.2, true);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -5.5, 0, -2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("gekkouga_attack_arc"), Hash40::new("gekkouga_attack_arc"), Hash40::new("top"), -2, 13, 1.5, 152, -35, 97, 0.9, true, *EF_FLIP_YZ, 0.6);
        LAST_EFFECT_SET_COLOR(fighter, 2, 2, 2);
        LAST_EFFECT_SET_RATE(fighter, 2);
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("gekkouga_attack_arc"), Hash40::new("gekkouga_attack_arc"), Hash40::new("top"), -2, 13, 1.5, 152, -35, 97, 0.75, true, *EF_FLIP_YZ, 0.5);
        LAST_EFFECT_SET_COLOR(fighter, 1.4, 1.4, 1.4);
        LAST_EFFECT_SET_RATE(fighter, 2);
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("gekkouga_attack_arc"), Hash40::new("gekkouga_attack_arc"), Hash40::new("top"), -2, 13, 1.5, 152, -35, 97, 0.55, true, *EF_FLIP_YZ, 0.4);
        LAST_EFFECT_SET_COLOR(fighter, 0.8, 0.8, 0.8);
        LAST_EFFECT_SET_RATE(fighter, 2);
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("gekkouga_attack_arc"), Hash40::new("gekkouga_attack_arc"), Hash40::new("top"), -2, 13, 1.5, 152, -35, 97, 0.3, true, *EF_FLIP_YZ, 0.3);
        LAST_EFFECT_SET_COLOR(fighter, 0.6, 0.6, 0.6);
        LAST_EFFECT_SET_RATE(fighter, 2);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_spin_wind_s"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("gekkouga_attack_arc"), true, true);
    }
    
}

#[acmd_script( agent = "gekkouga", script = "expression_attackhi3", category = ACMD_EXPRESSION, low_priority )]
unsafe fn gekkouga_attack_hi3_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
}

#[acmd_script( agent = "gekkouga", script = "game_attacklw3" , category = ACMD_GAME , low_priority)]
unsafe fn gekkouga_attack_lw3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 4.0, 65, 100, 0, 60, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("legl"), 4.0, 65, 100, 0, 60, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("kneel"), 5.0, 77, 91, 0, 60, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("kneel"), 6.0, 85, 67, 0, 65, 6.0, 6.0, -1.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 4, 0, Hash40::new("kneel"), 6.0, 85, 67, 0, 65, 6.0, 8.0, -1.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SQUAT_WAIT, false);
    }
    
}

#[acmd_script( agent = "gekkouga", script = "effect_attacklw3" , category = ACMD_EFFECT , low_priority)]
unsafe fn gekkouga_attack_lw3_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 2.5, 2, -4.5, 0, 0, 0, 1.4, true, *EF_FLIP_YZ);
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -5.5, 0, -2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new_raw(0x1156ac182a), Hash40::new("footl"), 0.0, 0.0, 0.0, 0, 0, 0, 1.5, true, 1.0);
        EFFECT_FOLLOW(fighter, Hash40::new("gekkouga_water_impact"), Hash40::new("top"), 0.0, 2.0, 20.0, 0, 0, 0, 1.0, true);
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        gekkouga_attack_s3_hi_game,
        gekkouga_attack_s3_hi_effect,
        gekkouga_attack_s3_s_game,
        gekkouga_attack_s3_s_effect,
        gekkouga_attack_s3_lw_game,
        gekkouga_attack_s3_lw_effect,
        gekkouga_attack_hi3_game,
        gekkouga_attack_hi3_effect,
        gekkouga_attack_hi3_expression,
        gekkouga_attack_lw3_game,
        gekkouga_attack_lw3_effect,
    );
}

