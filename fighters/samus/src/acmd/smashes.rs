
use super::*;

#[acmd_script( agent = "samus", script = "game_attacks4hi" , category = ACMD_GAME , low_priority)]
unsafe fn attack_s4_hi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 10.0);
    if is_excute(fighter) {
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

#[acmd_script( agent = "samus", script = "game_attacks4" , category = ACMD_GAME , low_priority)]
unsafe fn attack_s4_s(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
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

#[acmd_script( agent = "samus", script = "game_attacks4lw" , category = ACMD_GAME , low_priority)]
unsafe fn attack_s4_lw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 10.0);
    if is_excute(fighter) {
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

#[acmd_script( agent = "samus", script = "game_attackhi4" , category = ACMD_GAME , low_priority)]
unsafe fn attack_hi4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

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

#[acmd_script( agent = "samus", script = "effect_attackhi4", category = ACMD_EFFECT , low_priority)]
pub unsafe fn attack_hi4_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();

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

#[acmd_script( agent = "samus", script = "game_attackhi42", category = ACMD_GAME, low_priority)]
unsafe fn game_attackhi42(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent , 7.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent , 13.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 15.0, 83, 95, 0, 53, 3.0, 2.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 15.0, 83, 95, 0, 53, 2.75, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 15.0, 83, 95, 0, 53, 3.5, 0.0, 3.0, 3.0, Some(0.0), Some(9.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    wait(fighter.lua_state_agent , 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 2, false);
    }
    frame(fighter.lua_state_agent , 16.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 12.5, 83, 95, 0, 53, 3.0, 2.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 12.5, 83, 95, 0, 53, 2.75, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent , 20.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}
#[acmd_script( agent = "samus", script = "game_attackhi42_break", category = ACMD_GAME, low_priority)]
unsafe fn game_attackhi42_break(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        AttackModule::set_power(fighter.module_accessor, 0, 12.5, false);
        AttackModule::set_power(fighter.module_accessor, 1, 12.5, false);
    }
}

#[acmd_script( agent = "samus", script = "effect_attackhi42", category = ACMD_EFFECT, low_priority)]
unsafe fn effect_attackhi42(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent , 1.5);
    if macros::is_excute(fighter) {
        MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EFFECT, Hash40::new("effect_ice_punch"), -1);
    }
    frame(fighter.lua_state_agent , 5.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("armr"), 4.289, -0.272, -0.135, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent , 11.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent , 12.0);
    if macros::is_excute(fighter) {

        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_samus_ice1"), Hash40::new("tex_samus_ice1"), 4, 
        Hash40::new("armr"), 0.0, 0.0, -1.0, Hash40::new("armr"), 9.25, 0.0, -1.0, 
        true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 
        1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 3.0, 0.1);
    }
    frame(fighter.lua_state_agent , 20.0);
    if macros::is_excute(fighter) {
        macros::AFTER_IMAGE_OFF(fighter, 4);
    }
    frame(fighter.lua_state_agent , 30.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent , 31.0);
    if macros::is_excute(fighter) {
        EFFECT_OFF_KIND(fighter,Hash40::new("sys_ice"),false,false);
        if VarModule::is_flag(fighter.battle_object, vars::samus::status::ATTACK_HAS_ICE){
            macros::EFFECT(fighter, Hash40::new("sys_ice_landing"), Hash40::new("top"), 0, 0, 0.0, 0, 0, 0, 1.125, 0, 0, 0, 0, 0, 0, true);
        }
    }
}


#[acmd_script( agent = "samus", script = "sound_attackhi42", category = ACMD_SOUND, low_priority)]
unsafe fn sound_attackhi42(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent , 5.0);
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_common_smash_start"));
        macros::STOP_SE(fighter, Hash40::new("se_common_smash_start_02"));
    }
    frame(fighter.lua_state_agent , 10.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_swing_08"));
    }
    frame(fighter.lua_state_agent , 11.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_samus_attackdash"));
    }
    frame(fighter.lua_state_agent , 30.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_samus_landing02"));
    }
    frame(fighter.lua_state_agent , 31.0);
    if macros::is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::samus::status::ATTACK_HAS_ICE){
            macros::PLAY_SE(fighter, Hash40::new("se_item_freezer_landing"));
        }
    }
}

