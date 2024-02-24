#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;

pub mod status;
pub mod opff;
mod vtable_hook;
pub mod vl;

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
            wait,
            execute
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
    acmd::install();
    opff::install();
    status::install();
    vtable_hook::install();
}
