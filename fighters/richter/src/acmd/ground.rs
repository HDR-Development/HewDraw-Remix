
use super::*;

#[acmd_script( agent = "richter", script = "game_attack11" , category = ACMD_GAME , low_priority)]
unsafe fn richter_attack_11_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 70, 45, 0, 25, 7.0, 0.0, 9.0, 6.2, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}

#[acmd_script( agent = "richter", script = "effect_attack11" , category = ACMD_EFFECT , low_priority)]
unsafe fn richter_attack_11_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 1, 9.5, 2.2, 13, -29, 154, 0.8, true);
        LAST_EFFECT_SET_RATE(fighter, 2.0);
        LAST_EFFECT_SET_COLOR(fighter, 0.902, 0.784, 0.333);
    }
}
#[acmd_script( agent = "richter", script = "sound_attack11" , category = ACMD_SOUND , low_priority)]
unsafe fn richter_attack_11_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_punch_kick_swing_m"));
    }
}


#[acmd_script( agent = "richter", script = "game_attack12" , category = ACMD_GAME , low_priority)]
unsafe fn richter_attack_12_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 70, 30, 0, 25, 3.5, 0.0, 3.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 70, 30, 0, 25, 3.5, 0.0, 2.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 70, 30, 0, 25, 4.0, 0.0, 1.0, 11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 3.0, 70, 30, 0, 25, 2.5, 0.0, 5.1, 3.0, Some(0.0), Some(5.1), Some(11.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);

        AttackModule::set_attack_height_all(boma, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}

#[acmd_script( agent = "richter", script = "effect_attack12" , category = ACMD_EFFECT , low_priority)]
unsafe fn richter_attack_12_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 4, 2.5, 9, -6.2, 0, 0.45, true);
    }
}

#[acmd_script( agent = "richter", script = "sound_attack12" , category = ACMD_SOUND , low_priority)]
unsafe fn richter_attack_12_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_punch_kick_swing_s"));
    }
}

#[acmd_script( agent = "richter", script = "game_attack13" , category = ACMD_GAME , low_priority)]
unsafe fn richter_attack_13_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 65, 80, 0, 60, 5.0, 0.0, 26.0, 9.5, Some(0.0), Some(6.0), Some(7.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_WHIP);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "richter", script = "effect_attack13" , category = ACMD_EFFECT , low_priority)]
unsafe fn richter_attack_13_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        EFFECT_FLW_POS(fighter, Hash40::new("richter_upper"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 0.7, true);
        LAST_EFFECT_SET_RATE(fighter, 0.9);
        FOOT_EFFECT(fighter, Hash40::new("sys_atk_smoke2"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.0);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("richter_upper"), true, true);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, false);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
        }
}

#[acmd_script( agent = "richter", script = "sound_attack13" , category = ACMD_SOUND , low_priority)]
unsafe fn richter_attack_13_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_richter_attack05"));
        PLAY_SE(fighter, Hash40::new("se_richter_special_h01"));
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_landing01"));
    }
}

#[acmd_script( agent = "richter", script = "expression_attack13", category = ACMD_EXPRESSION, low_priority )]
unsafe fn richter_attack_13_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
}

#[acmd_script( agent = "richter", script = "game_attackdash" , category = ACMD_GAME , low_priority)]
unsafe fn richter_attack_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 361, 50, 0, 20, 4.0, 0.0, 9.5, 0.0, Some(0.0), Some(8.5), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_kick"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 361, 90, 0, 30, 8.0, 0.0, 12.0, 6.0, None, None, None, 1.75, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_kick"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }

}
#[acmd_script( agent = "richter", script = "effect_attackdash" , category = ACMD_EFFECT , low_priority)]
unsafe fn richter_attack_dash_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 1, 12.0, 1.0, 12, -20, 160, 1.1, true);
        LAST_EFFECT_SET_RATE(fighter, 1.8);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 8, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 1, 11.0, 1.0, 12, -20, 150, 1.2, true);
        LAST_EFFECT_SET_RATE(fighter, 1.8);
        LAST_EFFECT_SET_COLOR(fighter, 0.89, 0.706, 0.106);
        }
    
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, false);
        LAST_EFFECT_SET_RATE(fighter, 2.0);
        }

}

#[acmd_script( agent = "richter", script = "sound_attackdash" , category = ACMD_SOUND , low_priority)]
unsafe fn richter_attack_dash_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_dash_start"));
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_punch_kick_swing_l"));
    }

    frame(lua_state, 27.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_landing01"));
    }
}

#[acmd_script( agent = "richter", script = "expression_attackdash", category = ACMD_EXPRESSION, low_priority )]
unsafe fn richter_attack_dash_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_rush"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
}

pub fn install() {
    install_acmd_scripts!(
        richter_attack_11_game,
        richter_attack_11_effect,
        richter_attack_11_sound,
        richter_attack_12_game,
        richter_attack_12_effect,
        richter_attack_12_sound,
        richter_attack_13_game,
        richter_attack_13_effect,
        richter_attack_13_sound,
        richter_attack_13_expression,
        richter_attack_dash_game,
        richter_attack_dash_effect,
        richter_attack_dash_sound,
        richter_attack_dash_expression
    );
}