#[acmd_script( agent = "samus", script = "expression_attackhi42", category = ACMD_EXPRESSION, low_priority)]
unsafe fn expression_attackhi42(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent , 2.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 5);
    }
    frame(fighter.lua_state_agent , 3.0);
    if macros::is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_hide_gun") as i64);
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, true, -1);
        //ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, Hash40::new("special_s"), false, -1.0);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, Hash40::new("s4s2"), false, -1.0);
        LinkModule::send_event_nodes(fighter.module_accessor, *LINK_NO_ARTICLE, Hash40::new_raw(0x1c5609e30f), 0);
    }
    frame(fighter.lua_state_agent , 7.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 5);
    }
    frame(fighter.lua_state_agent , 8.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 2, true);
    }
    frame(fighter.lua_state_agent , 10.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 3);
    }
    frame(fighter.lua_state_agent , 11.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent , 13.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackll"), 0);
    }
    frame(fighter.lua_state_agent , 26.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 5);
    }
    frame(fighter.lua_state_agent , 30.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_lands"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
        
    frame(fighter.lua_state_agent , 55.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
    }
}

#[acmd_script( agent = "samus", script = "sound_attackhi4charge2", category = ACMD_SOUND, low_priority)]
unsafe fn sound_attackhi42charge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent , 2.0);
    if macros::is_excute(fighter) {
        //macros::PLAY_SE(fighter, Hash40::new("vc_samus_attack06"));
        macros::PLAY_SE(fighter, Hash40::new("se_common_smash_start_02"));
    }
}

#[acmd_script( agent = "samus", script = "effect_attackhi4charge2", category = ACMD_EFFECT, low_priority)]
unsafe fn effect_attackhi42charge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent , 5.0);
    for i in 1..i32::MAX{
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1, 15, 0, 4, 0, 0, 0, false);
            
            macros::EFFECT(fighter, Hash40::new("sys_starrod_splash"), Hash40::new("armr"), 5.0, 0.0, 0.0, 0, 0, -90, 1, 4, 4, 4, 0, 0, 0, true);
            LAST_EFFECT_SET_COLOR(fighter,0.375, 1.0,1.0);
        }
        wait(fighter.lua_state_agent , 5.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("armr"), 4.289, -0.272, -0.135, 0, 0, 0, 1, 4, 4, 4, 0, 0, 0, true);
        }
        wait(fighter.lua_state_agent , 5.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1, 15, 0, 4, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent , 5.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("armr"), 4.289, -0.272, -0.135, 0, 0, 0, 1, 4, 4, 4, 0, 0, 0, true);
        }
    }
}
#[acmd_script( agent = "samus", script = "expression_attackhi4charge2", category = ACMD_EXPRESSION, low_priority)]
unsafe fn expression_attackhi42charge(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_hide_gun") as i64);
        //if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN)
        {
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, true, -1);
            //ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, Hash40::new("s4s2"), false, -1.0);
            //physics!(*MA_MSC_CMD_PHYSICS_START_CHARGE, 0.9, 0.8, -1, 0.9, 0.8, -1, Hash40::new("invalid"));
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
            LinkModule::send_event_nodes(fighter.module_accessor, *LINK_NO_ARTICLE, Hash40::new_raw(0x1c5609e30f), 0);
        }
    }
    frame(fighter.lua_state_agent , 61.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_smashhold2"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}


#[acmd_script( agent = "samus", script = "game_attacks42", category = ACMD_GAME, low_priority)]
unsafe fn game_attacks42(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent , 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent , 12.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 12.0-2.0, 361, 112, 0, 16, 2.8, -1.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 12.0, 361, 100, 0, 40, 3.5, 3.75, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("armr"), 12.0, 361, 100, 0, 40, 3.25, 7.25, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 3, 0, Hash40::new("armr"), 12.0+2.0, 361, 100, 0, 40, 3.125, 14.0-1.3, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    wait(fighter.lua_state_agent , 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "samus", script = "game_attacks42_break", category = ACMD_GAME, low_priority)]
unsafe fn game_attacks42_break(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 12.0, 361, 112, 0, 16, 2.8, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("armr"), 12.0, 361, 112, 0, 40, 2.8, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);

        AttackModule::clear(fighter.module_accessor,3,false);
    }
}

