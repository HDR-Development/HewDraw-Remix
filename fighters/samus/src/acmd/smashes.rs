
use super::*;

#[acmd_script( agent = "samus", script = "game_attacks4hi" , category = ACMD_GAME , low_priority)]
unsafe fn attack_s4_hi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if !VarModule::is_flag(fighter.battle_object, vars::samus::instance::STANCE_ICE) {
        frame(lua_state, 5.0);
        if is_excute(fighter) {
            MotionModule::set_rate(boma, (10.0-5.0)/10.0);
        }
        frame(lua_state, 10.0);
        if is_excute(fighter) {
            MotionModule::set_rate(boma, 1.0);
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 13.5, 361, 102, 0, 30, 3.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("armr"), 13.5, 361, 102, 0, 30, 3.3, -4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("armr"), 15.5, 361, 106, 0, 40, 5.0, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 10.0, 361, 102, 0, 30, 3.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("armr"), 10.0, 361, 102, 0, 30, 3.3, -4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("armr"), 12.0, 361, 106, 0, 40, 5.0, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        }
        wait(lua_state, 6.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
    }
    else {
        frame(lua_state, 10.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 10.5, 361, 90, 0, 28, 3.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("armr"), 10.5, 361, 90, 0, 28, 3.3, -4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("armr"), 13.5, 361, 96, 0, 38, 5.0, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_MAGIC);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 8.0, 361, 90, 0, 28, 3.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("armr"), 8.0, 361, 90, 0, 28, 3.3, -4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("armr"), 10.0, 361, 96, 0, 38, 5.0, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_MAGIC);
        }
        wait(lua_state, 6.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
    }
}

