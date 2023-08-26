#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;

pub mod status;
pub mod opff;

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

pub const MAX_COOLDOWN : i32 = 900;
pub const LUCAS_CHARGE_TIME : i32 = 120;

pub fn install(is_runtime: bool) {
    acmd::install();
    status::install();
    opff::install(is_runtime);
    use opff::*;
    if !is_runtime {
        smashline::install_agent_frames!(
            hammer_landcancel
        );
    }

    if !is_runtime || is_hdr_available() {
        status::add_statuses();
    }
}

pub fn delayed_install() {
    status::add_statuses();
}