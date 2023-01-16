#![deny(deprecated)]
#![allow(unused)]
#![allow(non_snake_case)]

pub mod acmd;

//pub mod status;
pub mod opff;
pub mod adaptive_roots;

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

#[status_script(agent = "packun", status = FIGHTER_STATUS_KIND_GUARD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn guard_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
    && ControlModule::get_stick_y(fighter.module_accessor) < 0.3 {
        if ((ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R)
        || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L))
        && VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) != 0)
        || (ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI)
        && VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) != 1)
        || (ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW)
        && VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) != 2) {
            StatusModule::set_status_kind_interrupt(
                fighter.module_accessor,
                CustomStatusModule::get_agent_status_kind(fighter.battle_object, statuses::packun::ADAPTIVE_ROOTS)
            );
            return 1.into();
        }
    
    }

    original!(fighter)
}

#[fighter_init]
fn packun_init(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[globals::FIGHTER_KIND] != FIGHTER_KIND_PACKUN {
        return;
    }

    VarModule::set_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE, 0);
}

pub fn install(is_runtime: bool) {
    if is_runtime {
        utils::singletons::init();
    }

    smashline::install_agent_init_callbacks!(packun_init);
    acmd::install();
    //status::install();
    opff::install(is_runtime);

    smashline::install_status_script!(guard_main);

    if !is_runtime || is_hdr_available() {
        adaptive_roots::install();
    }
}

pub fn delayed_install() {
    adaptive_roots::install();
}