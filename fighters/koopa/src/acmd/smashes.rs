
use super::*;


#[acmd_script(agent = "koopa", script =  "game_attacks4", category = ACMD_GAME, low_priority)]
unsafe fn koopa_attack_s4_game(fighter: &mut L2CAgentBase) {
    	let lua_state = fighter.lua_state_agent;
        let boma = fighter.module_accessor;
        VarModule::on_flag(fighter.battle_object, vars::koopa::instance::PUNCH_CAN_ZOOM);
        VarModule::set_int(fighter.battle_object, vars::koopa::instance::PUNCH_ZOOM, 0);
        frame(lua_state, 13.0);
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        }
		frame(lua_state, 22.0);
		if macros::is_excute(fighter) {
			if !VarModule::is_flag(fighter.battle_object, vars::koopa::instance::IS_EXCELLENT_PUNCH) { // normal fsmash
                macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 23.0, 361, 97, 0, 28, 8.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            } else { // excellent punch data
                macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 25.5, 361, 97, 0, 28, 8.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
		frame(lua_state, 26.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
            EffectModule::kill_kind(boma, Hash40::new("sys_damage_fire"), false, false);
            VarModule::off_flag(fighter.battle_object, vars::koopa::instance::IS_EXCELLENT_PUNCH);
		}
}

#[acmd_script(agent = "koopa", script =  "effect_attacks4", category = ACMD_EFFECT, low_priority)] // shoutouts to phazoganon
unsafe fn koopa_attack_s4_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FLIP(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), 12, 15, 5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_YZ);
    }
    frame(lua_state, 7.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 20.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("koopa_smash_line"), Hash40::new("koopa_smash_line"), Hash40::new("top"), 0, 13, -20, 0, 0, 0, 2.3, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 21.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("koopa_smash_line"), Hash40::new("koopa_smash_line"), Hash40::new("top"), 0, 10, -26, 0, 0, 0, 2.3, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.9);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 12, 22, 0, 0, 0, 2.6, true);
    }
    frame(lua_state, 23.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 25.0);
    if macros::is_excute(fighter) {
        SlowModule::clear_whole(fighter.module_accessor);
        CameraModule::reset_all(fighter.module_accessor);
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_bg_criticalhit"), false, false);
        macros::CAM_ZOOM_OUT(fighter);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_line"), true, true);
    }
}	

#[acmd_script(agent = "koopa", script =  "sound_attacks4", category = ACMD_SOUND, low_priority)]
unsafe fn koopa_attack_s4_sound(fighter: &mut L2CAgentBase) {
    	let lua_state = fighter.lua_state_agent;
		frame(lua_state, 19.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_koopa_nailswing02"));
			macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack06"));
		}
		frame(lua_state, 21.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_koopa_smash_h01"));
		}
}	

#[acmd_script( agent = "koopa", script = "effect_attacks4charge", category = ACMD_EFFECT, low_priority )]
unsafe fn koopa_attack_s4_hold_effect(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    for _ in 0..34 {
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 20, 0, 8, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FLIP(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 6, 3, 6, 0, 0, 0, true, *EF_FLIP_YZ);
        }
    }
}

#[acmd_script( agent = "koopa", script = "game_attackhi4" , category = ACMD_GAME , low_priority)]
unsafe fn koopa_attack_hi4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 8.0);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("bust"), *HIT_STATUS_INVINCIBLE);
        HIT_NODE(fighter, Hash40::new("hip"), *HIT_STATUS_INVINCIBLE);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        ATTACK(fighter, 0, 0, Hash40::new("rot"), 22.0, 90, 90, 0, 23, 8.0, 0.0, -0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("rot"), 22.0, 90, 90, 0, 23, 6.5, 0.0, -3.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("rot"), 22.0, 90, 90, 0, 23, 6.5, 0.0, -3.0, -6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(lua_state, 37.0);
    if is_excute(fighter) {
        /* Ground-only */
        ATTACK(fighter, 0, 0, Hash40::new("rot"), 12.0, 280, 85, 0, 54, 8.0, 0.0, -1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("rot"), 12.0, 280, 85, 0, 54, 6.5, 0.0, -5.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("rot"), 12.0, 280, 85, 0, 54, 6.5, 0.0, -5.0, -8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        /* Air-only */
        ATTACK(fighter, 3, 0, Hash40::new("rot"), 12.0, 280, 36, 0, 54, 8.0, 0.0, -1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(fighter, 4, 0, Hash40::new("rot"), 12.0, 280, 36, 0, 54, 6.5, 0.0, -5.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(fighter, 5, 0, Hash40::new("rot"), 12.0, 280, 36, 0, 54, 6.5, 0.0, -5.0, -8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 38.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "koopa", script = "game_attacklw4" , category = ACMD_GAME , low_priority)]
unsafe fn koopa_attack_lw4_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 9.0);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 9.0);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 9.0);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        ATTACK(fighter, 0, 0, Hash40::new("handl"), 16.0, 40, 90, 0, 56, 6.0, 0.0, 0.0, 1.0, Some(5.3), Some(0.0), Some(1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("arml"), 14.5, 40, 90, 0, 56, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("kneel"), 14.5, 40, 90, 0, 56, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        FT_MOTION_RATE(fighter, 0.693);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        FT_MOTION_RATE(fighter, 1.000);
    }
    frame(lua_state, 28.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("handr"), 15.0, 40, 96, 0, 56, 6.0, 0.0, -0.6, 0.5, Some(4.5), Some(-0.6), Some(0.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("armr"), 13.5, 40, 96, 0, 56, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("kneer"), 13.5, 40, 96, 0, 56, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        koopa_attack_s4_game,
        koopa_attack_s4_effect,
        koopa_attack_s4_sound,
        koopa_attack_s4_hold_effect,
        koopa_attack_hi4_game,
        koopa_attack_lw4_game,
    );
}

