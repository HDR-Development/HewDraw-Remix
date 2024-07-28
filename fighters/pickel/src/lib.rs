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

pub const FIGHTER_PICKEL_GENERATE_ARTICLE_ENDERPEARL: i32 = 0x8;

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
    trolley::install();

    // doesnt work on steve (?)
    //smashline::clone_weapon("mario", "fireball", "pickel", "enderpearl", true);
}
