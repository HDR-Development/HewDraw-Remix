#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;

pub mod opff;
pub mod status;

// articles

mod autoaimbullet;
mod explosiveflame;
mod reflectionboard;
mod meteor;

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
use smash_script::macros::ATTACK_ABS;
use utils::{
    *,
    util::*,
    ext::*,
    consts::*,
};
use smashline::*;
#[macro_use] extern crate smash_script;

pub fn install() {
    let agent = &mut Agent::new("palutena");
    acmd::install(agent);
    opff::install(agent);
    status::install(agent);
    agent.install();

    autoaimbullet::install();
    explosiveflame::install();
    reflectionboard::install();
    meteor::install();

    unsafe {
        let meteor = smashline::clone_weapon("palutena", "meteor", *WEAPON_KIND_RYU_HADOKEN, false);
        articles::generate::palutena::METEOR = meteor.generate_id;
        articles::kind::palutena::METEOR = meteor.weapon_id;
    }
}