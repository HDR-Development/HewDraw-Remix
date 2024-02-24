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

pub fn install() {
    acmd::install();
    status::install();
    opff::install();

    // Disables Foresight
    skyline::patching::Patch::in_text(0xa28e78).nop();
    skyline::patching::Patch::in_text(0xa28e84).data(0x140000ACu32);
}
