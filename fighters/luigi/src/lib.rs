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
use utils::{
    *,
    util::*,
    ext::*,
    consts::*,
};
use smashline::*;
#[macro_use] extern crate smash_script;

pub fn calculate_misfire_number(fighter: &mut L2CFighterCommon) {
    unsafe {
        let max = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "misfire.remaining_missile_max");
        let min = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "misfire.remaining_missile_min");
        let range = max - min;
        let remaining = app::sv_math::rand(hash40("fighter"), range).clamp(min + 1, max);
        VarModule::set_int(
            fighter.battle_object,
            vars::luigi::instance::REMAINING_SPECIAL_S_UNTIL_MISFIRE,
            remaining
        );
    }
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
