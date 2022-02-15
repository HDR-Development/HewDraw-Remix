
use super::*;

#[acmd_script( agent = "duckhunt", script = "game_attacks4" , category = ACMD_GAME , low_priority)]
unsafe fn duckhunt_attack_s4_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        WorkModule::set_int(boma, 5, *FIGHTER_DUCKHUNT_STATUS_ATTACK_INT_SMASH_DELAY_FRAME);
        WorkModule::set_int(boma, 6, *FIGHTER_DUCKHUNT_STATUS_ATTACK_INT_SMASH_RETICLE_DISPLAY_FRAME);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 45, 100, 66, 0, 6.0, 0.0, 6.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 45, 100, 70, 0, 7.0, 0.0, 6.0, 18.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        ATTACK(fighter, 2, 0, Hash40::new("top"), 9.0, 361, 131, 0, 50, 9.0, 0.0, 6.0, 26.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    
}

#[acmd_script( agent = "duckhunt", script = "game_attacklw4" , category = ACMD_GAME , low_priority)]
unsafe fn duckhunt_attack_lw4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        WorkModule::set_int(boma, 5, *FIGHTER_DUCKHUNT_STATUS_ATTACK_INT_SMASH_DELAY_FRAME);
        WorkModule::set_int(boma, 6, *FIGHTER_DUCKHUNT_STATUS_ATTACK_INT_SMASH_RETICLE_DISPLAY_FRAME);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 368, 100, 0, 0, 6.0, 0.0, 4.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        let hit1 = smash::phx::Vector2f { x: -9.0, y: 2.0 };
        AttackModule::set_vec_target_pos(boma, 0, smash::phx::Hash40::new("top"), &hit1, 8, false);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 368, 100, 0, 0, 7.0, 0.0, 5.0, -10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        let hit2 = smash::phx::Vector2f { x: 12.0, y: 3.0 };
        AttackModule::set_vec_target_pos(boma, 1, smash::phx::Hash40::new("top"), &hit2, 8, false);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 150, 160, 0, 37, 9.0, 0.0, 7.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        duckhunt_attack_s4_s_game,
        duckhunt_attack_lw4_game,
    );
}

