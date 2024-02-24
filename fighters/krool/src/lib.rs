#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

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
pub mod acmd;
pub mod status;
pub mod opff;
pub mod vtable_hook;
pub use status::krool_belly_damage_hook_impl;

pub fn install() {
    acmd::install();
    status::install();
    opff::install();
    use opff::*;

    // prevents shield break on belly
    skyline::patching::Patch::in_text(0xc04f00).data(0x1400001Eu32);
}