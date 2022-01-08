
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

macro_rules! smash_attacks_common { 
    ($fighter:ident) => {{
        let lua_state = $fighter.lua_state_agent;
        let boma = sv_system::battle_object_module_accessor(lua_state);
        frame(lua_state, 1.0);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_XLU), 0);
        original!($fighter);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }}
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attackhi4" , category = ACMD_GAME , low_priority)]
unsafe fn game_attackhi4(fighter: &mut L2CAgentBase) {
    smash_attacks_common!(fighter);
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attackhi4_b" , category = ACMD_GAME , low_priority)]
unsafe fn game_attackhi4_b(fighter: &mut L2CAgentBase) {
    smash_attacks_common!(fighter);
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attackhi4_y" , category = ACMD_GAME , low_priority)]
unsafe fn game_attackhi4_y(fighter: &mut L2CAgentBase) {
    smash_attacks_common!(fighter);
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attackhi4_w" , category = ACMD_GAME , low_priority)]
unsafe fn game_attackhi4_w(fighter: &mut L2CAgentBase) {
    smash_attacks_common!(fighter);
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attackhi4_v" , category = ACMD_GAME , low_priority)]
unsafe fn game_attackhi4_v(fighter: &mut L2CAgentBase) {
    smash_attacks_common!(fighter);
}



#[acmd_script( agent = "pikmin_pikmin", script = "game_attacks4" , category = ACMD_GAME , low_priority)]
unsafe fn game_attacks4(fighter: &mut L2CAgentBase) {
    smash_attacks_common!(fighter);
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attacks4_b" , category = ACMD_GAME , low_priority)]
unsafe fn game_attacks4_b(fighter: &mut L2CAgentBase) {
    smash_attacks_common!(fighter);
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attacks4_y" , category = ACMD_GAME , low_priority)]
unsafe fn game_attacks4_y(fighter: &mut L2CAgentBase) {
    smash_attacks_common!(fighter);
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attacks4_w" , category = ACMD_GAME , low_priority)]
unsafe fn game_attacks4_w(fighter: &mut L2CAgentBase) {
    smash_attacks_common!(fighter);
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attacks4_v" , category = ACMD_GAME , low_priority)]
unsafe fn game_attacks4_v(fighter: &mut L2CAgentBase) {
    smash_attacks_common!(fighter);
}




#[acmd_script( agent = "pikmin_pikmin", script = "game_attacklw4" , category = ACMD_GAME , low_priority)]
unsafe fn game_attacklw4(fighter: &mut L2CAgentBase) {
    smash_attacks_common!(fighter);
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attacklw4_b" , category = ACMD_GAME , low_priority)]
unsafe fn game_attacklw4_b(fighter: &mut L2CAgentBase) {
    smash_attacks_common!(fighter);
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attacklw4_y" , category = ACMD_GAME , low_priority)]
unsafe fn game_attacklw4_y(fighter: &mut L2CAgentBase) {
    smash_attacks_common!(fighter);
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attacklw4_w" , category = ACMD_GAME , low_priority)]
unsafe fn game_attacklw4_w(fighter: &mut L2CAgentBase) {
    smash_attacks_common!(fighter);
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attacklw4_v" , category = ACMD_GAME , low_priority)]
unsafe fn game_attacklw4_v(fighter: &mut L2CAgentBase) {
    smash_attacks_common!(fighter);
}

pub fn install() {
    install_acmd_scripts!(
        game_attackhi4,
        game_attackhi4_b,
        game_attackhi4_y,
        game_attackhi4_w,
        game_attackhi4_v,

        game_attacks4,
        game_attacks4_b,
        game_attacks4_y,
        game_attacks4_w,
        game_attacks4_v,

        game_attacklw4,
        game_attacklw4_b,
        game_attacklw4_y,
        game_attacklw4_w,
        game_attacklw4_v,
    );
}

