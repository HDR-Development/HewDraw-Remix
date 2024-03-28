#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;

pub mod opff;
pub mod status;

// articles

mod bowlingball;
mod bullet;
mod clayrocket;
mod firework;
mod flowerpot;
mod slingshot;

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

pub fn install() {
    let agent = &mut Agent::new("murabito");
    acmd::install(agent);
    opff::install(agent);
    status::install(agent);
    agent.install();

    bowlingball::install();
    bullet::install();
    clayrocket::install();
    firework::install();
    flowerpot::install();
    slingshot::install();
}