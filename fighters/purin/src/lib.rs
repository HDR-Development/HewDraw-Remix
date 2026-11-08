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
    let agent = &mut Agent::new("purin");
    acmd::install(agent);
    opff::install(agent);
    status::install(agent);
    agent.install();

    disarmingvoice::install();

    unsafe {
        let disarmingvoice = smashline::clone_weapon("purin", "disarmingvoice", *WEAPON_KIND_KOOPAJR_CANNONBALL, false);
        articles::generate::purin::DISARMING_VOICE = disarmingvoice.generate_id;
        articles::kind::purin::DISARMING_VOICE = disarmingvoice.weapon_id;
    }
}