#[acmd_script( agent = "samus", script = "effect_attacks4hi" , category = ACMD_EFFECT , low_priority)]
unsafe fn effect_attacks4hi (fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let boma = fighter.boma();
    if !VarModule::is_flag(fighter.battle_object, vars::samus::instance::STANCE_ICE) {
        frame(lua_state, 3.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("armr"), 4.289, -0.272, -0.135, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(lua_state, 10.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 10, 3, -32, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
            LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            EFFECT(fighter, Hash40::new("samus_atk_bomb"), Hash40::new("armr"), 14.387, -0.341, -0.169, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        }
    }
    else {
        frame(lua_state, 3.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("armr"), 4.289, -0.272, -0.135, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_ice"), Hash40::new("armr"), 2.5, 0.0, 0.0, 0.0, 0.0, 20.0, 0.15, true);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_ice"), Hash40::new("armr"), 5.0, 0.0, 0.0, 0.0, 0.0, 20.0, 0.15, true);
        }
        frame(lua_state, 5.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_ice"), Hash40::new("armr"), 2.5, 0.0, 0.0, 0.0, 0.0, 20.0, 0.15, true);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_ice"), Hash40::new("armr"), 5.0, 0.0, 0.0, 0.0, 0.0, 20.0, 0.15, true);
        }
        frame(lua_state, 10.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 10, 3, -32, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
            LANDING_EFFECT(fighter, Hash40::new("sys_steam"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_COLOR(fighter, 1.0, 1.2, 1.9);
            LAST_EFFECT_SET_ALPHA(fighter, 1.2);
            EFFECT(fighter, Hash40::new("sys_freezer"), Hash40::new("armr"), 10.387, -0.341, -0.169, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
            EFFECT(fighter, Hash40::new("sys_hit_ice"), Hash40::new("armr"), 10.387, -0.341, -0.169, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true);
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_ice"), false, true);
        }
    }
}

#[acmd_script( agent = "samus", script = "game_attacks4" , category = ACMD_GAME , low_priority)]
unsafe fn attack_s4_s(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if !VarModule::is_flag(fighter.battle_object, vars::samus::instance::STANCE_ICE) {
        frame(lua_state, 4.0);
        if is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
        frame(lua_state, 5.0);
        if is_excute(fighter) {
            MotionModule::set_rate(boma, (10.0-5.0)/10.0);
        }
        frame(lua_state, 10.0);
        if is_excute(fighter) {
            MotionModule::set_rate(boma, 1.0);
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 13.0, 361, 102, 0, 30, 3.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("armr"), 13.0, 361, 102, 0, 30, 3.3, -4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("armr"), 15.0, 361, 106, 0, 40, 5.0, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 10.0, 361, 102, 0, 30, 3.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("armr"), 10.0, 361, 102, 0, 30, 3.3, -4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("armr"), 12.0, 361, 106, 0, 40, 5.0, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        }
        wait(lua_state, 6.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
    } 
    else {
        frame(lua_state, 4.0);
        if is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
        frame(lua_state, 10.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 10.0, 361, 90, 0, 28, 3.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("armr"), 10.0, 361, 90, 0, 28, 3.3, -4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("armr"), 13.0, 361, 96, 0, 38, 5.0, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_MAGIC);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 8.0, 361, 90, 0, 28, 3.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("armr"), 8.0, 361, 90, 0, 28, 3.3, -4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("armr"), 10.0, 361, 96, 0, 38, 5.0, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_MAGIC);
        }
        wait(lua_state, 6.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
    }
}

#[acmd_script( agent = "samus", script = "effect_attacks4" , category = ACMD_EFFECT , low_priority)]
unsafe fn effect_attacks4 (fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let boma = fighter.boma();
    if !VarModule::is_flag(fighter.battle_object, vars::samus::instance::STANCE_ICE) {
        frame(lua_state, 3.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("armr"), 4.289, -0.272, -0.135, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(lua_state, 10.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 11.5, 3, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
            LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            EFFECT(fighter, Hash40::new("samus_atk_bomb"), Hash40::new("armr"), 14.387, -0.341, -0.169, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        }
    }
	else {
        frame(lua_state, 3.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("armr"), 4.289, -0.272, -0.135, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_ice"), Hash40::new("armr"), 2.5, 0.0, 0.0, 0.0, 0.0, 20.0, 0.15, true);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_ice"), Hash40::new("armr"), 5.0, 0.0, 0.0, 0.0, 0.0, 20.0, 0.15, true);
        }
        frame(lua_state, 5.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_ice"), Hash40::new("armr"), 2.5, 0.0, 0.0, 0.0, 0.0, 20.0, 0.15, true);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_ice"), Hash40::new("armr"), 5.0, 0.0, 0.0, 0.0, 0.0, 20.0, 0.15, true);
        }
        frame(lua_state, 10.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 11.5, 3, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_ice"), false, true);
            EFFECT(fighter, Hash40::new("sys_freezer"), Hash40::new("armr"), 10.387, -0.341, -0.169, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
            EFFECT(fighter, Hash40::new("sys_hit_ice"), Hash40::new("armr"), 10.387, -0.341, -0.169, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true);
            EFFECT(fighter, Hash40::new("sys_steam"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_COLOR(fighter, 1.0, 1.2, 1.9);
            LAST_EFFECT_SET_ALPHA(fighter, 1.2);
        }
    }
}

#[acmd_script( agent = "samus", script = "game_attacks4lw" , category = ACMD_GAME , low_priority)]
unsafe fn attack_s4_lw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if !VarModule::is_flag(fighter.battle_object, vars::samus::instance::STANCE_ICE) { 
        frame(lua_state, 5.0);
        if is_excute(fighter) {
            MotionModule::set_rate(boma, (10.0-5.0)/10.0);
        }
        frame(lua_state, 10.0);
        if is_excute(fighter) {
            MotionModule::set_rate(boma, 1.0);
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 12.5, 361, 102, 0, 30, 3.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("armr"), 12.5, 361, 102, 0, 30, 3.3, -4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("armr"), 14.5, 361, 106, 0, 40, 5.0, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 10.0, 361, 102, 0, 30, 3.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("armr"), 10.0, 361, 102, 0, 30, 3.3, -4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("armr"), 12.0, 361, 106, 0, 40, 5.0, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        }
        wait(lua_state, 6.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
    }
    else {
        frame(lua_state, 10.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 9.5, 361, 90, 0, 28, 3.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("armr"), 9.5, 361, 90, 0, 28, 3.3, -4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("armr"), 11.5, 361, 96, 0, 38, 5.0, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_MAGIC);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 8.0, 361, 90, 0, 28, 3.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("armr"), 8.0, 361, 90, 0, 28, 3.3, -4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("armr"), 10.0, 361, 96, 0, 38, 5.0, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_MAGIC);
        }
        wait(lua_state, 6.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
    }
}

#[acmd_script( agent = "samus", script = "effect_attacks4lw" , category = ACMD_EFFECT , low_priority)]
unsafe fn effect_attacks4lw (fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let boma = fighter.boma();
    if !VarModule::is_flag(fighter.battle_object, vars::samus::instance::STANCE_ICE) { 
        frame(lua_state, 3.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("armr"), 4.289, -0.272, -0.135, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(lua_state, 10.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 11.5, 3, 28, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
            LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            EFFECT(fighter, Hash40::new("samus_atk_bomb"), Hash40::new("armr"), 14.387, -0.341, -0.169, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        }
    } else {
        frame(lua_state, 3.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("armr"), 4.289, -0.272, -0.135, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_ice"), Hash40::new("armr"), 2.5, 0.0, 0.0, 0.0, 0.0, 20.0, 0.15, true);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_ice"), Hash40::new("armr"), 5.0, 0.0, 0.0, 0.0, 0.0, 20.0, 0.15, true);
        }
        frame(lua_state, 5.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_ice"), Hash40::new("armr"), 2.5, 0.0, 0.0, 0.0, 0.0, 20.0, 0.15, true);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_ice"), Hash40::new("armr"), 5.0, 0.0, 0.0, 0.0, 0.0, 20.0, 0.15, true);
        }
        frame(lua_state, 10.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_ice"), false, true);
            EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 11.5, 3, 28, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
            LANDING_EFFECT(fighter, Hash40::new("sys_steam"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_COLOR(fighter, 1.0, 1.2, 1.9);
            LAST_EFFECT_SET_ALPHA(fighter, 1.2);
            EFFECT(fighter, Hash40::new("sys_freezer"), Hash40::new("armr"), 10.387, -0.341, -0.169, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
            EFFECT(fighter, Hash40::new("sys_hit_ice"), Hash40::new("armr"), 10.387, -0.341, -0.169, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true);
        }
    }
	
}

#[acmd_script( agent = "samus", script = "game_attackhi4" , category = ACMD_GAME , low_priority)]
unsafe fn attack_hi4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if !VarModule::is_flag(fighter.battle_object, vars::samus::instance::STANCE_ICE) {
        frame(lua_state, 7.0);
        if is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
        frame(lua_state, 13.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 368, 70, 100, 0, 4.0, 0.0, 3.5, 8.0, None, None, None, 1.0, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 368, 70, 100, 0, 4.0, 0.0, 3.5, -5.5, None, None, None, 1.0, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            AttackModule::set_no_finish_camera(boma, 0, true, false);
            AttackModule::set_no_finish_camera(boma, 1, true, false);
            let hit1 = Vector2f { x: 0.0, y: 19.0 };
            AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &hit1, 10, false);
            AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &hit1, 10, false);
        }
        wait(lua_state, 2.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        frame(lua_state, 15.0);
        if is_excute(fighter) {
            FT_MOTION_RATE(fighter, 8.0/(25.0-15.0));
        }
        frame(lua_state, 25.0);
        if is_excute(fighter) {
            FT_MOTION_RATE(fighter, 1.0);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 90, 115, 0, 50, 8.0, 0.0, 28.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        }
        wait(lua_state, 3.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        frame(lua_state, 63.0);
        if is_excute(fighter) {
            StatusModule::change_status_request(boma, *FIGHTER_STATUS_KIND_WAIT, false);
        }
    } 
    else {
        if !fighter.is_motion(Hash40::new("attack_hi4_ice")) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_hi4_ice"), fighter.motion_frame(), 1.0, false, 0.0, false, false);
        }
        frame(lua_state, 7.0);
        if is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
        frame(lua_state, 11.0);
        if is_excute(fighter) {
            FT_MOTION_RATE(fighter, 1.5);
        }
        frame(lua_state, 11.5);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 17.0, 80, 85, 0, 45, 3.7, -2.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("armr"), 17.0, 80, 85, 0, 45, 5.7,  5.9, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_MAGIC);
        }
        frame(lua_state, 12.0);
        if is_excute(fighter) {
            FT_MOTION_RATE(fighter, 1.0);
        }
        frame(lua_state, 14.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 14.0, 80, 100, 0, 40, 3.7, -2.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("armr"), 14.0, 80, 100, 0, 40, 5.7, 5.9, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_MAGIC);
        }
        frame(lua_state, 16.0);
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("armr"), 12.0, 85, 90, 0, 20, 3.7, -2.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("armr"), 12.0, 85, 90, 0, 20, 5.7, 5.9, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_MAGIC);
        }
        wait(lua_state, 3.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        frame(lua_state, 63.0);
        if is_excute(fighter) {
            StatusModule::change_status_request(boma, *FIGHTER_STATUS_KIND_WAIT, false);
        }
    }
}

