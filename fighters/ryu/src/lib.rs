#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;

pub mod opff;
pub mod status;

// articles

mod hadoken;
mod shinkuhadoken;

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
    smashline::update_weapon_count(*WEAPON_KIND_RYU_HADOKEN, 2);
    let agent = &mut Agent::new("ryu");
    acmd::install(agent);
    opff::install(agent);
    status::install(agent);
    agent.install();

    hadoken::install();
    shinkuhadoken::install();
}
