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

pub fn calculate_misfire(fighter: &mut L2CFighterCommon) -> i32 {
    unsafe {
        let divisor_scale_min = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "misfire.divisor_scale_min");
        let divisor_scale_max = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "misfire.divisor_scale_max");
        let damage_scale_min = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "misfire.damage_scale_min");
        let damage_scale_max = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "misfire.damage_scale_max");
        let damage = DamageModule::damage(fighter.module_accessor, 0);
        let lerp = (divisor_scale_max - divisor_scale_min) as f32 / (damage_scale_max - damage_scale_min);
        let damage_lerp = (damage * lerp) as i32;
        let calc_misfire = (divisor_scale_max - damage_lerp).clamp(divisor_scale_min, divisor_scale_max);
        let rand = app::sv_math::rand(hash40("fighter"), calc_misfire);
        return rand;
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