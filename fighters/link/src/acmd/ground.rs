
use super::*;


#[acmd_script( agent = "link", script = "game_attack11" , category = ACMD_GAME , low_priority)]
unsafe fn attack_11(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 4.0/(8.0-1.0));
    frame(lua_state, 8.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("sword1"), 4.0, 70, 25, 0, 30, 3.0, 1.0, 0.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("sword1"), 4.0, 70, 25, 0, 30, 3.0, 6.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("sword1"), 4.0, 70, 25, 0, 30, 3.0, 11.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 4.0, 361, 25, 0, 25, 2.5, 0.0, 3.0, 18.0, Some(0.0), Some(3.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}

#[acmd_script( agent = "link", script = "game_attack12" , category = ACMD_GAME , low_priority)]
unsafe fn attack_12 (fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let boma = fighter.boma();
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("sword1"), 3.0, 361, 25, 0, 25, 3.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 1, 0, Hash40::new("sword1"), 3.0, 361, 25, 0, 25, 3.0, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 2, 0, Hash40::new("sword1"), 3.0, 361, 25, 0, 25, 3.0, 11.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
	}
}

#[acmd_script( agent = "link", script = "game_attack13" , category = ACMD_GAME , low_priority)]
unsafe fn attack_13 (fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let boma = fighter.boma();
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("sword1"), 4.0, 361, 70, 0, 70, 3.0, 1.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 1, 0, Hash40::new("sword1"), 4.0, 361, 70, 0, 70, 3.0, 6.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		ATTACK(fighter, 2, 0, Hash40::new("sword1"), 4.0, 361, 70, 0, 70, 3.0, 11.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}

#[acmd_script( agent = "link", script = "game_attackdash" , category = ACMD_GAME , low_priority)]
unsafe fn game_attackdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let nodes = ["head", "bust", "waist", "arml", "shoulderl", "armr", "shoulderr"]; // nodes that become intangible during the attack
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        for node in nodes { HIT_NODE(fighter, Hash40::new(node), *HIT_STATUS_INVINCIBLE); }
        ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 70, 55, 0, 80, 3.5, 0.0, 4.0, 6.0, Some(0.0), Some(12.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
	frame(lua_state, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 65, 30, 0, 80, 6.0, 0.0, 8.0, 5.0, Some(0.0), Some(8.0), Some(5.75), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        for node in nodes { HIT_NODE(fighter, Hash40::new(node), *HIT_STATUS_NORMAL); }
    }
}

#[acmd_script( agent = "link", script = "effect_attackdash", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_attackdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("shield"), 0, 0, 0, 0, 0, 0, 0.6, true);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
        FOOT_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 11, 0, -3, 0, 0, 0, 0.45, 0, 0, 3, 0, 0, 0, false);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 9, 2, 0, 180, 0, 1.1, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.6, 0.6, 0.6);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 9.2, -1.0, 0, 0, 45, 0.85, true);
        LAST_EFFECT_SET_RATE(fighter, 2.0);
        LAST_EFFECT_SET_COLOR(fighter, 0.6, 0.6, 0.6);
    }
    frame(lua_state, 34.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "link", script = "sound_attackdash", category = ACMD_SOUND, low_priority )]
unsafe fn sound_attackdash(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_link_dash_start_ft"));
    }
    wait(lua_state, 6.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_link_rnd_attack"));
        PLAY_SE(agent, Hash40::new("se_link_swing_m"));
    }
    wait(lua_state, 19.0);
    if is_excute(agent) {
        PLAY_LANDING_SE(agent, Hash40::new("se_link_landing01"));
    }
}

#[acmd_script( agent = "link", script = "expression_attackdash", category = ACMD_EXPRESSION, low_priority )]
unsafe fn expression_attackdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        AttackModule::set_attack_reference_joint_id(boma, Hash40::new("sword1"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
        ItemModule::set_have_item_visibility(boma, false, 0);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_jump"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_slashm"), 0);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_R, 3);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_landl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
}

pub fn install() {
    install_acmd_scripts!(
        attack_11,
        attack_12,
        attack_13,
        game_attackdash,
        effect_attackdash,
        sound_attackdash,
        expression_attackdash
    );
}

