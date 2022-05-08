use super::*;


pub fn install() {
    install_acmd_scripts!(buddy_attack_hi4_game);
}

#[acmd_script( agent = "buddy", script = "game_attackhi4" , category = ACMD_GAME , low_priority)]
unsafe fn buddy_attack_hi4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 140, 100, 110, 0, 4.0, 0.0, 3.0, 6.0, Some(0.0), Some(3.0), Some(-6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 367, 100, 10, 0, 2.0, 0.0, 26.0, 0.0, None, None, None, 0.75, 0.4, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 1.5, 110, 100, 30, 0, 3.6, 0.0, 19.6, -1.4, Some(0.0), Some(19.6), Some(1.4), 0.75, 0.4, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 1.5, 130, 100, 60, 0, 4.2, 0.0, 11.2, -3.6, Some(0.0), Some(11.2), Some(3.6), 0.75, 0.4, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
    }
    wait(lua_state, 13.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 1, Hash40::new("top"), 3.6, 88, 172, 0, 76, 3.2, 0.0, 26.4, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 1, 1, Hash40::new("top"), 3.6, 88, 172, 0, 76, 4.2, 0.0, 19.6, -1.4, Some(0.0), Some(19.6), Some(1.4), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
        ATTACK(fighter, 2, 1, Hash40::new("top"), 3.6, 88, 172, 0, 76, 4.8, 0.0, 11.2, -3.6, Some(0.0), Some(11.2), Some(3.6), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}