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

extern "C" fn reflet_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_REFLET {
            return;
        }
    }
}

extern "C" fn reflet_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        if fighter.global_table[globals::FIGHTER_KIND] != FIGHTER_KIND_REFLET {
        return;
        }
        WorkModule::set_int(&mut *fighter.module_accessor, 8, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT);
    }
}

pub fn install() {
    acmd::install();
    status::install();
    opff::install();
    smashline::Agent::new("reflet")
        .on_start(reflet_reset)
        .on_start(reflet_init)
        .install();
}
