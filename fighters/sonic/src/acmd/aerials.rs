
use super::*;


#[acmd_script( agent = "sonic", script = "game_attackairn" , category = ACMD_GAME , low_priority)]
unsafe fn sonic_attack_air_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 4.0/(8.0 - 1.0));
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 8.0);
    FT_MOTION_RATE(fighter, 0.7);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 14.0, 361, 105, 0, 20, 3.0, 2.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"),  *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 14.0, 361, 105, 0, 25, 4.0, 7.5, -0.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        AttackModule::clear(boma,1,false);
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 8.0, 361, 105, 0, 15, 3.0, 2.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 8.0, 361, 105, 0, 15, 3.5, 7.7, -0.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    
    }
    frame(lua_state, 34.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 39.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "sonic", script = "expression_attackairn", category = ACMD_EXPRESSION, low_priority )]
unsafe fn expression_attackairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 4.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_elecattack"), 8, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        ItemModule::set_attach_item_visibility(boma, false, 0);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }

    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_elecattacks"), 8, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_elecattacks"), 8, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_elecattacks"), 8, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 39.0);
    if is_excute(agent) {
        ItemModule::set_attach_item_visibility(boma, true, 0);
        ItemModule::set_have_item_visibility(boma, true, 0);
    }
    if is_excute(agent) {
        VisibilityModule::set_int64(boma, hash40("body") as i64, hash40("body_normal") as i64);
        HIT_NODE(agent, Hash40::new("waist"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("s_stingd1"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("footr"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("footl"), *HIT_STATUS_NORMAL);
        HIT_NODE(agent, Hash40::new("rot"), *HIT_STATUS_OFF);
    }
}

#[acmd_script( agent = "sonic", script = "sound_attackairn", category = ACMD_SOUND, low_priority )]
unsafe fn sound_attackairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_sonic_rnd_attack"));
        PLAY_SE(agent, Hash40::new("se_sonic_attackair_f01"));
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_sonic_attackair_f02"));
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_sonic_attackair_f03"));
    }
}

#[acmd_script( agent = "sonic", script = "effect_attackairn", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_attackairn(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sonic_spinwind"), Hash40::new("top"), 1, 9.4, 0, 75, 180, 115, 1.4, true);
        LAST_EFFECT_SET_COLOR(agent, 0.117, 0.262, 1);
        EFFECT_FOLLOW(agent, Hash40::new("sonic_spinwind"), Hash40::new("top"), 1, 9.4, 0, 75, 180, 115, 1.6, true);
        LAST_EFFECT_SET_COLOR(agent, 0.458, 0.603, 1);
    }
   

    frame(lua_state, 36.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("sonic_spinwind"), false, false);
    }
    
}

    #[acmd_script( agent = "sonic", script = "expression_landingairn", category = ACMD_EXPRESSION, low_priority )]
unsafe fn expression_landingairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_lands"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }

}


#[acmd_script( agent = "sonic", script = "game_attackairf" , category = ACMD_GAME , low_priority)]
unsafe fn sonic_attack_air_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    FT_MOTION_RATE(fighter, 5.0/6.0);
    frame(lua_state, 4.8);  // effectively f4
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 9.0, 55, 100, 0, 30, 3.0, -2.0, 0.3, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("footr"), 11.0, 361, 90, 0, 45, 5.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 9.6);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 9.0, 55, 100, 0, 30, 3.0, -2.0, 0.3, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("footr"), 11.0, 270, 52, 0, 25, 5.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("footr"), 11.0, 270, 50, 0, 25, 5.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 33.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 34.0);
    FT_MOTION_RATE(fighter, 0.5);
    frame(lua_state, 40.0);
    FT_MOTION_RATE(fighter, 1.0);
}


#[acmd_script( agent = "sonic", script = "effect_attackairf" , category = ACMD_EFFECT , low_priority)]
unsafe fn sonic_attack_air_f_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new_raw(0x10e97de698), Hash40::new("top"), 0, 9.5, 1.0, 180, -150 , 90, 0.92, true);
        LAST_EFFECT_SET_COLOR(fighter, 3.0, 0.15, 0.15);
        LAST_EFFECT_SET_RATE(fighter, 2.0);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new_raw(0x1156ac182a), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 1.3, true, 0.9);
        LAST_EFFECT_SET_RATE(fighter, 2);
    }

}


#[acmd_script( agent = "sonic", script = "sound_attackairf" , category = ACMD_SOUND , low_priority)]
unsafe fn sonic_attack_air_f_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new_raw(0x1031b0368f));
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new_raw(0x141b86df88));
        PLAY_SE(fighter, Hash40::new_raw(0x1651c5e760));
    }

}


