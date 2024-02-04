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

// cycle magic to firaga at start of match

extern "C" fn trail_init(fighter: &mut L2CFighterCommon) { 
    unsafe {
        if fighter.kind() == *FIGHTER_KIND_TRAIL {
            if !is_training_mode() {
                VarModule::off_flag(fighter.battle_object, vars::trail::instance::CYCLE_MAGIC);
                let magic_kind = WorkModule::get_int(fighter.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND);
                let trail = fighter.global_table[0x4].get_ptr() as *mut Fighter;
                if magic_kind == *FIGHTER_TRAIL_SPECIAL_N_MAGIC_KIND_FIRE {
                    WorkModule::on_flag(fighter.boma(), *FIGHTER_TRAIL_STATUS_SPECIAL_N1_FLAG_CHANGE_MAGIC);
                    FighterSpecializer_Trail::change_magic(trail); // cycles to thunder
                }
                if magic_kind == *FIGHTER_TRAIL_SPECIAL_N_MAGIC_KIND_THUNDER {
                    FighterSpecializer_Trail::change_magic(trail); // cycles to "blizzard", which is now fire
                }
            } else {
                VarModule::on_flag(fighter.battle_object, vars::trail::instance::CYCLE_MAGIC); // initial training cycle is handled through opff
            }
        }
    } 
}


pub fn install() {
    acmd::install();
    opff::install();
    status::install();
    smashline::Agent::new("trail").on_init(trail_init).install();
}
