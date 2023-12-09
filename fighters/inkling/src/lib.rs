#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;

pub mod status;
pub mod opff;

use smash::{
    app::{
        self,
        lua_bind::*,
        sv_animcmd::{frame, wait},
        *,
    },
    hash40,
    lib::lua_const::*,
    lib::{L2CValue, LuaConst},
    lua2cpp::*,
    phx::*,
};
use smash_script::{macros::*, *};
use smashline::*;
use utils::{consts::*, ext::*, util::*, *};

pub fn install(is_runtime: bool) {
    acmd::install();
    status::install();
    opff::install(is_runtime);
    skyline::install_hook!(opff::get_ink_colors);
}
