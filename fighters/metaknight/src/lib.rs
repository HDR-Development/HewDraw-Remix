#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;

//pub mod status;
pub mod opff;
pub mod meta_quick;

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

#[status_script(agent = "metaknight", status = FIGHTER_STATUS_KIND_APPEAL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn appeal_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_cat_flag(Cat2::AppealLw)
    && MeterModule::level(fighter.battle_object) >= 10
    {
        MeterModule::drain(fighter.battle_object, 10);
        meta_quick::start_meta_quick(fighter, 8 * 60);
        StatusModule::set_status_kind_interrupt(
            fighter.module_accessor,
            CustomStatusModule::get_agent_status_kind(fighter.battle_object, statuses::metaknight::METAQUICK_SUMMON)
        );
        return 1.into();
    }

    original!(fighter)
}

#[fighter_init]
fn mk_init(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[globals::FIGHTER_KIND] != FIGHTER_KIND_METAKNIGHT {
        return;
    }

    VarModule::set_int(fighter.battle_object, vars::metaknight::instance::META_QUICK_CHARGE_EFFECT_HANDLE, -1);
    VarModule::set_int(fighter.battle_object, vars::metaknight::instance::META_QUICK_EFFECT_HANDLE, -1);
    VarModule::set_int(fighter.battle_object, vars::metaknight::instance::META_QUICK_EFFECT_HANDLE2, -1);
    VarModule::set_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 0);
    MeterModule::reset(fighter.battle_object);
}

pub fn install(is_runtime: bool) {
    if is_runtime {
        utils::singletons::init();
    }

    smashline::install_agent_init_callbacks!(mk_init);
    acmd::install();
    //status::install();
    opff::install(is_runtime);

    smashline::install_status_script!(appeal_pre);

    if !is_runtime || is_hdr_available() {
        meta_quick::install();
    }
}

pub fn delayed_install() {
    meta_quick::install();
}