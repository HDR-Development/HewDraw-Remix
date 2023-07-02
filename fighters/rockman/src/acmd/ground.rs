
use super::*;


#[acmd_script( agent = "rockman", script = "game_attackdash" , category = ACMD_GAME , low_priority)]
unsafe fn rockman_attack_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.75);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 40, 70, 0, 60, 4.0, 0.0, 5.0, 5.0, Some(0.0), Some(11.0), Some(5.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 6.0);
    if macros::is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 6.5, 85, 70, 0, 75, 2.5, 0.0, 5.0, 5.0, Some(0.0), Some(10.0), Some(5.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    FT_MOTION_RATE(fighter, 0.5);
}

#[acmd_script( agent = "rockman", script = "effect_attackdash", category = ACMD_EFFECT )]
unsafe fn rockman_attack_dash_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    for _ in 0..5 {
        if is_excute(fighter) {
            EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), -3, 8, 0, 0, 180, 0, 0.55, true, *EF_FLIP_YZ, 0.6);
            EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 4, 11, -1, 0, 180, 0, 0.55, true, *EF_FLIP_YZ, 0.6);
            EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 3, -6, 0, 180, 0, 0.6, true, *EF_FLIP_YZ, 0.6);
        }
        wait(lua_state, 5.0);
    }
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
}

#[acmd_script( agent = "rockman", script = "sound_attackdash", category = ACMD_SOUND )]
unsafe fn rockman_attack_dash_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_rockman_attackhard_l01"));
    }
}

#[acmd_script( agent = "rockman", script = "expression_attackdash", category = ACMD_EXPRESSION )]
unsafe fn rockman_attack_dash_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, MA_MSC_CMD_SLOPE_SLOPE_INTP, SLOPE_STATUS_LR, 6);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f5b14bb65), FIGHTER_ROCKMAN_ARM_LEFT, FIGHTER_ROCKMAN_ARMFORM_HAND, 5);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f5b14bb65), FIGHTER_ROCKMAN_ARM_RIGHT, FIGHTER_ROCKMAN_ARMFORM_HAND, 5);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(
            boma,
            Hash40::new("rbkind_nohitm"),
            7,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 6);
    }
    wait(lua_state, 6.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 6);
    }
}

pub fn install() {
    install_acmd_scripts!(
        rockman_attack_dash_game, rockman_attack_dash_effect, rockman_attack_dash_sound, rockman_attack_dash_expression
    );
}

