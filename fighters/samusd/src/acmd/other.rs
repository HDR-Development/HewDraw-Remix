
use super::*;

#[acmd_script( agent = "samusd", script = "game_dash" , category = ACMD_GAME , low_priority)]
unsafe fn dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.5);
    }
	frame(lua_state, 11.0); // Effectively F16
    if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
		WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

#[acmd_script( agent = "samusd", script = "sound_dash" , category = ACMD_SOUND , low_priority)]
unsafe fn dash_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        let dash_sfx_handle = SoundModule::play_se(fighter.module_accessor, Hash40::new("se_samusd_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
}

#[acmd_script( agent = "samusd", script = "game_turndash" , category = ACMD_GAME , low_priority)]
unsafe fn turn_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

#[acmd_script( agent = "samusd_cshot", script = "game_shoot" , category = ACMD_GAME , low_priority)]
unsafe fn samusd_cshot_shoot_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 42, 0, 14, 1.9, 0.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 20.0, 40, 72, 0, 50, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -7, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
		attack!(fighter, *MA_MSC_CMD_ATTACK_SET_LERP, 0, 1);
	}
    
}

#[acmd_script( agent = "samusd_cshot", script = "sound_shoot", category = ACMD_SOUND, low_priority)]
unsafe fn samusd_cshot_shoot_sound (fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_samusd_special_n01"));
        if WorkModule::get_float(boma, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE) <= 0.25 {
            PLAY_SE_REMAIN(fighter, Hash40::new("se_samusd_special_n02"));
        }
        else if WorkModule::get_float(boma, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE) <= 0.625 {
            PLAY_SE_REMAIN(fighter, Hash40::new("se_samusd_special_n03"));
        }
        else if WorkModule::get_float(boma, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE) <= 0.875 {
            PLAY_SE_REMAIN(fighter, Hash40::new("se_samusd_special_n04"));
        }
        else {
            PLAY_SE_REMAIN(fighter, Hash40::new("se_samusd_special_n05"));
        }
    }
    
}

#[acmd_script( agent = "samusd_missile", script = "game_homing", category = ACMD_GAME, low_priority)]
unsafe fn samusd_homing_missile_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 0, 25, 0, 26, 2.4, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(boma);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 60, 55, 0, 26, 2.4, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(boma);
    }
}

#[acmd_script( agent = "samusd_missile", script = "effect_homing", category = ACMD_EFFECT, low_priority)]
unsafe fn samusd_homing_missile_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_missile_homing"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_missile_homing"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, true);
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 0, 3, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 2.0, 0.5, 0.5);
    }
    frame(lua_state, 50.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("samusd_missile_homing"), false, true);
    }
}

#[acmd_script( agent = "samusd_supermissile", script = "game_ready", category = ACMD_GAME, low_priority)]
unsafe fn samusd_super_missile_ready_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 65, 65, 0, 50, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(boma);
    }
}

#[acmd_script( agent = "samusd_supermissile", script = "game_straight", category = ACMD_GAME, low_priority)]
unsafe fn samusd_super_missile_straight_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 65, 65, 0, 50, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
    }
}

// #[acmd_script( agent = "samusd_supermissile", script = "game_sburst", category = ACMD_GAME, low_priority)]
// unsafe fn samusd_super_missile_burst_game(fighter: &mut L2CAgentBase) {
//     let lua_state = fighter.lua_state_agent;
//     let boma = fighter.boma();
//     if is_excute(fighter) {
//         ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 75, 70, 0, 50, 12.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
//         ControlModule::set_rumble(boma, Hash40::new("rbkind_erase"), 0, false, 0x50000000 /* default value */);
//     }
//     wait(lua_state, 1.0);
//     if is_excute(fighter) {
//         ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 0, false, 0x50000000 /* default value */);
//         notify_event_msc_cmd!(fighter, Hash40::new_raw(0x199c462b5d), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
//     }
// }

#[acmd_script( agent = "samusd_bomb", script = "game_fall", category = ACMD_GAME, low_priority)]
unsafe fn samusd_bomb_fall_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_SAMUSD { // Ensure the owner is Dark Samus.
        let dsamus = utils::util::get_battle_object_from_id(owner_id);
        VarModule::set_int(dsamus, vars::samusd::instance::BOMB_OBJECT_ID, fighter.battle_object_id as i32);
        // Store the bomb's object id in Dark Samus's VarModule, if the owner is Dark Samus.
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 361, 45, 0, 22, 2.0, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 6, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 40.0);
    if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_SAMUSD {
        let dsamus = utils::util::get_battle_object_from_id(owner_id);
        VarModule::on_flag(dsamus, vars::samusd::instance::MANUAL_DETONATE_READY);
    }
}

#[acmd_script( agent = "samusd_bomb", script = "game_burstattack", category = ACMD_GAME, low_priority)]
unsafe fn samusd_bomb_burst_attack_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_SAMUSD {
        let dsamus = utils::util::get_battle_object_from_id(owner_id);
        VarModule::off_flag(dsamus, vars::samusd::instance::MANUAL_DETONATE_READY);
    }
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 90, 90, 0, 70, 10.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 6, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 0, false, 0);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AREA_WIND_2ND_RAD(fighter, 0, 0.5, 0.02, 1000, 1, 0, 0, 16);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AreaModule::erase_wind(boma, 0);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::set_size(boma, 0, 4.9);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::set_size(boma, 0, 2.5);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x199c462b5d));
    }
}

#[acmd_script( agent = "samusd_bomb", script = "effect_burstattack", category = ACMD_EFFECT, low_priority)]
unsafe fn samusd_bomb_burst_attack_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_hit_purple"), Hash40::new("top"), 0, 0.5, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("samusd_entry"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 77.0/24.0);
    }
}

#[acmd_script( agent = "samusd", script = "game_escapeair" , category = ACMD_GAME , low_priority)]
unsafe fn escape_air_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let escape_air_cancel_frame = WorkModule::get_param_float(boma, hash40("param_motion"), hash40("escape_air_cancel_frame"));

    frame(lua_state, 29.0);
    if is_excute(fighter) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
    }
    frame(lua_state, escape_air_cancel_frame);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd_script( agent = "samusd", script = "game_escapeairslide" , category = ACMD_GAME , low_priority)]
unsafe fn escape_air_slide_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
    }
    frame(lua_state, 39.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

pub fn install() {
    install_acmd_scripts!(
        escape_air_game,
        escape_air_slide_game,
        dash_game,
        dash_sound,
        turn_dash_game,
        samusd_cshot_shoot_game,
        samusd_cshot_shoot_sound,
        samusd_homing_missile_game,
        samusd_homing_missile_effect,
        samusd_super_missile_ready_game,
        samusd_super_missile_straight_game,
        //samusd_super_missile_burst_game,
        samusd_bomb_fall_game,
        samusd_bomb_burst_attack_game,
        samusd_bomb_burst_attack_effect,
    );
}

