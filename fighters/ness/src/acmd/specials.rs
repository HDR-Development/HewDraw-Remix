
use smash::app::{sv_system, sv_animcmd::{frame, wait, wait_loop_clear}, self, lua_bind::*};
use smash::phx::*;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CAgentBase;
use smash::app::utility::*;
use smash_script::*;
use smashline::*;
use smashline::*;
use smash_script::macros::*;
use crate::utils::hdr;
use crate::vars::*;

#[acmd_script( agent = "ness", script = "game_specialnfire" , category = ACMD_GAME , low_priority)]
unsafe fn special_n_fire_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 75, 10, 0, 150, 6.0, 0.0, 5.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "ness", script = "game_specialairnfire" , category = ACMD_GAME , low_priority)]
unsafe fn special_air_n_fire_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 75, 10, 0, 150, 3.0, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "ness", script = "sound_specials" , category = ACMD_SOUND )]
unsafe fn sound_specials(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    let rng = app::sv_math::rand(smash::hash40("fighter"), 2);

    if rng == 0 {
        frame(lua_state, 5.0);
        if is_excute(fighter) {
            PLAY_SE_REMAIN(fighter, Hash40::new("vc_ness_002"));
            PLAY_SE(fighter, Hash40::new("se_ness_special_s03"));
        }
    }
    else {
        frame(lua_state, 5.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_ness_special_s03"));
        }
        frame(lua_state, 20.0);
        if is_excute(fighter) {
            PLAY_SE_REMAIN(fighter, Hash40::new("vc_ness_attack04"));
        }
    }
}

#[acmd_script( agent = "ness", script = "sound_specialairs" , category = ACMD_SOUND )]
unsafe fn sound_specialairs(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let rng = app::sv_math::rand(smash::hash40("fighter"), 2);

    if rng == 0 {
        frame(lua_state, 5.0);
        if is_excute(fighter) {
            PLAY_SE_REMAIN(fighter, Hash40::new("vc_ness_002"));
        }
    }
    else {
        frame(lua_state, 20.0);
        if is_excute(fighter) {
            PLAY_SE_REMAIN(fighter, Hash40::new("vc_ness_attack04"));
        }
    }
}

#[acmd_script( agent = "ness", script = "game_speciallwhold" , category = ACMD_GAME , low_priority)]
unsafe fn special_lw_hold_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    //wait_loop_clear(lua_state);
    for _ in 0..999 {
        if is_excute(fighter) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 361, 100, 70, 0, 8.5, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 10, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
        wait(lua_state, 6.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 14.0);
    }

}

pub fn install() {
    install_acmd_scripts!(
        sound_specials,
        sound_specialairs,
        special_n_fire_game,
        special_air_n_fire_game,
        special_lw_hold_game
    );
}