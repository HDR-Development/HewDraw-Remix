#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;
pub mod status;
pub mod opff;
mod vtable_hook;

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

pub fn install(is_runtime: bool) {
    acmd::install();
    status::install();
    opff::install(is_runtime);
    vtable_hook::install();

    if !is_runtime || is_hdr_available() {
        status::add_statuses();
    }
}

pub fn delayed_install() {
    status::add_statuses();
}