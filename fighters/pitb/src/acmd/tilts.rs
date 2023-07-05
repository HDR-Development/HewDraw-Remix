
use super::*;


#[acmd_script( agent = "pitb", script = "game_attacks3" , category = ACMD_GAME , low_priority)]
unsafe fn pitb_attack_s3_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.778);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 6.5, 361, 100, 0, 40, 4.0, 0.0, 7.5, 13.0, Some(0.0), Some(7.5), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 361, 100, 0, 40, 4.0, 0.0, 7.5, 20.0, Some(0.0), Some(7.5), Some(10.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "pitb", script = "effect_attacks3", category = ACMD_EFFECT, low_priority)]
unsafe fn pitb_attack_s3_s_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pitb_sword"), Hash40::new("swordr2"), 0, 0, 0, 0, 90, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("pitb_sword"), Hash40::new("swordl"), 0, 0, 0, 180, 90, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("pitb_atk_s3"), Hash40::new("top"), 0, -2, 0, 0, 0, 0, 1.2, true);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pitb_sword"), true, true);
    }
    
}

#[acmd_script( agent = "pitb", script = "game_attacklw3" , category = ACMD_GAME , low_priority)]
unsafe fn pitb_attack_lw3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 6.5, 69, 56, 0, 60, 3.5, 0.0, 2.5, 19.5, Some(0.0), Some(5.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        FT_MOTION_RATE(fighter, 0.720);
    }
    
}

#[acmd_script( agent = "pitb", script = "effect_attacklw3", category = ACMD_EFFECT, low_priority)]
unsafe fn pitb_attack_lw3_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pitb_atk_wind"), Hash40::new("top"), -7, 6, 2.3, -12, -42, 168, 1.3, true);
        EFFECT_FOLLOW(fighter, Hash40::new("pitb_sword"), Hash40::new("swordr2"), 0, 0, 0, 0, 90, 0, 1, true);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pitb_sword"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pitb_atk_wind"), true, true);
    }

}

#[acmd_script( agent = "pitb", script = "game_attackhi3" , category = ACMD_GAME , low_priority)]
unsafe fn pitb_attack_hi3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 365, 100, 85, 0, 4.0, 0.0, 24.0, 2.4, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        let hitVec = smash::phx::Vector2f { x: 4.5, y: 21.5 };
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &hitVec, 10, false);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 69, 50, 0, 70, 6.0, 0.0, 23.5, 6.0, Some(0.0), Some(20.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        FT_MOTION_RATE(fighter, 0.850);
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        pitb_attack_s3_s_game,
        pitb_attack_s3_s_effect,
        pitb_attack_lw3_game,
        pitb_attack_lw3_effect,
        pitb_attack_hi3_game,
    );
}

