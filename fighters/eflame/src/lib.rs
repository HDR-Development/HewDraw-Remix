#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;

pub mod opff;
pub mod status;

// articles

mod esword;
mod firepillar;

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
    let agent = &mut Agent::new("eflame");
    acmd::install(agent);
    opff::install(agent);
    status::install(agent);
    agent.install();

    esword::install();
    firepillar::install();

    unsafe {
        // Disables the sword catch animation unless you are completely idle.
        skyline::patching::Patch::in_text(0xa0caf4).data(0x7100001F);
    }
}