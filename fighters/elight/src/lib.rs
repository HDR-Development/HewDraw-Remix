#![feature(asm)]#![allow(unused)]#![allow(non_snake_case)]

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

unsafe extern "C" fn jump_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_Jump()
}

unsafe extern "C" fn jump(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Jump()
}

unsafe extern "C" fn jump_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_Jump()
}

pub fn install(is_runtime: bool) {
    CustomStatusManager::add_new_agent_status_script(
        Hash40::new("fighter_kind_elight"),
        0,
        StatusInfo::new()
            .with_pre(jump_pre)
            .with_main(jump)
            .with_end(jump_end)
    );
    acmd::install();
    status::install();
    opff::install(is_runtime);
}