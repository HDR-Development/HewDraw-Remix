use super::*;

#[acmd_script( agent = "peach", script = "game_attacklw4" , category = ACMD_GAME , low_priority)]
unsafe fn peach_attack_lw4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 0, 100, 20, 0, 3.8, -2.5, 2.0, 0.0, Some(2.5), Some(2.0), Some(0.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 180, 100, 25, 0, 4.8, -2.0, 2.0, 7.5, Some(2.0), Some(2.0), Some(7.5), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 180, 100, 25, 0, 4.8, -2.0, 2.0, -7.5, Some(2.0), Some(2.0), Some(-7.5), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 0, 100, 20, 0, 3.8, -2.5, 2.0, 0.0, Some(2.5), Some(2.0), Some(0.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 2.5, 180, 100, 25, 0, 4.8, -2.0, 2.0, 7.5, Some(2.0), Some(2.0), Some(7.5), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 180, 100, 25, 0, 4.8, -2.0, 2.0, -7.5, Some(2.0), Some(2.0), Some(-7.5), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 45, 120, 0, 40, 4.8, 0.0, 1.8, 8.0, Some(0.0), Some(1.8), Some(-8.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        AttackModule::set_attack_height_all(boma, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "peach", script = "sound_attacklw4", category = ACMD_SOUND, low_priority )]
unsafe fn peach_attack_lw4_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_common_smash_start_03"));
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_peach_attack07"));
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_peach_smash_l01"));
    }
    wait(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_peach_smash_l02"));
    }
}

#[acmd_script( agent = "peach", script = "effect_attacklw4", category = ACMD_EFFECT, low_priority )]
unsafe fn peach_attack_lw4_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 3, 6, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_COLOR(fighter, 1.3, 1.3, 1.3);
        }
        if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 { 
            frame(lua_state, 4.0);
            if is_excute(fighter) {
                EFFECT_FOLLOW(fighter, Hash40::new("peach_smash_arc"), Hash40::new("top"), 0, 1, 0, 0, -90, 0, 0.95, true);
                LAST_EFFECT_SET_COLOR(fighter, 1.3, 1.3, 1.3);
            }
            frame(lua_state, 5.0);
            if is_excute(fighter) {
                EFFECT_FOLLOW(fighter, Hash40::new("peach_smash_arc"), Hash40::new("top"), 0, 1.8, 1, 0, 120, 0, 0.95, true);
                LAST_EFFECT_SET_COLOR(fighter, 1.3, 1.3, 1.3);
            }
            frame(lua_state, 9.0);
            if is_excute(fighter) {
                EFFECT_FOLLOW(fighter, Hash40::new("peach_smash_arc"), Hash40::new("top"), 0, 1, 0, 0, -90, 0, 1.0, true);
                LAST_EFFECT_SET_COLOR(fighter, 1.3, 1.3, 1.3);
            }
            frame(lua_state, 13.0);
            if is_excute(fighter) {
                LANDING_EFFECT(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
                LAST_EFFECT_SET_RATE(fighter, 1.1);
                EFFECT_FOLLOW(fighter, Hash40::new("peach_smash_arc"), Hash40::new("top"), 0, 1.8, 1, 0, -90, 0, 0.95, true);
                LAST_EFFECT_SET_COLOR(fighter, 1.3, 1.3, 1.3);
            }
        }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("peach_smash_arc"), Hash40::new("top"), 0, 1, 0, 180, -90, 0, 0.95, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.3, 1.3, 1.3);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("peach_smash_arc"), Hash40::new("top"), 0, 1.8, -1, 180, 120, 0, 0.95, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.3, 1.3, 1.3);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("peach_smash_arc"), Hash40::new("top"), 0, 1, 0, 180, -90, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.3, 1.3, 1.3);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.05, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.1);
        EFFECT_FOLLOW(fighter, Hash40::new("peach_smash_arc"), Hash40::new("top"), 0, 1.8, -1, 180, -90, 0, 1.05, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.3, 1.3, 1.3);
    }
}

pub fn install() {
    install_acmd_scripts!(
        peach_attack_lw4_game,
        peach_attack_lw4_sound,
        peach_attack_lw4_effect
    );
}