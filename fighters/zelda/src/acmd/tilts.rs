
use super::*;

#[acmd_script( agent = "zelda", script = "game_attacks3hi" , category = ACMD_GAME , low_priority)]
unsafe fn zelda_attack_s3_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 7.0/(10.0-1.0));
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("bust"), 12.0, 361, 75, 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
        ATTACK(fighter, 1, 0, Hash40::new("arml"), 12.0, 361, 75, 0, 60, 2.5, -2.0, 0.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
        ATTACK(fighter, 2, 0, Hash40::new("arml"), 12.0, 361, 75, 0, 60, 2.5, 0.7, 0.3, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
        ATTACK(fighter, 3, 0, Hash40::new("arml"), 9.5, 145, 42, 0, 75, 2.5, 3.2, 0.4, -0.1, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        ATTACK(fighter, 4, 0, Hash40::new("handl"), 9.5, 145, 42, 0, 75, 3.0, 3.5, 0.0, 0.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        ATTACK(fighter, 5, 0, Hash40::new("handl"), 9.5, 145, 42, 0, 75, 3.5, 6.2, 0.0, 1.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        FT_MOTION_RATE(fighter, 0.880);
    }
    
}

#[acmd_script( agent = "zelda", script = "effect_attacks3hi" , category = ACMD_EFFECT , low_priority)]
unsafe fn zelda_attack_s3_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("zelda_atk_flash_s"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 2.0, true);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("zelda_atk_arc"), Hash40::new("zelda_atk_arc"), Hash40::new("top"), 0, 12, 0, -20, 0, 8, 1.05, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "zelda", script = "game_attacks3" , category = ACMD_GAME , low_priority)]
unsafe fn zelda_attack_s3_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 7.0/(10.0-1.0));
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("bust"), 12.0, 361, 75, 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
        ATTACK(fighter, 1, 0, Hash40::new("arml"), 12.0, 361, 75, 0, 60, 2.5, -2.0, 0.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
        ATTACK(fighter, 2, 0, Hash40::new("arml"), 12.0, 361, 75, 0, 60, 2.5, 0.7, 0.3, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
        ATTACK(fighter, 3, 0, Hash40::new("arml"), 9.5, 110, 52, 0, 80, 2.5, 3.2, 0.4, -0.1, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        ATTACK(fighter, 4, 0, Hash40::new("handl"), 9.5, 110, 52, 0, 80, 3.0, 3.5, 0.0, 0.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        ATTACK(fighter, 5, 0, Hash40::new("handl"), 9.5, 110, 52, 0, 80, 3.5, 6.2, 0.0, 1.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        FT_MOTION_RATE(fighter, 0.880);
    }
    
}

#[acmd_script( agent = "zelda", script = "effect_attacks3" , category = ACMD_EFFECT , low_priority)]
unsafe fn zelda_attack_s3_s_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("zelda_atk_flash_s"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 2.0, true);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("zelda_atk_arc"), Hash40::new("zelda_atk_arc"), Hash40::new("top"), 0, 11.3, 0, 3, 0, 5, 1.05, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "zelda", script = "expression_attacks3", category = ACMD_EXPRESSION, low_priority )]
unsafe fn zelda_attack_s3_s_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 3);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
    }
}

#[acmd_script( agent = "zelda", script = "expression_attacks3hi", category = ACMD_EXPRESSION, low_priority )]
unsafe fn zelda_attack_s3_hi_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 3);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
    }
}

#[acmd_script( agent = "zelda", script = "expression_attacks3lw", category = ACMD_EXPRESSION, low_priority )]
unsafe fn zelda_attack_s3_lw_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 3);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
    }
}


