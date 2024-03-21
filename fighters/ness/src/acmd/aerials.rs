
use super::*;

unsafe extern "C" fn game_attackairn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        ATTACK(fighter, 0, 0, Hash40::new("handr"), 11.0, 361, 100, 0, 25, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        ATTACK(fighter, 1, 0, Hash40::new("handl"), 11.0, 361, 100, 0, 25, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        ATTACK(fighter, 2, 0, Hash40::new("hip"), 11.0, 361, 100, 0, 25, 5.4, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("handr"), 8.0, 361, 90, 0, 25, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        ATTACK(fighter, 1, 0, Hash40::new("handl"), 8.0, 361, 90, 0, 25, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        ATTACK(fighter, 2, 0, Hash40::new("hip"), 8.0, 361, 90, 0, 25, 5.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

}

unsafe extern "C" fn game_attackairf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 361, 83, 0, 19, 4.9, 0.0, 4.8, 10.5, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 1.5, 361, 83, 0, 19, 2.5, 0.0, 3.8, 6.0, Some(0.0), Some(3.8), Some(3.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 361, 83, 0, 19, 4.9, 0.0, 4.8, 10.5, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 1.5, 361, 83, 0, 19, 2.5, 0.0, 3.8, 6.0, Some(0.0), Some(3.8), Some(3.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 361, 125, 0, 12, 4.9, 0.0, 4.8, 10.5, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 1.5, 361, 125, 0, 12, 2.5, 0.0, 3.8, 6.0, Some(0.0), Some(3.8), Some(3.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 05, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 361, 125, 0, 12, 4.9, 0.0, 4.8, 10.5, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 1.5, 361, 125, 0, 12, 2.5, 0.0, 3.8, 6.0, Some(0.0), Some(3.8), Some(3.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 361, 140, 0, 32, 8.0, 0.0, 4.8, 9.6, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 33.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

}

unsafe extern "C" fn expression_attackairf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 6);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohits"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_attackairb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    MotionModule::set_rate(boma, 11.0/(10.0-1.0));
    frame(lua_state, 10.0);
    MotionModule::set_rate(boma, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 16.0, 361, 100, 0, 16, 4.0, -1.0, 0.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 16.0, 361, 100, 0, 16, 5.0, 3.7, 2.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 10.0, 361, 100, 0, 16, 4.0, -1.0, 0.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 10.0, 361, 100, 0, 16, 5.0, 3.7, 2.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
    }
    wait(lua_state, 6.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

}

unsafe extern "C" fn game_attackairhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    MotionModule::set_rate(boma, 7.5 / 7.0);
    frame(lua_state, 7.5);
    MotionModule::set_rate(boma, 1.0);
    frame(lua_state, 8.5);
    MotionModule::set_rate(boma, (14.0 - 8.5) / 2.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        ATTACK(fighter, 0, 0, Hash40::new("handr"), 13.0, 85, 118, 0, 13, 5.9, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
    }
    frame(lua_state, 14.0);
    MotionModule::set_rate(boma, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("handr"), 13.0, 85, 118, 0, 13, 5.9, 0.0, 0.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
    }
    frame(lua_state, 16.0);
    MotionModule::set_rate(boma, (37.0 - 16.0) / 28.0);//43
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 26.5);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn game_attackairlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    MotionModule::set_rate(boma, 2.72);
    frame(lua_state, 19.0);
    MotionModule::set_rate(boma, 1.0);
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 14.0, 270, 74, 0, 37, 6.0, 3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PSI);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 14.0, 270, 37, 0, 37, 6.0, 3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PSI);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 12.0, 45, 74, 0, 30, 4.5, 2.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PSI);
        AttackModule::clear(boma, 1, false);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 39.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

}

unsafe extern "C" fn effect_attackairn (fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let boma = fighter.boma();
	frame(lua_state, 5.0);
	if is_excute(fighter) {
        fighter.clear_lua_stack();
		EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4.7, 4, 40, 0, -13, 0.7, true);
		lua_args!(fighter, Hash40::new("ness_psi_atk"), Hash40::new("handr"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, true);
        smash::app::sv_animcmd::EFFECT_FOLLOW_NO_SCALE(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
		lua_args!(fighter, Hash40::new("ness_psi_atk"), Hash40::new("handl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, true);
        smash::app::sv_animcmd::EFFECT_FOLLOW_NO_SCALE(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
	}
	frame(lua_state, 11.0);
	if is_excute(fighter) {
        fighter.clear_lua_stack();
		lua_args!(fighter, Hash40::new("ness_psi_atk"), Hash40::new("handr"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, true);
		smash::app::sv_animcmd::EFFECT_FOLLOW_NO_SCALE(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
        fighter.clear_lua_stack();
        lua_args!(fighter, Hash40::new("ness_psi_atk"), Hash40::new("handl"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, true);
        smash::app::sv_animcmd::EFFECT_FOLLOW_NO_SCALE(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
    }
}

unsafe extern "C" fn effect_attackairb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), -2, 3.7, 3, 0, 180, 0, 0.85, true, *EF_FLIP_YZ);
        LAST_PARTICLE_SET_COLOR(fighter, 0.2, 1, 0.8);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("ness_psi_atk"), Hash40::new("ness_psi_atk"), Hash40::new("top"), -3.0, 4, -8.45, 0, 180, 0, 1.24, true, *EF_FLIP_YZ);
    }
}

unsafe extern "C" fn effect_attackairhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 8, 0, 0, 285, 275, 0.5, true);
    }
    frame(lua_state, 8.2);
    if is_excute(fighter) {
        fighter.clear_lua_stack();
        lua_args!(fighter, Hash40::new("ness_psi_rush"), Hash40::new("top"), 0.0, 10.2, -5.5, 0.0, 0.0, 0.0, 0.82, true);
        smash::app::sv_animcmd::EFFECT_FOLLOW_NO_SCALE(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
    }
    frame(lua_state, 8.5);
    if is_excute(fighter) {
        fighter.clear_lua_stack();
        lua_args!(fighter, Hash40::new("ness_psi_rush"), Hash40::new("top"), 0.0, 13.2, -1.0, 0.0, 0.0, 0.0, 0.9, true);
        smash::app::sv_animcmd::EFFECT_FOLLOW_NO_SCALE(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
    }
    frame(lua_state, 9.75);
    if is_excute(fighter) {
        fighter.clear_lua_stack();
        lua_args!(fighter, Hash40::new("ness_psi_rush"), Hash40::new("top"), 0.0, 10, 4.8, 0.0, 0.0, 0.0, 0.82, true);
        smash::app::sv_animcmd::EFFECT_FOLLOW_NO_SCALE(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        fighter.clear_lua_stack();
        lua_args!(fighter, Hash40::new("ness_psi_rush"), Hash40::new("top"), 0.0, 5.0, 5.4, 0.0, 0.0, 0.0, 0.78, true);
        smash::app::sv_animcmd::EFFECT_FOLLOW_NO_SCALE(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
    }
}

unsafe extern "C" fn effect_attackairlw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, -4.0, 0, 0, 0, 0, 0.27, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 1.3);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("ness_psi_atk"), Hash40::new("top"), 0.3 * PostureModule::lr(boma), -0.9, 0, 0, 0, 0, 1.5, true);
    }
}

unsafe extern "C" fn sound_attackairhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.5);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_ness_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_ness_attackair_h01"));
        PLAY_SE(fighter, Hash40::new("se_ness_pk_l"));
    }
}

unsafe extern "C" fn expression_attackairhi(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 8.5);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
}

pub fn install() {
    smashline::Agent::new("ness")
        .acmd("game_attackairn", game_attackairn)
        .acmd("game_attackairf", game_attackairf)
        .acmd("expression_attackairf", expression_attackairf)
        .acmd("game_attackairb", game_attackairb)
        .acmd("game_attackairhi", game_attackairhi)
        .acmd("game_attackairlw", game_attackairlw)
        .acmd("effect_attackairn", effect_attackairn)
        .acmd("effect_attackairb", effect_attackairb)
        .acmd("effect_attackairhi", effect_attackairhi)
        .acmd("effect_attackairlw", effect_attackairlw)
        .acmd("sound_attackairhi", sound_attackairhi)
        .acmd("expression_attackairhi", expression_attackairhi)
        .install();
}
