#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;

pub mod opff;
pub mod status;

// articles

mod spikeball;
mod poisonbreath;
mod firebreath;

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

pub const FIGHTER_PACKUN_GENERATE_ARTICLE_FIREBREATH: i32 = articles::packun::FIREBREATH;
pub const WEAPON_PACKUN_FIREBREATH_STATUS_KIND_REGULAR: i32 = statuses::packun_firebreath::REGULAR;

pub fn install() {
    let agent = &mut Agent::new("packun");
    acmd::install(agent);
    opff::install(agent);
    status::install(agent);
    agent.install();

    spikeball::install();
    poisonbreath::install();
    firebreath::install();

    smashline::clone_weapon("mario", "fireball", "packun", "firebreath", false);
}