#[acmd_script( agent = "sonic", script = "game_attackairb" , category = ACMD_GAME , low_priority)]
unsafe fn sonic_attack_air_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.7);
    frame(lua_state, 5.0);
    FT_MOTION_RATE(fighter, 0.75);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 15.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 11.0, 361, 94, 0, 30, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 14.0, 361, 94, 0, 30, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("kneer"), 14.0, 361, 94, 0, 30, 5.5, 4.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 7.0, 361, 80, 0, 40, 3.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 10.0, 361, 80, 0, 40, 4.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("kneer"), 10.0, 361, 80, 0, 40, 5.3, 4.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

}

#[acmd_script( agent = "sonic", script = "effect_attackairb" , category = ACMD_EFFECT , low_priority)]
unsafe fn sonic_attack_air_b_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    frame(lua_state, 13.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 6.0, -4.0, 165, 65, 150, 1.1, true, *EF_FLIP_YZ);
	    LAST_EFFECT_SET_RATE(fighter, 1.75);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_arc"), true, true);
    }

}

#[acmd_script( agent = "sonic", script = "game_attackairhi" , category = ACMD_GAME , low_priority)]
unsafe fn sonic_attack_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    FT_MOTION_RATE(fighter, 5.0/(4.0-0.0));
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 4.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 110, 100, 90, 0, 3.5, 0.0, 7.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 110, 100, 90, 0, 3.5, 0.0, 7.0, -1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 4.0, 123, 100, 120, 0, 4.0, 0.0, 8.0, 7.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 4.0, 123, 100, 120, 0, 4.0, 0.0, 8.0, -7.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 7.0);
    FT_MOTION_RATE(fighter, 0.333);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 13.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 90, 85, 0, 52, 5.0, 0.0, 19.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 90, 85, 0, 52, 4.5, 0.0, 15.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 90, 85, 0, 52, 4.0, 0.0, 11.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 36.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

}

#[acmd_script( agent = "sonic", script = "effect_attackairhi" , category = ACMD_EFFECT , low_priority)]
unsafe fn sonic_attack_air_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new_raw(0x1441eaf0b3), Hash40::new("top"), 0, 7.5, 2.5, -10, 0, 0, 0.5, true);
        LAST_EFFECT_SET_COLOR(fighter, 3.0, 0.7, 0.2);
        LAST_EFFECT_SET_COLOR(fighter, 3.0, 0.15, 0.15);
        EFFECT_FOLLOW(fighter, Hash40::new_raw(0x1441eaf0b3), Hash40::new("top"), -1, 7.5, -2.5, 190, 0, 0, 0.5, true);
        LAST_EFFECT_SET_COLOR(fighter, 3.0, 0.7, 0.2);
        LAST_EFFECT_SET_COLOR(fighter, 3.0, 0.15, 0.15);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP_FLIP(fighter, Hash40::new_raw(0x0c824d7183), Hash40::new_raw(0x0c824d7183), Hash40::new("top"), 0.85, 11.5, 0, 59, 74, 154, 1.1, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 3.0, 0.7, 0.2);
        LAST_EFFECT_SET_COLOR(fighter, 3.0, 0.15, 0.15);
        EFFECT_FOLLOW_NO_STOP_FLIP(fighter, Hash40::new_raw(0x0c824d7183), Hash40::new_raw(0x0c824d7183), Hash40::new("top"), -0.95, 11.5, 0, 245, -77, 22, 1.1, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, 3.0, 0.7, 0.2);
        LAST_EFFECT_SET_COLOR(fighter, 3.0, 0.15, 0.15);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new_raw(0x1156ac182a), Hash40::new("top"), 0.0, 19.5, 0.0, 0, 0, 0, 1.55, 0, 0, 0, 0, 0, 361, true);
    }

}

#[acmd_script( agent = "sonic", script = "game_attackairlw" , category = ACMD_GAME , low_priority)]
unsafe fn sonic_attack_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    FT_MOTION_RATE(fighter, 10.0/(16.0-0.0));
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 16.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        SET_SPEED_EX(fighter, 1.4, -4, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 287, 76, 0, 13, 5.0, -4.0, 5.0, 1.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 37, 88, 0, 45, 4.1, 0.0, -0.3, 3.1, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 60, 66, 0, 30, 5.0, -4.0, 5.0, 1.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 361, 88, 0, 45, 4.1, 0.0, 1.3, 3.1, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        AttackModule::clear(boma, 1, false);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 361, 75, 0, 45, 4.3, -4.0, 6.0, 1.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 33.0);
    FT_MOTION_RATE(fighter, 14.0/(49.0-33.0));
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 61.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
        KineticModule::resume_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }

}

pub fn install() {
    install_acmd_scripts!(
        sonic_attack_air_n_game,
        sonic_attack_air_f_game,
        sonic_attack_air_f_effect,
        sonic_attack_air_f_sound,
        sonic_attack_air_b_game,
        sonic_attack_air_b_effect,
        sonic_attack_air_hi_game,
        sonic_attack_air_hi_effect,
        sonic_attack_air_lw_game,
        expression_attackairn,
        effect_attackairn,
        sound_attackairn,
        expression_landingairn,
    );
}
