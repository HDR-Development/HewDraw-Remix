
use super::*;

#[acmd_script( agent = "rockman", script = "game_dash" , category = ACMD_GAME , low_priority)]
unsafe fn dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

#[acmd_script( agent = "rockman", script = "sound_dash" , category = ACMD_SOUND , low_priority)]
unsafe fn dash_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        let dash_sfx_handle = SoundModule::play_se(fighter.module_accessor, Hash40::new("se_rockman_dash_start"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, dash_sfx_handle as i32, 0.5, 0);
    }
    wait(lua_state, 6.0);
    if is_excute(fighter) {
        PLAY_STEP_FLIPPABLE(fighter, Hash40::new("se_rockman_step_right_m"), Hash40::new("se_rockman_step_left_m"));
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        PLAY_STEP_FLIPPABLE(fighter, Hash40::new("se_rockman_step_left_m"), Hash40::new("se_rockman_step_right_m"));
    }
}

#[acmd_script( agent = "rockman", script = "game_turndash" , category = ACMD_GAME , low_priority)]
unsafe fn turn_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

// #[acmd_script( agent = "rockman_rockbuster", script = "game_regular", category = ACMD_GAME, low_priority )]
// unsafe fn rockman_rockbuster_regular_game(fighter: &mut L2CAgentBase) {
//     let lua_state = fighter.lua_state_agent;
//     let boma = fighter.boma();
//     if is_excute(fighter) {
//         ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 0, 0, 0, 3.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, -1, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_ENERGY);
//     }
//     frame(lua_state, 4.0);
//     if is_excute(fighter) {
//         ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 0, 0, 0, 2.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, -1, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_ENERGY);
//     }
//     frame(lua_state, 8.0);
//     if is_excute(fighter) {
//         ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 0, 0, 0, 1.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, -1, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_ENERGY);
//     }
// }

#[acmd_script( agent = "rockman_rockbuster", script = "game_regular", category = ACMD_GAME, low_priority )]
unsafe fn rockman_rockbuster_regular_game(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 0, 0, 22, 3.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, -1, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_ENERGY);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 0, 0, 11, 2.2, 0.0, 0.0, 0.0, None, None, None, 0.75, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, -1, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_ENERGY);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 0, 0, 0, 1.8, 0.0, 0.0, 0.0, None, None, None, 0.1, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, -1, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_ENERGY);
    }
}

#[acmd_script( agent = "rockman_airshooter", script = "game_regular", category = ACMD_GAME, low_priority )]
unsafe fn rockman_airshooter_regular_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.25, 366, 100, 50, 0, 5.0, 0.0, 3.0, 0.0, None, None, None, 0.2, 0.7, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 6, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 0.5);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, vars::rockman_airshooter::status::MOVE);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 90, 50, 0, 90, 7.0, 0.0, 3.0, 0.0, None, None, None, 0.2, 0.7, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 0.5);
        WorkModule::on_flag(boma, *WEAPON_ROCKMAN_AIRSHOOTER_INSTANCE_WORK_ID_FLAG_ATTACK_VECTOR_REVERSE_UD_CHECK);
    }
}

#[acmd_script( agent = "rockman_hardknuckle", script = "game_regular" , category = ACMD_GAME , low_priority)]
unsafe fn rockman_hardknuckle_regular_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 270, 61, 0, 8, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 16.0, 270, 74, 0, 38, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 3.0);
	if is_excute(fighter) {
        WorkModule::on_flag(boma, *WEAPON_ROCKMAN_HARDKNUCKLE_INSTANCE_WORK_ID_FLAG_ATTACK_VECTOR_REVERSE_UD_CHECK);
    }
    frame(lua_state, 5.0);
	if is_excute(fighter) {
        sv_kinetic_energy!(
            set_speed,
            fighter,
            WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
            0.0,
            -0.5
        );
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
            0.0,
            0.0
        );
        sv_kinetic_energy!(
            set_brake,
            fighter,
            WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
            0.0,
            0.05
        );
    }
    frame(lua_state, 6.0);
	if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "rockman_hardknuckle", script = "effect_regular" , category = ACMD_EFFECT , low_priority)]
unsafe fn rockman_hardknuckle_regular_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("rockman_hardknuckle"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("rockman_hardknuckle"), true, true);
    }
}

/*
#[acmd_script( agent = "rockman_crashbomb", script = "game_regular" , category = ACMD_GAME , low_priority)]
unsafe fn rockman_crashbomb_stick_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
	let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if is_excute(fighter) {
        VisibilityModule::set_int64(boma, Hash40::new("body").hash as i64, Hash40::new("body_open").hash as i64);
        SEARCH(fighter, 0, 0, Hash40::new("top"), 4.0, 0.0, 0.0, 0.0, *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, true);
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        if VarModule::is_flag(owner_boma, rockman::DETONATE_CRASHBOMB){
            ArticleModule::change_status(boma, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_CRASHBOMB, *, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
    
}
*/

#[acmd_script( agent = "rockman", script = "game_escapeair" , category = ACMD_GAME , low_priority)]
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

#[acmd_script( agent = "rockman", script = "game_escapeairslide" , category = ACMD_GAME , low_priority)]
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
        rockman_rockbuster_regular_game,
        rockman_hardknuckle_regular_effect,
        rockman_airshooter_regular_game,
        rockman_hardknuckle_regular_game,
    );
}

