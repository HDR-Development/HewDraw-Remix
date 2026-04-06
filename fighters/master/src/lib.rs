#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;

pub mod opff;
pub mod status;

// articles

mod axe;
mod arrow1;
mod arrow2;
//mod axethrown;

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
use smash_script::macros::ATTACK_ABS;
use utils::{
    *,
    util::*,
    ext::*,
    consts::*,
};
use smashline::*;
#[macro_use] extern crate smash_script;

#[no_mangle]
unsafe extern "C" fn master_link_event_inner(
    vtable: u64,
    fighter: &mut Fighter,
    event: &mut smash_rs::app::LinkEvent,
    original: extern "C" fn(u64, &mut Fighter, &mut smash_rs::app::LinkEvent) -> bool
) -> bool {
    let kind = event.link_event_kind.0;
    let (enable_flag, early_return) = match kind {
        0xcf409680b => (true, false),
        0x1e4c0767e5 => (false, true),
        _ => (false, false)
    };
    if enable_flag {
        let battle_object = &mut fighter.battle_object;
        let module_accessor = battle_object.module_accessor;
        let status = StatusModule::status_kind(module_accessor);
        if [
            *FIGHTER_STATUS_KIND_SPECIAL_LW,
            *FIGHTER_MASTER_STATUS_KIND_SPECIAL_LW_HIT,
        ].contains(&status) {
            VarModule::on_flag(battle_object, vars::master::status::SPECIAL_LW_ENABLE_CANCEL);
        }
    }
    if early_return {
        return false;
    }
    original(vtable, fighter, event)
}

pub fn install() {
    let agent = &mut Agent::new("master");
    acmd::install(agent);
    opff::install(agent);
    status::install(agent);
    agent.install();

    axe::install();
    arrow1::install();
    arrow2::install();

    //smashline::clone_weapon("master", "arrow1", "master", "axethrown", true);
    //axethrown::install();

    let _ = skyline::patching::Patch::in_text(0x3448d20).nop();
}