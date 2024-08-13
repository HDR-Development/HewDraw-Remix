#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]
// use ::common::prelude::*;

pub mod acmd;

pub mod opff;
pub mod status;

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
    consts::*,
    ext::*
};
use smashline::*;
#[macro_use] extern crate smash_script;

pub fn install() {
    let agent = &mut Agent::new("captain");
    acmd::install(agent);
    opff::install(agent);
    status::install(agent);
    agent.install();
}