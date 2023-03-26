
use super::*;

#[acmd_script( agent = "samus", script = "game_attacks3hi" , category = ACMD_GAME , low_priority)]
unsafe fn attack_s3_hi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.67);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("handr"), *HIT_STATUS_XLU);
	}
	frame(lua_state, 7.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
		HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("footr"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("handr"), *HIT_STATUS_NORMAL);
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 10.0, 361, 100, 0, 10, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 10.0, 361, 100, 0, 10, 3.5, 0.6, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("kneer"), 11.0, 361, 120, 0, 20, 3.5, 6.0, -0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 3, 0, Hash40::new("kneel"), 10.0, 361, 100, 0, 10, 3.5, 0.0, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    
}

#[acmd_script( agent = "samus", script = "effect_attacks3hi", category = ACMD_EFFECT, low_priority)]
unsafe fn effect_attacks3hi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("top"), -3, 13.5, 14.5, 32, 6, -192, 0.9, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
}

#[acmd_script( agent = "samus", script = "game_attacks3" , category = ACMD_GAME , low_priority)]
unsafe fn attack_s3_s(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.67);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("handr"), *HIT_STATUS_XLU);
	}
	frame(lua_state, 7.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("footr"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("handr"), *HIT_STATUS_NORMAL);
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 10.0, 361, 100, 0, 10, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 10.0, 361, 100, 0, 10, 3.5, 0.6, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("kneer"), 11.0, 361, 120, 0, 20, 3.5, 6.0, -0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 3, 0, Hash40::new("kneel"), 10.0, 361, 100, 0, 10, 3.5, 0.0, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    
}

#[acmd_script( agent = "samus", script = "effect_attacks3", category = ACMD_EFFECT, low_priority)]
unsafe fn effect_attacks3s(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc"), Hash40::new("top"), -1.5, 11.5, 14, 2, 5, 165, 0.95, true);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
}

#[acmd_script( agent = "samus", script = "game_attacks3lw" , category = ACMD_GAME , low_priority)]
unsafe fn attack_s3_lw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.67);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("handr"), *HIT_STATUS_XLU);
	}
	frame(lua_state, 7.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("footr"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("handr"), *HIT_STATUS_NORMAL);
        ATTACK(fighter, 0, 0, Hash40::new("hip"), 10.0, 361, 100, 0, 10, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 10.0, 361, 100, 0, 10, 3.5, 0.6, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("kneer"), 11.0, 361, 120, 0, 20, 3.5, 6.0, -0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 3, 0, Hash40::new("kneel"), 10.0, 361, 100, 0, 10, 3.5, 0.0, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    
}

#[acmd_script( agent = "samus", script = "effect_attacks3lw", category = ACMD_EFFECT, low_priority)]
unsafe fn effect_attacks3lw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc"), Hash40::new("top"), -2, 6, 11, -15, 0, 195, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
}

#[acmd_script( agent = "samus", script = "game_attackhi3" , category = ACMD_GAME , low_priority)]
unsafe fn attack_hi3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("handr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 13.0, 270, 100, 0, 40, 5.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 13.0, 270, 100, 0, 40, 5.8, 6.0, -0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("kneer"), 13.0, 361, 107, 0, 40, 5.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 3, 0, Hash40::new("kneer"), 13.0, 361, 107, 0, 40, 5.8, 6.0, -0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "samus", script = "effect_attackhi3", category = ACMD_EFFECT, low_priority)]
unsafe fn effect_attackhi3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 1, 11, 5, 1.7, -39, -92, 1.65, true);
        LAST_EFFECT_SET_RATE(fighter, 1);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 20, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "samus", script = "game_attacklw3" , category = ACMD_GAME , low_priority)]
unsafe fn attack_lw3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if !VarModule::is_flag(fighter.battle_object, vars::samus::instance::STANCE_ICE) { 
        frame(lua_state, 6.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 100, 77, 0, 80, 8.7, 0.0, 2.5, 13.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
        wait(lua_state, 3.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
    } 
    else {
        frame(lua_state, 6.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 5, 100, 63, 0, 5.7, 0.0, 2.5, 13.5, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_MAGIC);
            AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
        wait(lua_state, 3.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
            let pos = &mut Vector3f{x: 0.0, y: 0.0, z: 0.0};
            ModelModule::joint_global_position_with_offset(fighter.module_accessor, Hash40::new("top"), &Vector3f{x: 0.0, y: 2.5, z: 18.5}, pos, true);
            if GroundModule::ray_check(fighter.module_accessor, &Vector2f{x: pos.x, y: pos.y}, &Vector2f{x: 0.0, y: -2.5}, true) == 1 {
                ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 5, 100, 40, 0, 5.3, 0.0, 2.5, 20.5, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_MAGIC);
                AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
            }
            else {
                VarModule::set_flag(fighter.battle_object, vars::samus::status::ATTACK_LW3_ADVANCE_STOP, true);
                println!("No ground detected!");
            }
        }
        wait(lua_state, 3.0);
        if is_excute(fighter) {
            if !VarModule::is_flag(fighter.battle_object, vars::samus::status::ATTACK_LW3_ADVANCE_STOP) {
                AttackModule::clear_all(boma);
                let pos = &mut Vector3f{x: 0.0, y: 0.0, z: 0.0};
                ModelModule::joint_global_position_with_offset(fighter.module_accessor, Hash40::new("top"), &Vector3f{x: 0.0, y: 2.5, z: 23.5}, pos, true);
                if GroundModule::ray_check(fighter.module_accessor, &Vector2f{x: pos.x, y: pos.y}, &Vector2f{x: 0.0, y: -2.5}, true) == 1 {
                    ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 5, 100, 40, 0, 4.7, 0.0, 2.5, 27.5, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_MAGIC);
                    AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
                }
                else {
                    VarModule::set_flag(fighter.battle_object, vars::samus::status::ATTACK_LW3_ADVANCE_STOP, true);
                    println!("No ground detected!");
                }
            }
        }
        wait(lua_state, 3.0);
        if is_excute(fighter) {
            if !VarModule::is_flag(fighter.battle_object, vars::samus::status::ATTACK_LW3_ADVANCE_STOP) {
                AttackModule::clear_all(boma);
                let pos = &mut Vector3f{x: 0.0, y: 0.0, z: 0.0};
                ModelModule::joint_global_position_with_offset(fighter.module_accessor, Hash40::new("top"), &Vector3f{x: 0.0, y: 2.5, z: 28.5}, pos, true);
                if GroundModule::ray_check(fighter.module_accessor, &Vector2f{x: pos.x, y: pos.y}, &Vector2f{x: 0.0, y: -2.5}, true) == 1 {
                    ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 5, 100, 63, 0, 4.3, 0.0, 2.5, 35.5, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_MAGIC);
                    AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
                }
            }
        }
        wait(lua_state, 3.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
    }
    
    // let ground_hit_pos = &mut Vector2f{x: 0.0, y: 0.0};
    //     if GroundModule::ray_check_hit_pos(
    //         // checks for ground between previous tail pos and current tail pos, saves collision pos to "ground_hit_pos"
    //         fighter.module_accessor,
    //         &Vector2f{x: pos_x_prev + pos_x_global, y: pos_y_prev + pos_y_global},
    //         &Vector2f{x: (v3f_tail_pos.x - (pos_x_prev + pos_x_global)) + (8.0 * lr), y: v3f_tail_pos.y - (pos_y_prev + pos_y_global) - 8.0},
    //         ground_hit_pos,
    //         true
    //     ) == 1
}

