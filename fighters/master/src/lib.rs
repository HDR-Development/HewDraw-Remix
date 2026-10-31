#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;

pub mod opff;
pub mod status;

mod vtable;

// articles

mod axe;
mod arrow1;
mod arrow2;
//mod axethrown;

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
    let agent = &mut Agent::new("master");
    acmd::install(agent);
    opff::install(agent);
    status::install(agent);
    agent.install();

    vtable::install();

    axe::install();
    arrow1::install();
    arrow2::install();

    //smashline::clone_weapon("master", "arrow1", "master", "axethrown", true);
    //axethrown::install();

    let _ = skyline::patching::Patch::in_text(0x34492d0).nop();
}