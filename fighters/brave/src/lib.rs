#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

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

pub mod acmd;

pub mod menu;
pub mod opff;
pub use menu::hero_rng_hook_impl;

extern "C" fn brave_init(fighter: &mut L2CFighterCommon) {
    unsafe {
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
}

pub fn install(_: bool) {
    acmd::install();
    opff::install();
    smashline::Agent::new("brave").on_init(brave_init).install();
}