#[acmd_script( agent = "samus", script = "effect_attacklw3", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_attacklw3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if !VarModule::is_flag(fighter.battle_object, vars::samus::instance::STANCE_ICE) {
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(lua_state, 6.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("samus_atk_bomb"), Hash40::new("armr"), 14.387, -0.341, -0.169, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            EFFECT(fighter, Hash40::new("sys_bomb_a"),  Hash40::new("top"), 0, 0, 13.4, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
            EFFECT(fighter, Hash40::new("sys_bomb_b"),  Hash40::new("top"), 0, 0, 13.4, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        }
    } else {
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_ice"), Hash40::new("armr"), 2.5, 0.0, 0.0, 0.0, 0.0, 20.0, 0.15, true);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_ice"), Hash40::new("armr"), 5.0, 0.0, 0.0, 0.0, 0.0, 20.0, 0.15, true);
        }
        frame(lua_state, 6.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_ice"), false, true);
            EFFECT(fighter, Hash40::new("sys_freezer"), Hash40::new("top"), 0, 0, 13.4, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
            EFFECT(fighter, Hash40::new("sys_steam"), Hash40::new("top"), 0, 0, 13.4, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_COLOR(fighter, 1.0, 1.2, 1.9);
            LAST_EFFECT_SET_ALPHA(fighter, 1.2);
        }
        wait(lua_state, 3.0);
        if is_excute(fighter) {
            if !VarModule::is_flag(fighter.battle_object, vars::samus::status::ATTACK_LW3_ADVANCE_STOP) {
                EFFECT_OFF_KIND(fighter, Hash40::new("sys_ice"), false, true);
                EFFECT(fighter, Hash40::new("sys_freezer"), Hash40::new("top"), 0, 0, 20.4, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
                EFFECT(fighter, Hash40::new("sys_steam"), Hash40::new("top"), 0, 0, 20.4, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, true);
                LAST_EFFECT_SET_COLOR(fighter, 1.0, 1.2, 1.9);
                LAST_EFFECT_SET_ALPHA(fighter, 1.2);
            }
        }
        wait(lua_state, 3.0);
        if is_excute(fighter) {
            if !VarModule::is_flag(fighter.battle_object, vars::samus::status::ATTACK_LW3_ADVANCE_STOP) {
                EFFECT_OFF_KIND(fighter, Hash40::new("sys_ice"), false, true);
                EFFECT(fighter, Hash40::new("sys_freezer"), Hash40::new("top"), 0, 0, 27.4, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
                EFFECT(fighter, Hash40::new("sys_steam"), Hash40::new("top"), 0, 0, 27.4, 0, 0, 0, 1.15, 0, 0, 0, 0, 0, 0, true);
                LAST_EFFECT_SET_COLOR(fighter, 1.0, 1.2, 1.9);
                LAST_EFFECT_SET_ALPHA(fighter, 1.2);
            }
        }
        wait(lua_state, 3.0);
        if is_excute(fighter) {
            if !VarModule::is_flag(fighter.battle_object, vars::samus::status::ATTACK_LW3_ADVANCE_STOP) {
                EFFECT_OFF_KIND(fighter, Hash40::new("sys_ice"), false, true);
                EFFECT(fighter, Hash40::new("sys_freezer"), Hash40::new("top"), 0, 0, 35.4, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true);
                EFFECT(fighter, Hash40::new("sys_steam"), Hash40::new("top"), 0, 0, 35.4, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
                LAST_EFFECT_SET_COLOR(fighter, 1.0, 1.2, 1.9);
                LAST_EFFECT_SET_ALPHA(fighter, 1.2);
            }
        }
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        attack_s3_hi,
        effect_attacks3hi,
        attack_s3_s,
        effect_attacks3s,
        attack_s3_lw,
        effect_attacks3lw,
        attack_hi3,
        effect_attackhi3,
        attack_lw3,
        effect_attacklw3,
    );
}