#[acmd_script( agent = "samus", script = "effect_attacks42", category = ACMD_EFFECT, low_priority)]
unsafe fn effect_attacks42(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent , 1.5);
    if macros::is_excute(fighter) {
        MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EFFECT, Hash40::new("effect_ice_lance"), -1);
    }
    frame(fighter.lua_state_agent , 3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 12, 10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent , 12.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);

        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_samus_ice1"), Hash40::new("tex_samus_ice1"), 4, 
        Hash40::new("armr"), 3.0, 0.0, 0.0, Hash40::new("armr"), 17.0, 0.0, 0.0, 
        true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 
        1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
    }
    frame(fighter.lua_state_agent , 13.0);
    if macros::is_excute(fighter) {
        macros::AFTER_IMAGE_OFF(fighter, 4);
    }
    frame(fighter.lua_state_agent , 36.0);
    if macros::is_excute(fighter) {
        EFFECT_OFF_KIND(fighter,Hash40::new("sys_ice"),false,false);
    }
}


#[acmd_script( agent = "samus", script = "sound_attacks42", category = ACMD_SOUND, low_priority)]
unsafe fn sound_attacks42(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent , 5.0);
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_common_smash_start"));
        macros::STOP_SE(fighter, Hash40::new("se_common_smash_start_02"));
    }
    wait(fighter.lua_state_agent , 3.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_sword_swing_l"));
    }
    frame(fighter.lua_state_agent , 36.0);
    if macros::is_excute(fighter) {
        //macros::PLAY_SE(fighter, Hash40::new("se_item_ice_crash"));
    }
}

#[acmd_script( agent = "samus", script = "expression_attacks42", category = ACMD_EXPRESSION, low_priority)]
unsafe fn expression_attacks42(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent , 2.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
    frame(fighter.lua_state_agent , 3.0);
    if macros::is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_hide_gun") as i64);
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, true, -1);
        //ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, Hash40::new("special_s"), false, -1.0);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, Hash40::new("s4s2"), false, -1.0);
    }
    frame(fighter.lua_state_agent , 10.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_slashl"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent , 50.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
    }
}

#[acmd_script( agent = "samus", script = "sound_attacks42charge", category = ACMD_SOUND, low_priority)]
unsafe fn sound_attacks42charge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent , 2.0);
    if macros::is_excute(fighter) {
        //macros::PLAY_SE(fighter, Hash40::new("vc_samus_attack06"));
        macros::PLAY_SE(fighter, Hash40::new("se_common_smash_start_02"));
    }
}

#[acmd_script( agent = "samus", script = "effect_attacks42charge", category = ACMD_EFFECT, low_priority)]
unsafe fn effect_attacks42charge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent , 5.0);
    for i in 1..i32::MAX{
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 15, 0, 4, 0, 0, 0, false);
            
            macros::EFFECT(fighter, Hash40::new("sys_starrod_splash"), Hash40::new("armr"), 8.0, 0.0, 0.0, 0, 0, -90, 1, 4, 4, 4, 0, 0, 0, true);
            LAST_EFFECT_SET_COLOR(fighter,0.375, 1.0,1.0);
        }
        wait(fighter.lua_state_agent , 5.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("armr"), 4.289, -0.272, -0.135, 0, 0, 0, 1, 4, 4, 4, 0, 0, 0, true);
        }
        wait(fighter.lua_state_agent , 5.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 15, 0, 4, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent , 5.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("armr"), 4.289, -0.272, -0.135, 0, 0, 0, 1, 4, 4, 4, 0, 0, 0, true);
        }
    }
}
#[acmd_script( agent = "samus", script = "expression_attacks42charge", category = ACMD_EXPRESSION, low_priority)]
unsafe fn expression_attacks42charge(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_hide_gun") as i64);
        //if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN)
        {
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, true, -1);
            //ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, Hash40::new("s4s2"), false, -1.0);
            //physics!(*MA_MSC_CMD_PHYSICS_START_CHARGE, 0.9, 0.8, -1, 0.9, 0.8, -1, Hash40::new("invalid"));
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
            LinkModule::send_event_nodes(fighter.module_accessor, *LINK_NO_ARTICLE, Hash40::new_raw(0x1c5609e30f), 0);
        }
    }
    frame(fighter.lua_state_agent , 61.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_smashhold2"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    install_acmd_scripts!(
        attack_s4_hi,
        attack_s4_s,
        attack_s4_lw,
        attack_hi4,
        attack_hi4_effect,
        attack_hi4_sound,
        attack_lw4,
        
        game_attackhi42,
        game_attackhi42_break,

        effect_attackhi42,
        sound_attackhi42,
        expression_attackhi42,

        sound_attackhi42charge,
        effect_attackhi42charge,
        expression_attackhi42charge,

        
        game_attacks42,
        game_attacks42_break,

        effect_attacks42,
        sound_attacks42,
        expression_attacks42,

        sound_attacks42charge,
        effect_attacks42charge,
        expression_attacks42charge,
    );
}

