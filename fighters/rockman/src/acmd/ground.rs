
use super::*;

#[acmd_script( agent = "rockman", script = "game_attack11", category = ACMD_GAME, low_priority )]
unsafe fn rockman_attack_11_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        ArticleModule::generate_article_enable(boma, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_ROCKBUSTER, false, 0);
        WorkModule::on_flag(boma, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_ROCKBUSTER_SHOOT);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 361, 135, 0, 40, 3.0, 0.0, 9.6, 5.5, Some(0.0), Some(3.7), Some(5.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 361, 135, 0, 40, 3.0, 0.0, 8.0, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 361, 25, 0, 25, 2.5, 0.0, 3.2, 5.5, Some(0.0), Some(3.2), Some(8.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "rockman", script = "game_attackdash" , category = ACMD_GAME , low_priority)]
unsafe fn rockman_attack_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.8);
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 11.0 / (22.0 - 8.0));
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 70, 0, 60, 4.0, 0.0, 5.0, 5.0, Some(0.0), Some(11.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 13.0);
    if macros::is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 6.5, 65, 70, 0, 75, 2.5, 0.0, 5.0, 5.0, Some(0.0), Some(10.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        FT_MOTION_RATE(fighter, 14.0 / (58.0 - 25.0));
    }
    frame(lua_state, 58.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
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
        rockman_attack_11_game,
        rockman_attack_dash_game,
        rockman_attack_dash_effect,
        rockman_attack_dash_sound,
        rockman_attack_dash_expression,
    );
}

