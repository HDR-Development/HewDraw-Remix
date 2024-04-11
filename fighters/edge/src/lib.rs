#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;

pub mod opff;
pub mod status;

// articles

mod fire;
mod flare1;
mod flare2;
mod flaredummy;
mod flash;

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
    lib::lua_const::*,
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

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    VarModule::set_int(fighter.battle_object, vars::edge::instance::FIRE_ID, -1);
}

pub fn install() {
    let agent = &mut Agent::new("edge");
    acmd::install(agent);
    opff::install(agent);
    status::install(agent);
    agent.on_start(on_start);
    agent.install();

    fire::install();
    flare1::install();
    flare2::install();
    flaredummy::install();
    flash::install();
}