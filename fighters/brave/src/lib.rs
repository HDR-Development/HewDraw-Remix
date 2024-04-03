#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;

pub mod opff;
//pub mod status;

// articles

mod crash;
mod deathball;
mod explosion;
mod fireball;
mod lightning;
mod spark;
mod tornado;

pub mod menu;
pub use menu::hero_rng_hook_impl;

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
    consts::*,
    ext::*
};
use smashline::*;
#[macro_use] extern crate smash_script;

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    // init roll history
    VarModule::set_int(fighter.battle_object, vars::brave::instance::SPELL_SLOT_USED_1_1, -1);
    VarModule::set_int(fighter.battle_object, vars::brave::instance::SPELL_SLOT_USED_1_2, -1);
    VarModule::set_int(fighter.battle_object, vars::brave::instance::SPELL_SLOT_USED_1_3, -1);
    VarModule::set_int(fighter.battle_object, vars::brave::instance::SPELL_SLOT_USED_1_4, -1);
    VarModule::set_int(fighter.battle_object, vars::brave::instance::SPELL_SLOT_USED_2_1, -1);
    VarModule::set_int(fighter.battle_object, vars::brave::instance::SPELL_SLOT_USED_2_2, -1);
    VarModule::set_int(fighter.battle_object, vars::brave::instance::SPELL_SLOT_USED_2_3, -1);
    VarModule::set_int(fighter.battle_object, vars::brave::instance::SPELL_SLOT_USED_2_4, -1);

    // roll to get two sets of fresh values
    let mut vals = vec![];
    menu::roll_spells(fighter, &mut vals);
    menu::roll_spells(fighter, &mut vals);

    VarModule::off_flag(fighter.battle_object, vars::brave::instance::PSYCHE_UP_ACTIVE);
    VarModule::set_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 0);
}

pub fn install() {
    let agent = &mut Agent::new("brave");
    acmd::install(agent);
    opff::install(agent);
    agent.on_start(on_start);
    agent.install();

    crash::install();
    deathball::install();
    explosion::install();
    fireball::install();
    lightning::install();
    spark::install();
    tornado::install();
}