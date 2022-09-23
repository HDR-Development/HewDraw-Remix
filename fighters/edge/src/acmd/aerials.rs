
use super::*;


#[acmd_script( agent = "edge", script = "game_attackairn" , category = ACMD_GAME , low_priority)]
unsafe fn sephiroth_attack_air_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
	if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 0.3);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 8.5, 60, 77, 0, 64, 3.5, 0.0, 10.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 8.5, 60, 77, 0, 64, 12.5, 0.0, 10.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 8.5, 60, 77, 0, 64, 12.5, 0.0, 10.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 8.5, 60, 77, 0, 64, 12.5, 0.0, 10.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("armr"), 6.5, 50, 70, 0, 65, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        ATTACK(fighter, 1, 0, Hash40::new("legl"), 6.5, 50, 70, 0, 65, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        ATTACK(fighter, 2, 0, Hash40::new("kneel"), 6.5, 50, 70, 0, 65, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        ATTACK(fighter, 3, 0, Hash40::new("kneer"), 6.5, 50, 70, 0, 65, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
    }
    wait(lua_state, 9.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
	frame(lua_state, 28.0);
    if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 0.824);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
	frame(lua_state, 45.0);
    if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
    }
    
}

#[acmd_script( agent = "edge", script = "effect_attackairn" , category = ACMD_EFFECT , low_priority)]
unsafe fn sephiroth_attack_air_n_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("edge_light_finger_aura"), Hash40::new("handr"), 2.5, 1, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("edge_light_finger_snapping"), -1);
        EFFECT_FLW_POS(fighter, Hash40::new("edge_attack_dash_aura"), Hash40::new("handr"), 2, 0, 0, 0, 0, 0, 1.25, true);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
        EFFECT_FLW_POS(fighter, Hash40::new("edge_attack_dash2"), Hash40::new("handr"), 2, 1, 0, 0, 0, 0, 0.8, true);
        LAST_EFFECT_SET_RATE(fighter, 1.1);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT_FLW_POS(fighter, Hash40::new("edge_attack_dash"), Hash40::new("handr"), 2, 1, 0, 0, 0, 0, 0.7, true);
        LAST_EFFECT_SET_RATE(fighter, 1.1);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT_FLW_POS(fighter, Hash40::new("edge_attack_dash"), Hash40::new("legl"), 1, 0, 0, 0, 0, 0, 0.7, true);
        LAST_EFFECT_SET_RATE(fighter, 1.1);
        EffectModule::enable_sync_init_pos_last(boma);
        EFFECT_FLW_POS(fighter, Hash40::new("edge_attack_dash"), Hash40::new("kneer"), 1, 0, 0, 0, 0, 0, 0.7, true);
        LAST_EFFECT_SET_RATE(fighter, 1.1);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("edge_light_finger_aura"), false, true);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("edge_light_impact2"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1, true);
        EFFECT_FLW_POS(fighter, Hash40::new("edge_light_impact"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("edge_light_finger_snapping"), Hash40::new("handr"), 3.5, 1, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(boma);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("edge_light_impact2"), false, false);
    }
    
}

#[acmd_script( agent = "edge", script = "game_attackairf" , category = ACMD_GAME , low_priority)]
unsafe fn sephiroth_attack_air_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 4.0);
    FT_MOTION_RATE(fighter, 0.858); // F12 startup
	//FT_MOTION_RATE(fighter, 0.57); // F10 startup
    frame(lua_state, 11.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_EDGE_STATUS_ATTACK_AIR_F_WORK_FLAG_PIERCE_GROUND);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 3.0/(15.0-13.0));
        ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 7.5, 361, 86, 0, 34, 4.0, 0.0, 0.0, 2.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("arml"), 7.5, 361, 86, 0, 34, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 4, 0, Hash40::new("swordl1"), 12.0, 361, 86, 0, 39, 2.5, 18.0, 0.0, 0.4, Some(25.0), Some(0.0), Some(0.8), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("swordl1"), 10.0, 361, 86, 0, 39, 3.0, 10.5, 0.0, -0.2, Some(15.5), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("swordl1"), 7.5, 361, 86, 0, 34, 3.0, 0.0, 0.0, -0.7, Some(5.5), Some(0.0), Some(-0.4), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_EDGE_STATUS_ATTACK_AIR_F_WORK_FLAG_PIERCE_GROUND);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 5.0, 361, 81, 0, 39, 3.5, 0.0, 0.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("arml"), 5.0, 361, 81, 0, 39, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 4, 0, Hash40::new("swordl1"), 8.0, 55, 81, 0, 39, 2.0, 18.0, 0.0, 0.4, Some(24.5), Some(0.0), Some(0.8), 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("swordl1"), 7.0, 50, 81, 0, 39, 2.5, 10.5, 0.0, -0.2, Some(15.5), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("swordl1"), 5.0, 50, 81, 0, 39, 2.5, 0.0, 0.0, -0.7, Some(5.5), Some(0.0), Some(-0.4), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 33.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 65.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    
}

