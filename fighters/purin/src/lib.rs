#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;

pub mod opff;
pub mod status;

// articles

mod disarmingvoice;

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

pub const FIGHTER_PURIN_GENERATE_ARTICLE_DISARMING_VOICE: i32 = articles::purin::DISARMING_VOICE;
pub const WEAPON_PURIN_DISARMING_VOICE_STATUS_KIND_SHOOT: i32 = statuses::purin_disarming_voice::SHOOT;

pub fn install() {
    let agent = &mut Agent::new("purin");
    acmd::install(agent);
    opff::install(agent);
    status::install(agent);
    agent.install();

    disarmingvoice::install();
    smashline::clone_weapon("koopajr", *WEAPON_KIND_KOOPAJR_CANNONBALL, "purin", "disarmingvoice", false);
}
