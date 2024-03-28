#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;

pub mod opff;
pub mod status;

// articles

mod bunshin;

use skyline::nro::NroInfo;
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
    let agent = &mut Agent::new("elight");
    acmd::install(agent);
    opff::install(agent);
    status::install(agent);
    agent.install();

    bunshin::install();

    // Disables Foresight
    skyline::patching::Patch::in_text(0xa28e78).nop();
    skyline::patching::Patch::in_text(0xa28e84).data(0x140000ACu32);
}