#[acmd_script( agent = "edge", script = "effect_attackairf" , category = ACMD_EFFECT , low_priority)]
unsafe fn sephiroth_attack_air_f_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("edge_sword_light3"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 1.4);

        EFFECT_FLW_POS(fighter, Hash40::new("edge_attack_dash"), Hash40::new("swordl2"), 22, 0, 0.8, 0, 0, 0, 0.7, true);
        LAST_EFFECT_SET_RATE(fighter, 1.1);
        EFFECT_FLW_POS(fighter, Hash40::new("edge_attack_dash_aura"), Hash40::new("swordl2"), 22, 0, 0.8, 0, 0, 0, 0.7, true);
        LAST_EFFECT_SET_RATE(fighter, 1.1);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        EFFECT_FLW_POS(fighter, Hash40::new("edge_attack_speedline"), Hash40::new("swordl2"), -26, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
        LAST_EFFECT_SET_ALPHA(fighter, 0.25);
        EFFECT_FLW_POS(fighter, Hash40::new("edge_sword_flash2"), Hash40::new("swordl2"), 22, 0, 0.8, 0, 0, 0, 1.2, true);

        EFFECT(fighter, Hash40::new("edge_attack_dash2"), Hash40::new("swordl2"), 22.0, 0, 0.8, 0, 0, 0, 0.65, 0, 0, 0, 0, 0, 0, true);
        //EFFECT_FLW_POS(fighter, Hash40::new("edge_attack_dash2"), Hash40::new("swordl2"), 22.0, 0, 0.8, 0, 0, 0, 0.65, true);
        LAST_EFFECT_SET_RATE(fighter, 1.0);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("edge_attack_dash"), -1);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("edge_sword_flash2"), -1);
        EFFECT_OFF_KIND(fighter, Hash40::new("edge_attack_speedline"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("edge_attack_dash"), false, false);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("edge_attack_dash_aura"), -1);
    }
}

#[acmd_script( agent = "edge", script = "game_attackairb" , category = ACMD_GAME , low_priority)]
unsafe fn sephiroth_attack_air_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.7);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0/(15.6-15.0));
    }
    frame(lua_state, 15.6);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 2.0/(17.0-15.6));
        ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 8.0, 45, 102, 0, 45, 3.5, 0.0, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("arml"), 8.0, 45, 102, 0, 45, 3.5, 0.0, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("swordl1"), 8.0, 45, 102, 0, 45, 4.0, -1.5, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("swordl1"), 8.0, 45, 102, 0, 45, 4.0, 5.5, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 4, 0, Hash40::new("swordl1"), 14.5, 42, 87, 0, 45, 5.0, 11.0, 0.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 5, 0, Hash40::new("swordl1"), 14.5, 42, 87, 0, 45, 5.0, 15.0, 0.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 6, 0, Hash40::new("swordl1"), 11.5, 42, 104, 0, 39, 4.0, 22.0, 0.0, 0.0,None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 37.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

#[acmd_script( agent = "edge", script = "effect_attackairb" , category = ACMD_EFFECT , low_priority)]
unsafe fn sephiroth_attack_air_b_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("edge_sword_smash_flash"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        EFFECT_FLW_POS(fighter, Hash40::new("edge_attack_dash"), Hash40::new("swordl2"), 11.0, 0, 0.8, 0, 0, 0, 0.8, true);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
        EFFECT_FLW_POS(fighter, Hash40::new("edge_attack_dash"), Hash40::new("swordl2"), 15.0, 0, 0.8, 0, 0, 0, 0.8, true);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
        EFFECT_FLW_POS(fighter, Hash40::new("edge_attack_dash_aura"), Hash40::new("swordl2"), 11.0, 0, 0.8, 0, 0, 0, 0.8, true);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
        EFFECT_FLW_POS(fighter, Hash40::new("edge_attack_dash_aura"), Hash40::new("swordl2"), 15.0, 0, 0.8, 0, 0, 0, 0.8, true);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("edge_sword_flare"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_edge_sword3"), Hash40::new("tex_edge_sword4"), 5, Hash40::new("swordl2"), -4.0, 0.0, -0.6, Hash40::new("swordl2"), 29.0, 0.0, 1.4, true, Hash40::new("null"), Hash40::new("swordl2"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        EFFECT_FOLLOW(fighter, Hash40::new("edge_sword_light3"), Hash40::new("swordl2"), 0, 0, 0, 0, -180, -90, 1, true);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("edge_slash_arc"), Hash40::new("top"), -2, 12.0, -1.6, 0, -150, 15, 1.17, true);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("edge_slash_arc"), -1);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("edge_sword_flare"), false, false);
        AFTER_IMAGE_OFF(fighter, 5);
        EFFECT_DETACH_KIND(fighter, Hash40::new("edge_attack_dash"), -1);
        EFFECT_DETACH_KIND(fighter, Hash40::new("edge_attack_dash_aura"), -1);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("edge_attack_dash"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("edge_attack_dash_aura"), false, false);
    }
}

