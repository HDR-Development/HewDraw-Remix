#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;

pub mod status;
pub mod opff;

// articles

mod fire;
mod forge;
mod melt;
mod trolley;

// material table hook

pub mod material_table;

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
    let agent = &mut Agent::new("pickel");
    acmd::install(agent);
    opff::install(agent);
    status::install(agent);
    agent.install();

    fire::install();
    forge::install();
    melt::install();
    trolley::install();

    material_table::install();
}
