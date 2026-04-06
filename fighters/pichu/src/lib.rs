#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;

pub mod opff;
pub mod status;

// articles

mod dengekidama;
mod kaminari;

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

pub unsafe fn PICHU_ADD_DAMAGE(agent: &mut L2CAgentBase, damage: f32) {
    // catch kirby
    if agent.kind() != *FIGHTER_KIND_PICHU {
        return FT_ADD_DAMAGE(agent, damage);
    }

    // Add recoil damage
    let recoil_mul = VarModule::get_float(agent.battle_object, vars::pichu::instance::CHARGE_STATE_RECOIL_MUL);
    FT_ADD_DAMAGE(agent, damage * recoil_mul);

    // Add meter progress
    if !VarModule::is_flag(agent.battle_object, vars::pichu::instance::CHARGE_STATE_ENABLED) {
        MeterModule::add(agent.battle_object, damage);
    }
}

pub fn install() {
    let agent = &mut Agent::new("pichu");
    acmd::install(agent);
    opff::install(agent);
    status::install(agent);
    agent.install();

    dengekidama::install();
    kaminari::install();
}
