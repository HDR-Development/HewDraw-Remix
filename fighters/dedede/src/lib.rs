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

#[fighter_init]
fn dedede_init(fighter: &mut L2CFighterCommon){
    if fighter.global_table[globals::FIGHTER_KIND] != FIGHTER_KIND_DEDEDE{
        return;
    }

    VarModule::set_int(fighter.battle_object, vars::dedede::instance::RECATCH_COUNTER, 0);
}

pub fn install(is_runtime: bool) {
    smashline::install_agent_init_callbacks!(dedede_init);
    acmd::install();
    status::install();
    opff::install(is_runtime);
}