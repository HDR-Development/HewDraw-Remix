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

pub fn install(is_runtime: bool) {
    acmd::install();
    status::install();
    opff::install(is_runtime);

    // NOPs nana's change_status call (inner impl) to the FIGHTER_POPO_STATUS_KIND_THROW_NANA status when buffered
    skyline::patching::Patch::in_text(0xfb6414).nop();

    // NOPs nana's change_motion call (inner impl) to the throw animations (her cheer animations) status when buffered
    skyline::patching::Patch::in_text(0xfb6520).nop();

    // NOPs nana's change_status call (inner impl) to the FIGHTER_POPO_STATUS_KIND_THROW_NANA status status when not buffered
    skyline::patching::Patch::in_text(0xfba3c8).nop();

    // NOPs nana's change_motion call (inner impl) to the throw animations (her cheer animations) status when not buffered
    skyline::patching::Patch::in_text(0xfbb8b0).nop();
}
