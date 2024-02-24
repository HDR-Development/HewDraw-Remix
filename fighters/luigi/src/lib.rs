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

#[smashline::fighter_reset]
fn luigi_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        if fighter.kind() != *FIGHTER_KIND_LUIGI {
            return;
        }
    
        VarModule::off_flag(fighter.battle_object, vars::luigi::instance::IS_MISFIRE_STORED);
        VarModule::set_float(fighter.battle_object, vars::luigi::instance::MISFIRE_DAMAGE_MULTIPLIER, 1.0);
        VarModule::set_int(fighter.battle_object, vars::luigi::instance::CHARGE_SMOKE_EFFECT_HANDLE, -1);
        VarModule::set_int(fighter.battle_object, vars::luigi::instance::CHARGE_PULSE_EFFECT_HANDLE, -1);
    }
}

pub fn calculate_misfire_number(fighter: &mut L2CFighterCommon) {
    unsafe {
        let max = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "misfire.remaining_missile_max");
        let min = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "misfire.remaining_missile_min");
        let range = max - min;
        let remaining = app::sv_math::rand(hash40("fighter"), range).clamp(min + 1, max);
        VarModule::set_int(
            fighter.battle_object,
            vars::luigi::instance::REMAINING_SPECIAL_S_UNTIL_MISFIRE,
            remaining
        );
    }
}

pub fn install(is_runtime: bool) {
    acmd::install();
    status::install();
    opff::install(is_runtime);
    smashline::install_agent_resets!(luigi_reset);
}