#[acmd_script( agent = "samus", script = "effect_attackhi4", category = ACMD_EFFECT , low_priority)]
pub unsafe fn attack_hi4_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if !VarModule::is_flag(fighter.battle_object, vars::samus::instance::STANCE_ICE) {
        frame(lua_state, 14.0);
        if is_excute(fighter) {
            LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 3, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
            EFFECT(fighter, Hash40::new_raw(0x1441eaf0b3u64), Hash40::new("top"), 0, 8, 0, -90, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_ALPHA(fighter, Hash40::new_raw(0x1156ac182a), Hash40::new("armr"), 7.5, -0.341439992, -0.169369996, 0, 0, 0, 1.25, true, 0.9);
            LAST_EFFECT_SET_RATE(fighter, 2);
        }
        frame(lua_state, 25.0);
        if is_excute(fighter){
            LANDING_EFFECT(fighter, Hash40::new_raw(0x12afcb1820u64), Hash40::new("top"), 0, 0, 3, 0, 0, 0, 1.20000005, 0, 0, 0, 0, 0, 0, true);
            EFFECT(fighter, Hash40::new_raw(0x0e60909a7cu64), Hash40::new("top"), 0.0, 28.0, 1.0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
        }
    }
    else {
        frame(lua_state, 3.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_ice"), Hash40::new("armr"), 2.5, 0.0, 0.0, 0.0, 0.0, 20.0, 0.15, true);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_ice"), Hash40::new("armr"), 5.0, 0.0, 0.0, 0.0, 0.0, 20.0, 0.15, true);
        }
        frame(lua_state, 10.0);
        if is_excute(fighter) {
            AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_samus_trail1"), Hash40::new("tex_samus_trail2"), 14, Hash40::new("armr"), 6.0, -1.0, 0.0, Hash40::new("armr"), 13.0, -1.0, 0.0, true, Hash40::new("null"), Hash40::new("null"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2000000476837158, 0.20000000298023224);
            LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.0, 0.0);
        }
        frame(lua_state, 11.0);
        if is_excute(fighter) {
            LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 3, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
        for _ in 0..8 {
            wait(lua_state, 1.0);
            if is_excute(fighter) {
                if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) && !VarModule::is_flag(fighter.battle_object, vars::samus::status::ATTACK_HIT_STOP_EFFECT) {
                    VarModule::on_flag(fighter.battle_object, vars::samus::status::ATTACK_HIT_STOP_EFFECT);
                    EFFECT_OFF_KIND(fighter, Hash40::new("sys_ice"), false, true);
                    EFFECT(fighter, Hash40::new("sys_hit_ice"), Hash40::new("armr"), 5.0, 0.0, 0.0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
                }
            }
        }
        frame(lua_state, 21.0);
        if is_excute(fighter) {
            AFTER_IMAGE_OFF(fighter, 3);
        }
        frame(lua_state, 34.0);
        if is_excute(fighter) {
            if !VarModule::is_flag(fighter.battle_object, vars::samus::status::ATTACK_HIT_STOP_EFFECT) {
                EFFECT_OFF_KIND(fighter, Hash40::new("sys_ice"), false, true);
                EFFECT(fighter, Hash40::new("sys_freezer"), Hash40::new("armr"), 5.0, 0.0, 0.0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, true);
                EFFECT(fighter, Hash40::new("sys_steam"), Hash40::new("armr"), 5.0, 0.0, 0.0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
                LAST_EFFECT_SET_COLOR(fighter, 1.0, 1.2, 1.9);
                LAST_EFFECT_SET_ALPHA(fighter, 1.2);
            }
        }
        frame(lua_state, 40.0);
        if is_excute(fighter) {
            LANDING_EFFECT(fighter, Hash40::new_raw(0x12afcb1820u64), Hash40::new("top"), 0, 0, 3, 0, 0, 0, 1.20000005, 0, 0, 0, 0, 0, 0, true);
        }
    }
}

