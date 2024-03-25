#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;

pub mod opff;
pub mod status;

// articles

mod bullet;
mod pad;

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

static mut BAYONET_EGGS:[i32;8] = [0; 8]; //I have no idea why varmod doesn't work with this, so this will have to do

pub fn install() {
    let agent = &mut Agent::new("buddy");
    acmd::install(agent);
    opff::install(agent);
    status::install(agent);
    agent.install();

    bullet::install();
    pad::install();
}