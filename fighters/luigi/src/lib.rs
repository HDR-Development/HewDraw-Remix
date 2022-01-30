#![feature(asm)]#![allow(unused)]#![allow(non_snake_case)]

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

pub const REMAINING_SPECIAL_S_UNTIL_MISFIRE: i32 = 0x1000;
pub const CHARGE_SMOKE_EFFECT_HANDLE: i32 = 0x1001;
pub const CHARGE_PULSE_EFFECT_HANDLE: i32 = 0x1002;

pub const IS_MISFIRE_STORED: i32 = 0x1000;
pub const MISFIRE_DAMAGE_RATIO: i32 = 0x1000;

#[smashline::fighter_reset]
fn luigi_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        if fighter.kind() != *FIGHTER_KIND_LUIGI {
            return;
        }
    
        VarModule::off_flag(fighter.battle_object, IS_MISFIRE_STORED);
        VarModule::set_float(fighter.battle_object, MISFIRE_DAMAGE_RATIO, 1.0);
        VarModule::set_int(fighter.battle_object, CHARGE_SMOKE_EFFECT_HANDLE, -1);
        VarModule::set_int(fighter.battle_object, CHARGE_PULSE_EFFECT_HANDLE, -1);
        calculate_misfire_number(fighter);
    }
}

pub fn calculate_misfire_number(fighter: &mut L2CFighterCommon) {
    unsafe {
        VarModule::set_int(fighter.battle_object, REMAINING_SPECIAL_S_UNTIL_MISFIRE, app::sv_math::rand(hash40("fighter"), 8))
    }
}

pub fn install(is_runtime: bool) {
    acmd::install();
    status::install();
    opff::install(is_runtime);
    smashline::install_agent_resets!(luigi_reset);
}