#[acmd_script( agent = "zelda", script = "game_attacks3lw" , category = ACMD_GAME , low_priority)]
unsafe fn zelda_attack_s3_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 7.0/(10.0-1.0));
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("bust"), 12.0, 361, 75, 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
        ATTACK(fighter, 1, 0, Hash40::new("arml"), 12.0, 361, 75, 0, 60, 2.5, -2.0, 0.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
        ATTACK(fighter, 2, 0, Hash40::new("arml"), 12.0, 361, 75, 0, 60, 2.5, 0.7, 0.3, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
        ATTACK(fighter, 3, 0, Hash40::new("arml"), 9.5, 130, 52, 0, 60, 2.5, 3.2, 0.4, -0.1, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        ATTACK(fighter, 4, 0, Hash40::new("handl"), 9.5, 130, 52, 0, 60, 3.0, 3.5, 0.0, 0.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        ATTACK(fighter, 5, 0, Hash40::new("handl"), 9.5, 130, 52, 0, 60, 4.0, 6.2, 0.0, 1.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        FT_MOTION_RATE(fighter, 0.880);
    }
    
}

#[acmd_script( agent = "zelda", script = "effect_attacks3lw" , category = ACMD_EFFECT , low_priority)]
unsafe fn zelda_attack_s3_lw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("zelda_atk_flash_s"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 2.0, true);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("zelda_atk_arc"), Hash40::new("zelda_atk_arc"), Hash40::new("top"), 0, 9, 0, 22, 0, 3, 1.05, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "zelda", script = "game_attackhi3" , category = ACMD_GAME , low_priority)]
unsafe fn zelda_attack_hi3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
        ATTACK(fighter, 0, 0, Hash40::new("armr"), 9.5, 100, 65, 0, 80, 4.5, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        ATTACK(fighter, 1, 0, Hash40::new("armr"), 9.5, 100, 65, 0, 80, 4.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        ATTACK(fighter, 2, 0, Hash40::new("shoulderr"), 9.5, 100, 65, 0, 80, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
    }
    wait(lua_state, 15.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    
}

#[acmd_script( agent = "zelda", script = "expression_attackhi3", category = ACMD_EXPRESSION, low_priority )]
unsafe fn zelda_attack_hi3_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
}

#[acmd_script( agent = "zelda", script = "effect_attackhi3" , category = ACMD_EFFECT , low_priority)]
unsafe fn zelda_attack_hi3_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(fighter, Hash40::new("zelda_atk_hi_flash"), Hash40::new("armr"), 4.5, 0, 0, 0, 0, 0, 1, true);
    }
}

#[acmd_script( agent = "zelda", script = "game_attacklw3" , category = ACMD_GAME , low_priority)]
unsafe fn zelda_attack_lw3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("legl"), 6.5, 80, 85, 0, 47, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("legl"), 6.5, 80, 85, 0, 47, 3.8, 4.5, 0.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("kneel"), 10.0, 90, 60, 0, 60, 3.8, 6.0, 0.0, 2.8, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
        ATTACK(fighter, 3, 0, Hash40::new("kneel"), 10.0, 270, 65, 0, 80, 3.8, 6.0, 0.0, 2.8, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "zelda", script = "effect_attacklw3" , category = ACMD_EFFECT , low_priority)]
unsafe fn zelda_attack_lw3_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 0.35, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), -1, 4.6, -6, 14, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);

        EFFECT_FOLLOW(fighter, Hash40::new("zelda_atk"), Hash40::new("top"), 2.0, 3.0, 12.0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_RATE(fighter, 1.25);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        
    }
}

pub fn install() {
    install_acmd_scripts!(
        zelda_attack_s3_hi_game,
        zelda_attack_s3_hi_effect,
        zelda_attack_s3_hi_expression,
        zelda_attack_s3_s_game,
        zelda_attack_s3_s_effect,
        zelda_attack_s3_s_expression,
        zelda_attack_s3_lw_game,
        zelda_attack_s3_lw_effect,
        zelda_attack_s3_lw_expression,
        zelda_attack_hi3_game,
        zelda_attack_hi3_effect,
        zelda_attack_hi3_expression,
        zelda_attack_lw3_game,
        zelda_attack_lw3_effect,
    );
}

