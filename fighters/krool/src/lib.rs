#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;

pub mod opff;
pub mod status;

// articles

mod backpack;
mod ironball;

pub mod vtable_hook;
pub use status::krool_belly_damage_hook_impl;

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
    let agent = &mut Agent::new("krool");
    acmd::install(agent);
    opff::install(agent);
    status::install(agent);
    agent.install();

    backpack::install();
    ironball::install();

    // prevents shield break on belly
    use opff::*;
    skyline::patching::Patch::in_text(0xc04f00).data(0x1400001Eu32);
}