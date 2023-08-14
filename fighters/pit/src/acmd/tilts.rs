use super::*;

#[acmd_script( agent = "pit", script = "game_attacks3" , category = ACMD_GAME , low_priority)]
unsafe fn pit_attack_s3_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 361, 86, 0, 42, 4.0, 0.0, 7.5, 8.0, Some(0.0), Some(7.5), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 361, 86, 0, 42, 4.0, 0.0, 7.5, 15.0, Some(0.0), Some(7.5), Some(6.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "pit", script = "game_attackhi3" , category = ACMD_GAME , low_priority)]
unsafe fn pit_attack_hi3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 365, 100, 85, 0, 4.0, 0.0, 24.0, 2.3, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        let hitVec = smash::phx::Vector2f { x: 3.8, y: 23.5 };
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &hitVec, 10, false);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 72, 48, 0, 71, 6.0, 0.0, 23.5, 6.0, Some(0.0), Some(20.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 17.0);
    FT_MOTION_RATE_RANGE(fighter, 17.0, 34.0, 11.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 34.0);
    FT_MOTION_RATE(fighter, 1.0);
    
}

#[acmd_script( agent = "pit", script = "game_attacklw3" , category = ACMD_GAME , low_priority)]
unsafe fn pit_attack_lw3_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 75, 40, 0, 71, 3.5, 0.0, 3.0, 19.5, Some(0.0), Some(5.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(lua_state, 8.0);
    FT_MOTION_RATE_RANGE(fighter, 8.0, 30.0, 17.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 30.0);
    FT_MOTION_RATE(fighter, 1.0);
    
}

#[acmd_script( agent = "pit", script = "effect_attacklw3", category = ACMD_EFFECT, low_priority )]
unsafe fn pit_attack_lw3_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pit_atk_wind"), Hash40::new("top"), -7, 6, 7, -12, -42, 168, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("pit_sword"), Hash40::new("swordr2"), 0, 0, 0, 0, 90, 0, 1, true);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pit_sword"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("pit_atk_wind"), true, true);
    }
}

pub fn install() {
    install_acmd_scripts!(
        pit_attack_s3_s_game,
        pit_attack_hi3_game,
        pit_attack_lw3_game,
        pit_attack_lw3_effect,
    );
}