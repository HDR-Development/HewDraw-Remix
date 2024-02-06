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

/*pub unsafe extern "C" fn guard_cont_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ControlModule::get_stick_y(fighter.module_accessor) < 0.3
    && StatusModule::situation_kind(fighter.module_accessor) == SITUATION_KIND_GROUND {
        if ((ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R)
        || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L))
        && VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) != 0)
        || (ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI)
        && VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) != 1)
        || (ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW)
        && VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) != 2) {
            fighter.change_to_custom_status(statuses::packun::ADAPTIVE_ROOTS, false, false);
            return true.into();
        }
    }
    return false.into();
}*/


extern "C" fn packun_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_PACKUN {
            return;
        }
        //fighter.global_table[0x34].assign(&L2CValue::Ptr(guard_cont_pre as *const () as _));
    }
}


extern "C" fn packun_init(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[globals::FIGHTER_KIND] != FIGHTER_KIND_PACKUN {
        return;
    }

    VarModule::set_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE, 0);
}


pub fn install() {
    acmd::install();
    status::install();
    opff::install();
    smashline::Agent::new("packun")
        .on_start(packun_reset)
        .on_init(packun_init)
        .install();
}