#[acmd_script( agent = "samus", script = "sound_attackhi4", category = ACMD_SOUND , low_priority)]
pub unsafe fn attack_hi4_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

    frame(lua_state, 13.0);
    if is_excute(fighter){
        STOP_SE(fighter, Hash40::new_raw(0x156bea5f43u64));
        PLAY_SE(fighter, Hash40::new("se_samus_swing_l"));
    }
    frame(lua_state, 28.0);
    if is_excute(fighter){
        PLAY_SE(fighter, Hash40::new_raw(0x1210d94515u64));
    }
}

#[acmd_script( agent = "samus", script = "game_attackhi4charge", category = ACMD_GAME , low_priority)]
pub unsafe fn game_attackhi4charge(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if VarModule::is_flag(fighter.battle_object, vars::samus::instance::STANCE_ICE) {
        if !fighter.is_motion(Hash40::new("attack_hi4_hold_ice")) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_hi4_hold_ice"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
}

#[acmd_script( agent = "samus", script = "game_attacklw4" , category = ACMD_GAME , low_priority)]
unsafe fn attack_lw4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 16.0, 120, 87, 0, 50, 4.2, 8.0, -0.5, 0.0, Some(-2.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 15.0, 120, 87, 0, 50, 4.6, 8.0, -0.5, 0.0, Some(-2.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        attack_s4_hi,
        attack_s4_s,
        attack_s4_lw,
        effect_attacks4hi,
        effect_attacks4,
        effect_attacks4lw,
        attack_hi4,
        attack_hi4_effect,
        attack_hi4_sound,
        game_attackhi4charge,
        attack_lw4,
    );
}

