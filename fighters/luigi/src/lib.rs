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


extern "C" fn luigi_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        if fighter.kind() != *FIGHTER_KIND_LUIGI {
            return;
        }
    
        VarModule::off_flag(fighter.battle_object, vars::luigi::instance::IS_MISFIRE_STORED);
        VarModule::set_float(fighter.battle_object, vars::luigi::instance::MISFIRE_DAMAGE_MULTIPLIER, 1.0);
        VarModule::set_int(fighter.battle_object, vars::luigi::instance::CHARGE_SMOKE_EFFECT_HANDLE, -1);
        VarModule::set_int(fighter.battle_object, vars::luigi::instance::CHARGE_PULSE_EFFECT_HANDLE, -1);
        calculate_misfire_number(fighter);
    }
}

pub fn calculate_misfire_number(fighter: &mut L2CFighterCommon) {
    unsafe {
        let max = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "misfire.remaining_missile_max");
        let min = ParamModule::get_int(fighter.battle_object, ParamType::Agent, "misfire.remaining_missile_min");
        let range = max - min;
        let remaining = app::sv_math::rand(hash40("fighter"), range + 1);
        VarModule::set_int(
            fighter.battle_object,
            vars::luigi::instance::REMAINING_SPECIAL_S_UNTIL_MISFIRE,
            remaining + min
        );
    }
}

pub fn install() {
    status::install();
    acmd::install();
    opff::install();

    smashline::Agent::new("luigi")
        .on_start(luigi_reset)
        .install();
}