#[acmd_script( agent = "edge", script = "game_attackairhi" , category = ACMD_GAME , low_priority)]
unsafe fn sephiroth_attack_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.7);
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 10.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("arml"), 6.0, 64, 96, 0, 55, 3.5, -1.0, 0.0, 1.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("swordl1"), 6.0, 64, 96, 0, 55, 4.0, 0.5, 0.0, 1.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("swordl1"), 6.0, 64, 96, 0, 55, 4.5, 5.0, 0.0, 1.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("swordl1"), 11.0, 106, 96, 0, 55, 4.5, 11.5, 0.0, 1.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 4, 0, Hash40::new("swordl1"), 11.0, 106, 96, 0, 55, 4.5, 15.0, 0.0, 1.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 5, 0, Hash40::new("swordl1"), 8.5, 106, 96, 0, 55, 4.0, 22.0, 0.0, 1.0,  None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        ATTACK(fighter, 2, 0, Hash40::new("swordl1"), 6.0, 64, 96, 0, 55, 4.5, 5.0, 0.0, 1.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        ATTACK(fighter, 3, 0, Hash40::new("swordl1"), 11.0, 74, 96, 0, 55, 4.5, 15.0, 0.0, 1.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 4, 0, Hash40::new("swordl1"), 11.0, 74, 96, 0, 55, 4.5, 11.5, 0.0, 1.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 5, 0, Hash40::new("swordl1"), 8.5, 74, 96, 0, 55, 4.0, 22.0, 0.0, 1.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 52.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }   
}

#[acmd_script( agent = "edge", script = "effect_attackairhi" , category = ACMD_EFFECT , low_priority)]
unsafe fn sephiroth_attack_air_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("edge_sword_smash_flash"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 16, 19, 12, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        EFFECT_FLW_POS(fighter, Hash40::new("edge_attack_dash"), Hash40::new("swordl2"), 11.0, 0, 0.8, 0, 0, 0, 0.725, true);
        LAST_EFFECT_SET_RATE(fighter, 0.9);
        EFFECT_FLW_POS(fighter, Hash40::new("edge_attack_dash"), Hash40::new("swordl2"), 15.0, 0, 0.8, 0, 0, 0, 0.725, true);
        LAST_EFFECT_SET_RATE(fighter, 0.9);
        EFFECT_FLW_POS(fighter, Hash40::new("edge_attack_dash_aura"), Hash40::new("swordl2"), 11.0, 0, 0.8, 0, 0, 0, 0.725, true);
        LAST_EFFECT_SET_RATE(fighter, 0.9);
        EFFECT_FLW_POS(fighter, Hash40::new("edge_attack_dash_aura"), Hash40::new("swordl2"), 15.0, 0, 0.8, 0, 0, 0, 0.725, true);
        LAST_EFFECT_SET_RATE(fighter, 0.9);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("edge_sword_flare"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_edge_sword1"), Hash40::new("tex_edge_sword2"), 6, Hash40::new("swordl2"), -4.0, 0.0, -0.6, Hash40::new("swordl2"), 29.0, 0.0, 1.4, true, Hash40::new("null"), Hash40::new("swordl2"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        EFFECT_FOLLOW(fighter, Hash40::new("edge_sword_light3"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("edge_slash_light"), Hash40::new("top"), 0, 10, -2.7, 2.5, 120, 90, 1.05, true);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("edge_slash_light"), -1);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("edge_sword_smash_flash"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("edge_sword_flare"), false, false);
        AFTER_IMAGE_OFF(fighter, 5);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("edge_attack_dash"), -1);
        EFFECT_DETACH_KIND(fighter, Hash40::new("edge_attack_dash_aura"), -1);
    }
    frame(lua_state, 32.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("edge_attack_dash"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("edge_attack_dash_aura"), false, false);
    }
}

