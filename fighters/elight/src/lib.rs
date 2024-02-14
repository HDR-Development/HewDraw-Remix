#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;

pub mod status;
pub mod opff;

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

pub fn install(is_runtime: bool) {
    acmd::install();
    status::install();
    opff::install(is_runtime);

    if !is_runtime || is_hdr_available() {
        status::add_statuses();
    }

    // Disables Foresight
    skyline::patching::Patch::in_text(0xa28e58).nop();
    skyline::patching::Patch::in_text(0xa28e64).data(0x140000ACu32);
}

pub fn delayed_install() {
    status::add_statuses();
}