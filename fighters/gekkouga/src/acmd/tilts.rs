
use super::*;



unsafe extern "C" fn gekkouga_attack_s3_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneel"), 9.0, 361, 56, 0, 50, 4.0, -7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneel"), 9.0, 361, 56, 0, 50, 4.0, -0.6, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("footl"), 10.0, 361, 90, 0, 52, 4.5, 0.0, -2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 4.0);
    FT_MOTION_RATE(fighter, 0.75);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 38.0);
    FT_MOTION_RATE(fighter, 1.0);
    
}


unsafe extern "C" fn gekkouga_attack_s3_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        EFFECT_FLIP(fighter, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 9, 5, -40, -40, 55, 1.1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 1.7);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("gekkouga_water_impact"), Hash40::new("top"), 0.0, 13.5, 15.5, 0, 0, 0, 1.0, true);
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    
}


unsafe extern "C" fn gekkouga_attack_s3_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneel"), 9.0, 361, 56, 0, 50, 4.0, -7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneel"), 9.0, 361, 56, 0, 50, 4.0, -0.6, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("footl"), 10.0, 361, 90, 0, 52, 4.5, 0.0, -2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 4.0);
    FT_MOTION_RATE(fighter, 0.75);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 38.0);
    FT_MOTION_RATE(fighter, 1.0);
    
}


unsafe extern "C" fn gekkouga_attack_s3_s_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        EFFECT_FLIP(fighter, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 7.8, 7.5, -10, -40, 20, 1.1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 1.7);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("gekkouga_water_impact"), Hash40::new("top"), 0.0, 8.5, 18.5, 0, 0, 0, 1.0, true);
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    
}


unsafe extern "C" fn gekkouga_attack_s3_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 10.0);
    if is_excute(fighter) {        
        ATTACK(fighter, 0, 0, Hash40::new("kneel"), 9.0, 361, 56, 0, 50, 4.0, -7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneel"), 9.0, 361, 56, 0, 50, 4.0, -0.6, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("footl"), 10.0, 361, 90, 0, 52, 4.5, 0.0, -2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 4.0);
    FT_MOTION_RATE(fighter, 0.75);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 38.0);
    FT_MOTION_RATE(fighter, 1.0);
    
}


unsafe extern "C" fn gekkouga_attack_s3_lw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        EFFECT_FLIP(fighter, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 7.5, 8, 30, -60, -10, 1.1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 1.7);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("gekkouga_water_impact"), Hash40::new("top"), 0.0, 3.5, 15.5, 0, 0, 0, 1.0, true);
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    
}


unsafe extern "C" fn gekkouga_attack_hi3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("s_tongue5"), 7.0, 80, 72, 0, 69, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HARISEN, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 1, 0, Hash40::new("s_tongue7"), 7.0, 80, 72, 0, 69, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HARISEN, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 2, 0, Hash40::new("s_tongue9"), 7.0, 80, 72, 0, 69, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HARISEN, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}


unsafe extern "C" fn gekkouga_attack_hi3_effect(fighter: &mut L2CAgentBase) {
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


unsafe extern "C" fn gekkouga_attack_hi3_expression(fighter: &mut L2CAgentBase) {
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


unsafe extern "C" fn gekkouga_attack_lw3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("legl"), 6.0, 65, 80, 0, 62, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("kneel"), 6.0, 77, 80, 0, 62, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("kneel"), 7.0, 85, 59, 0, 66, 5.0, 6.0, -1.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_PUNCH);
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


unsafe extern "C" fn gekkouga_attack_lw3_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 2.5, 2, -7.0, 0, 0, 0, 1.4, true, *EF_FLIP_YZ);
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -5.5, 0, -2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("gekkouga_water_impact"), Hash40::new("top"), 0.0, 2.0, 15.0, 0, 0, 0, 1.0, true);
    }
    
}




pub fn install() {
    smashline::Agent::new("gekkouga")
        .acmd("game_attacks3hi", gekkouga_attack_s3_hi_game)
        .acmd("effect_attacks3hi", gekkouga_attack_s3_hi_effect)
        .acmd("game_attacks3", gekkouga_attack_s3_s_game)
        .acmd("effect_attacks3", gekkouga_attack_s3_s_effect)
        .acmd("game_attacks3lw", gekkouga_attack_s3_lw_game)
        .acmd("effect_attacks3lw", gekkouga_attack_s3_lw_effect)
        .acmd("game_attackhi3", gekkouga_attack_hi3_game)
        .acmd("effect_attackhi3", gekkouga_attack_hi3_effect)
        .acmd("expression_attackhi3", gekkouga_attack_hi3_expression)
        .acmd("game_attacklw3", gekkouga_attack_lw3_game)
        .acmd("effect_attacklw3", gekkouga_attack_lw3_effect)
        .install();
}
