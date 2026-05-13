#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;

pub mod opff;
pub mod status;

// articles
mod fireball;
mod obakyumu;

use smash::{
    lib::{
        L2CValue,
        LuaConst,
    },
    app::{
        *,
        self,
        sv_animcmd::{
            frame,
            wait
        },
        lua_bind::*
    },
    hash40,
    lib::{
        lua_const::*
    },
    lua2cpp::*,
    phx::*
};
use smash_script::{
    *,
    macros::*
};
use smash_script::macros::ATTACK_ABS;
use utils::{
    *,
    util::*,
    ext::*,
    consts::*,
};
use smashline::*;
#[macro_use] extern crate smash_script;

pub unsafe fn reset_misfire_queue(fighter: &mut L2CFighterCommon) {
    let denominator = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "misfire.denominator");
    let position = app::sv_math::rand(hash40("fighter"), denominator);
    VarModule::set_int(fighter.battle_object, vars::luigi::instance::SPECIAL_S_MISFIRE_COUNT, position); 
}

pub unsafe fn calculate_misfire(fighter: &mut L2CFighterCommon) -> bool {
    let misfire_count = VarModule::get_int(fighter.battle_object, vars::luigi::instance::SPECIAL_S_MISFIRE_COUNT);
    if misfire_count <= 0 {
        reset_misfire_queue(fighter);
        return true;
    }
    VarModule::dec_int(fighter.battle_object, vars::luigi::instance::SPECIAL_S_MISFIRE_COUNT);
    return false;
}

pub fn install() {
    let agent = &mut Agent::new("luigi");
    acmd::install(agent);
    opff::install(agent);
    status::install(agent);
    agent.install();

    fireball::install();
    obakyumu::install();
}