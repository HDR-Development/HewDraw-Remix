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

#[smashline::fighter_init]
fn shizue_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        VarModule::off_flag(fighter.object(), vars::shizue::instance::LLOID_ASYNC);
        VarModule::set_int(fighter.object(), vars::shizue::instance::LLOID_TIMER, 0);
    }
}

pub fn install(is_runtime: bool) {
    acmd::install();
    status::install();
    opff::install(is_runtime);
    use opff::*;
    smashline::install_agent_init_callbacks!(shizue_init);
    smashline::install_agent_frame_callback!(fishingrod_callback);
    smashline::install_agent_frame_callback!(lloid_callback);
}