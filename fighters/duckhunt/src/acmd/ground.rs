
use super::*;
#[acmd_script( agent = "duckhunt", script = "game_attackdash" , category = ACMD_GAME , low_priority)]
unsafe fn duckhunt_attackdash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 60, 40, 0, 100, 5.5, 0.0, 7.0, 9.0, Some(0.0), Some(7.0), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 105, 40, 0, 90, 3.5, 0.0, 7.0, 9.0, Some(0.0), Some(7.0), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
    AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "duckhunt", script = "game_attack12" , category = ACMD_GAME , low_priority)]
unsafe fn duckhunt_attack12_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter,  0,  0,  Hash40::new("top"),  1.5,  70,  30,  0,  30,  3.0,  0.0,  3.5,  2.0,  None,  None,  None,  1.2,  1.0,  *ATTACK_SETOFF_KIND_ON,  *ATTACK_LR_CHECK_F,  false,  0,  0.0,  0,  false,  false,  false,  false,  true,  *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL,  *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"),  *ATTACK_SOUND_LEVEL_S,  *COLLISION_SOUND_ATTR_PUNCH,  *ATTACK_REGION_HEAD);
        ATTACK(fighter,  1,  0,  Hash40::new("top"),  1.5,  70,  25,  0,  25,  5.0,  0.0,  7.5,  5.5,  None,  None,  None,  1.2,  1.0,  *ATTACK_SETOFF_KIND_ON,  *ATTACK_LR_CHECK_F,  false,  0,  0.0,  0,  false,  false,  false,  false,  true,  *COLLISION_SITUATION_MASK_GA,  *COLLISION_CATEGORY_MASK_ALL,  *COLLISION_PART_MASK_ALL,  false,  Hash40::new("collision_attr_normal"),  *ATTACK_SOUND_LEVEL_S,  *COLLISION_SOUND_ATTR_PUNCH,  *ATTACK_REGION_HEAD);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
    }
}

pub fn install() {
    install_acmd_scripts!(
        duckhunt_attackdash_game,
        duckhunt_attack12_game,
    );
}