#[acmd_script( agent = "edge", script = "game_attackairlw" , category = ACMD_GAME , low_priority)]
unsafe fn sephiroth_attack_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_LANDING_CLEAR_SPEED);
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        SET_SPEED_EX(fighter, 0, 1, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::suspend_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 1.0, 4.0, 8.0, 3.0);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    FT_MOTION_RATE(fighter, 1.2);
    frame(lua_state, 15.0);
    FT_MOTION_RATE(fighter, 1);
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 2.0, 4.0);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        SET_SPEED_EX(fighter, 0, -4.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        //ATTACK(fighter, 0, 0, Hash40::new("swordl1"), 10.0, 48, 76, 0, 58, 4.0, 0.5, 0.0, 1.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("swordl1"), 10.0, 52, 76, 0, 58, 4.0, 5.0, 0.0, 1.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("swordl1"), 10.0, 52, 76, 0, 58, 4.0, 9.5, 0.0, 1.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("swordl1"), 10.0, 52, 76, 0, 58, 4.0, 15.0, 0.0, 1.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // Ground only
        ATTACK(fighter, 4, 0, Hash40::new("swordl1"), 15.0, 270, 90, 0, 35, 4.0, 20.0, 0.0, 1.0,  None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // Air only
        ATTACK(fighter, 5, 0, Hash40::new("swordl1"), 15.0, 270, 38, 0, 35, 4.0, 20.0, 0.0, 1.0,  None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("swordl1"), 10.0, 48, 76, 0, 58, 3.0, 5.0, 0.0, 1.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("swordl1"), 10.0, 48, 76, 0, 58, 3.0, 9.5, 0.0, 1.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("swordl1"), 10.0, 48, 76, 0, 58, 3.0, 15.0, 0.0, 1.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // Ground only
        ATTACK(fighter, 4, 0, Hash40::new("swordl1"), 10.0, 48, 76, 0, 58, 3.0, 20.0, 0.0, 1.0,  None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        // Air only
        ATTACK(fighter, 5, 0, Hash40::new("swordl1"), 10.0, 48, 76, 0, 58, 3.0, 20.0, 0.0, 1.0,  None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 38.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
        KineticModule::resume_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    frame(lua_state, 53.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

#[acmd_script( agent = "edge", script = "effect_attackairlw" , category = ACMD_EFFECT , low_priority)]
unsafe fn sephiroth_attack_air_lw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("swordl2"), 13, 0, -3.5, 0, 0, 0, 0.8, 1, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("edge_sword_light3"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 0.4);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        EFFECT_FLW_POS(fighter, Hash40::new("edge_attack_dash"), Hash40::new("swordl2"), 19.0, 0, 0.8, 0, 0, 0, 0.9, true);
        LAST_EFFECT_SET_RATE(fighter, 1.1);
        EFFECT_FLW_POS(fighter, Hash40::new("edge_attack_dash_aura"), Hash40::new("swordl2"), 19.0, 0, 0.8, 0, 0, 0, 0.9, true);
        LAST_EFFECT_SET_RATE(fighter, 1.1);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("edge_gokumon_wind"), Hash40::new("top"), -2.2, 30, 1.7, 0, 0, 0, 1, true);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("edge_sword_light3"), true, true);
        EFFECT_FOLLOW(fighter, Hash40::new("edge_gokumon_speedline"), Hash40::new("top"), 0, 0, 0, 0, 46, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("edge_sword_light3"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
        EFFECT_FLW_POS(fighter, Hash40::new("edge_sword_flash2"), Hash40::new("swordl2"), 22, 0, 0.8, 0, 0, 0, 1.2, true);

        EFFECT_FLW_POS(fighter, Hash40::new("edge_attack_dash2"), Hash40::new("swordl2"), 19.0, 0, 0.8, 0, 0, 0, 0.65, true);
        LAST_EFFECT_SET_RATE(fighter, 1.0);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("edge_sword_flash2"), -1);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("edge_attack_dash"), -1);
        EFFECT_DETACH_KIND(fighter, Hash40::new("edge_attack_dash_aura"), -1);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("edge_attack_dash"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("edge_attack_dash_aura"), false, false);
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        sephiroth_attack_air_n_game,
        sephiroth_attack_air_n_effect,
        sephiroth_attack_air_f_game,
        sephiroth_attack_air_f_effect,
        sephiroth_attack_air_b_game,
        sephiroth_attack_air_b_effect,
        sephiroth_attack_air_hi_game,
        sephiroth_attack_air_hi_effect,
        sephiroth_attack_air_lw_game,
        sephiroth_attack_air_lw_effect,
    );
}

