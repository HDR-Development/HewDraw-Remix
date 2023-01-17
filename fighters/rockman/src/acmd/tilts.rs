
use super::*;


#[acmd_script( agent = "rockman", script = "game_attacklw3" , category = ACMD_GAME , low_priority)]
unsafe fn rockman_attack_lw3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        JostleModule::set_status(boma, false);
        HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        // MotionModule::set_rate(boma, 1.25);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.5, 69, 45, 0, 76, 4.0, 0.0, 3.2, 3.5, Some(0.0), Some(3.2), Some(4.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 7.5, 65, 55, 0, 75, 4.0, 0.0, 3.2, 3.5, Some(0.0), Some(3.2), Some(4.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        // AttackModule::set_add_reaction_frame(boma, 0, 5.0, false);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        // MotionModule::set_rate(boma, 1.00);
        JostleModule::set_status(boma, true);
        AttackModule::clear_all(boma);
    }

}

#[acmd_script( agent = "rockman", script = "game_attackhi3" , category = ACMD_GAME , low_priority)]
unsafe fn rockman_attack_hi3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
            FT_MOTION_RATE(fighter, 3.0 / (6.0 - 5.0));
        }
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
            FT_MOTION_RATE(fighter, 16.0 / (20.0 - 6.0));
            ATTACK(fighter, 0, 0, Hash40::new("top"), 17.0, 85, 105, 0, 40, 3.5, 0.0, 6.0, 8.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        } else {
            FT_MOTION_RATE(fighter, 11.0 / (20.0 - 6.0));
            ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 85, 54, 0, 80, 3.5, 0.0, 6.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("armr"), 10.0, 85, 54, 0, 80, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("armr"), 10.0, 85, 54, 0, 80, 5.0, 4.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);    
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("armr"), 7.0, 70, 41, 0, 70, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("armr"), 7.0, 70, 41, 0, 70, 5.0, 4.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
            FT_MOTION_RATE(fighter, 18.0 / (32.0 - 20.0));
        } else {
            FT_MOTION_RATE(fighter, 1.0);
        }
    }
    frame(lua_state, 32.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
}

#[acmd_script( agent = "rockman", script = "effect_attackhi3" , category = ACMD_EFFECT , low_priority)]
unsafe fn rockman_attack_hi3_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 6, 8, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        }
        LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("rockman_rockupper_power"), Hash40::new("armr"), 3.5, 0, 0, 0, 0, 0, 1, true);
        if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
			EFFECT_FOLLOW(fighter, Hash40::new("rockman_rockupper_arc"), Hash40::new("trans"), 4, 3, -2, 5, 0, 10, 1, true);
        }
        else {
			EFFECT_FOLLOW(fighter, Hash40::new("rockman_rockupper_arc"), Hash40::new("trans"), -4, 3, -2, 5, 0, -10, 1, true);
        }
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("rockman_rockupper_arc"), -1);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("rockman_rockupper_arc"), true, true);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("rockman_rockupper_power"), false, true);
    }
}

pub fn install() {
    install_acmd_scripts!(
        rockman_attack_lw3_game,
        rockman_attack_hi3_game,
        rockman_attack_hi3_effect
    );
}
