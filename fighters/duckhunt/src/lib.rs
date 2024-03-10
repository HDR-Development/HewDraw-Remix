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

extern "C" fn duckhunt_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        if fighter.kind() != *FIGHTER_KIND_DUCKHUNT {
            return;
        }
        VarModule::set_int(fighter.battle_object, vars::duckhunt::instance::GUNMAN_TIMER, 0);
    }
}

pub fn install() {
    acmd::install();
    opff::install();
    status::install();
    smashline::Agent::new("duckhunt")
        .on_start(duckhunt_reset)
        .install();
}