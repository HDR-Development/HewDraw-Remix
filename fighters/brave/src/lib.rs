#![feature(asm)] // #![allow(unused_imports)]#![allow(unused_variables)]
use smash::{
    app::{
        self,
        sv_animcmd::{
            frame,
            wait
        },
        lua_bind::*
    },
    lib::lua_const::*,
    lua2cpp::*,
    phx::*,
    hash40
};
use smash_script::{
    *,
    macros::*
};
use utils::{
    *,
    consts::*,
    ext::*
};
use smashline::*;

pub mod acmd;

//pub mod status;
pub mod opff;

pub fn install(is_runtime: bool) {
    acmd::install();
    //status::install();
    opff::install(is_runtime);
}