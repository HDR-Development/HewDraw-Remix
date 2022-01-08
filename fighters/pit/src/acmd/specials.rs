
use smash::app::{sv_system, sv_animcmd::{frame, wait}, self, lua_bind::*};
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

#[acmd_script( agent = "pit", script = "game_specialairnfires" , category = ACMD_GAME , low_priority)]
unsafe fn pit_special_n_fire_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        let stop_rise = Vector3f{x: 1.0, y: 0.0, z: 1.0};
        KineticModule::mul_speed(boma, &stop_rise, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let hop_vec = Vector3f{ x: 0.0, y: 1.0, z: 0.0 };
        KineticModule::add_speed(boma, &hop_vec);
        ArticleModule::shoot(boma, *FIGHTER_PIT_GENERATE_ARTICLE_BOWARROW, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
}

#[acmd_script( agent = "pit", script = "game_specialairnfirehi" , category = ACMD_GAME , low_priority)]
unsafe fn pit_special_n_fire_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        let stop_rise = Vector3f{x: 1.0, y: 0.0, z: 1.0};
        KineticModule::mul_speed(boma, &stop_rise, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let hop_vec = Vector3f{ x: 0.0, y: 1.0, z: 0.0 };
        KineticModule::add_speed(boma, &hop_vec);
        ArticleModule::shoot(boma, *FIGHTER_PIT_GENERATE_ARTICLE_BOWARROW, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
}

pub fn install() {
    install_acmd_scripts!(
        pit_special_n_fire_s_game,
        pit_special_n_fire_hi_game,
    );
}

