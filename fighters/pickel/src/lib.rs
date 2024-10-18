#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;

pub mod opff;
pub mod status;

// articles

mod fire;
mod fishingrod;
mod forge;
mod melt;
mod pushobject;
mod trolley;

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

pub const WEAPON_PICKEL_TROLLEY_STATUS_KIND_PEARL_FLY: i32 = statuses::pickel_trolley::PEARL_FLY;

pub fn install() {
    let agent = &mut Agent::new("pickel");
    acmd::install(agent);
    opff::install(agent);
    status::install(agent);
    agent.install();

    fire::install();
    fishingrod::install();
    forge::install();
    melt::install();
    pushobject::install();
    trolley::install();

    // increases the amount of trolley articles that can be spawned at once
    skyline::patching::Patch::in_text(0x50118a4).data(0x2u8);
}
