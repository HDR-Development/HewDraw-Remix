
use super::*;


#[acmd_script( agent = "lucario", script = "game_attacks4" , category = ACMD_GAME , low_priority)]
unsafe fn lucario_attack_s4_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        MeterModule::watch_damage(fighter.battle_object, true);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 361, 100, 0, 40, 5.7, 0.0, 8.0, 11.7, Some(0.0), Some(8.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13313725f6), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 361, 100, 0, 40, 3.3, 0.0, 8.0, 18.2, Some(0.0), Some(8.0), Some(6.2), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13313725f6), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        MeterModule::watch_damage(fighter.battle_object, false);
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "lucario", script = "game_attackhi4" , category = ACMD_GAME , low_priority)]
unsafe fn lucario_attack_hi4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        MeterModule::watch_damage(fighter.battle_object, true);
        ATTACK(fighter, 0, 0, Hash40::new("handr"), 5.0, 85, 100, 100, 0, 4.6, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        SA_SET(fighter, *SITUATION_KIND_AIR);
        // GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        ATTACK(fighter, 0, 0, Hash40::new("armr"), 7.0, 80, 121, 0, 78, 5.0, 3.0, -0.4, 0.0, Some(-3.0), Some(-0.4), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 16.0);
    FT_MOTION_RATE(fighter, 2.0);
    if is_excute(fighter) {
        MeterModule::watch_damage(fighter.battle_object, false);
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "lucario", script = "game_attacklw4" , category = ACMD_GAME , low_priority)]
unsafe fn lucario_attack_lw4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 6.0);
    FT_MOTION_RATE_RANGE(fighter, 6.0, 19.0, 7.0);
    frame(lua_state, 19.0);
    FT_MOTION_RATE(fighter, 1.000);
    if is_excute(fighter) {
        MeterModule::watch_damage(fighter.battle_object, true);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 33, 99, 0, 30, 4.3, 0.0, 6.0, 15.0, Some(0.0), Some(6.0), Some(4.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13313725f6), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 33, 99, 0, 30, 4.3, 0.0, 6.0, -15.0, Some(0.0), Some(6.0), Some(-4.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13313725f6), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        MeterModule::watch_damage(fighter.battle_object, false);
        AttackModule::clear_all(boma);
    }
}

pub fn install() {
    install_acmd_scripts!(
        lucario_attack_s4_s_game,
        lucario_attack_hi4_game,
        lucario_attack_lw4_game,
    );
}

