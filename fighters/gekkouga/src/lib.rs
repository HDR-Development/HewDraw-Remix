#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;

//pub mod status;
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

#[smashline::fighter_reset]
fn substitute_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        if fighter.kind() != *FIGHTER_KIND_GEKKOUGA {
            return;
        }
        VarModule::set_int(fighter.battle_object, vars::gekkouga::instance::SUBSTITUTE_TIMER, 0);
    }
}

pub fn install(is_runtime: bool) {
    acmd::install();
    //status::install();
    opff::install(is_runtime);
    use opff::*;
    smashline::install_agent_resets!(substitute_reset);
    smashline::install_raw_hook!(notify_log_event_collision_hit_replace);
}