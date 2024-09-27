#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;

pub mod opff;
pub mod status;

// articles
mod fireball;
mod obakyumu;

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
    lib::{
        lua_const::*
    },
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
#[macro_use] extern crate smash_script;

pub fn calculate_misfire_number(fighter: &mut L2CFighterCommon) {
    unsafe {
        let remaining_max = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "misfire.remaining_missile_max");
        let remaining_min = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "misfire.remaining_missile_min");
        let damage_min = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "misfire.remaining_scale_damage_min");
        let damage_max = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "misfire.remaining_scale_damage_max");
        let damage = DamageModule::damage(fighter.module_accessor, 0);
        let lerp = (remaining_max - remaining_min) as f32 / (damage_max - damage_min);
        let dec_remain = damage * lerp;
        let rand_max = remaining_max - (dec_remain as i32).clamp(0, remaining_max - remaining_min);
        let rand = app::sv_math::rand(hash40("fighter"), rand_max);
        VarModule::set_int(fighter.battle_object, vars::luigi::instance::SPECIAL_S_REMAINING_COUNT, rand + 1);
    }
}

pub fn install() {
    let agent = &mut Agent::new("luigi");
    acmd::install(agent);
    opff::install(agent);
    status::install(agent);
    agent.install();

    fireball::install();
    obakyumu::install();
}