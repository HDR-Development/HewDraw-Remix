#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;

pub mod opff;
pub mod status;

// articles

mod chakram;
mod lightshuriken;
mod tornado;

use smash::{
    lib::{
        L2CValue,
        LuaConst,
        L2CAgent,
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

pub fn install() {
    let agent = &mut Agent::new("miiswordsman");
    acmd::install(agent);
    opff::install(agent);
    status::install(agent);
    agent.install(); 

    chakram::install();
    lightshuriken::install();
    tornado::install();
}