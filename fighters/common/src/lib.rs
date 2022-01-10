#![feature(asm)]#![allow(unused_imports)]#![allow(unused_variables)]
pub mod prelude {
    pub use smash;
    pub use skyline;
    pub use smash_script;
    pub use smash_script::*;
    pub use smashline;
    pub use utils::{self, *, ext::*, consts::*, util::*};
    pub use super::StatusShift;
    pub use super::InputCheck;
    pub use super::GetObjects;
}

pub mod acmd_import {
    pub use super::prelude::*;
    pub use smash::app::{self, lua_bind::*};
    pub use smash::lua2cpp::*;
    pub use smash::lib::{*, lua_const::*};
    pub use smash::phx::*;
    pub use smash_script::macros::*;
    pub use smashline::*;
    pub use smash::hash40;
    pub use app::{sv_system, sv_animcmd::{frame, wait}};
    pub use smash::app::sv_battle_object::notify_event_msc_cmd;
    pub use utils::{VarModule, ParamModule, BufferModule, MeterModule};
}

pub mod opff_import {
    pub use super::prelude::*;
    pub use smash::app::{self, lua_bind::*};
    pub use smash::lua2cpp::*;
    pub use smash::lib::{*, lua_const::*};
    pub use smash::phx::*;
    pub use smash_script::macros::*;
    pub use smashline::*;
    pub use smash::hash40;
    pub use app::{sv_system, sv_animcmd::{frame, wait}};
    pub use smash::app::sv_battle_object::notify_event_msc_cmd;
    pub use utils::{VarModule, ParamModule, BufferModule, MeterModule};
}

use smash::app::lua_bind::*;
use smash::lua2cpp::*;
use smash::lib::{*, lua_const::*};
use smash::phx::*;
use utils::{*, consts::*, util::*};
use smash::app::*;

pub mod djc;
pub mod opff;

pub trait StatusShift {
    unsafe fn main_shift(&mut self, new_main: unsafe extern "C" fn(&mut L2CFighterCommon) -> L2CValue) -> L2CValue;
    unsafe fn fast_shift(&mut self, new_main: unsafe extern "C" fn(&mut L2CFighterCommon) -> L2CValue) -> L2CValue;
}

impl StatusShift for L2CFighterCommon {
    unsafe fn main_shift(&mut self, new_main: unsafe extern "C" fn(&mut L2CFighterCommon) -> L2CValue) -> L2CValue {
        self.sub_shift_status_main(L2CValue::Ptr(new_main as *const () as _))
    }

    unsafe fn fast_shift(&mut self, new_main: unsafe extern "C" fn(&mut L2CFighterCommon) -> L2CValue) -> L2CValue {
        self.fastshift(L2CValue::Ptr(new_main as *const () as _))
    }
}

pub trait GetObjects {
    unsafe fn get_object(agent: &mut Self) -> &'static mut BattleObject;
    unsafe fn get_boma(agent: &mut Self) -> &'static mut BattleObjectModuleAccessor;
    
    unsafe fn object(&mut self) -> &'static mut BattleObject {
        Self::get_object(self)
    }

    unsafe fn boma(&mut self) -> &'static mut BattleObjectModuleAccessor {
        Self::get_boma(self)
    }
}

impl GetObjects for L2CAgentBase {
    unsafe fn get_object(agent: &mut Self) -> &'static mut BattleObject {
        std::mem::transmute(agent.battle_object)
    }

    unsafe fn get_boma(agent: &mut Self) -> &'static mut BattleObjectModuleAccessor {
        std::mem::transmute(agent.module_accessor)
    }
}

impl GetObjects for BattleObjectModuleAccessor {
    unsafe fn get_object(boma: &mut Self) -> &'static mut BattleObject {
        return get_battle_object_from_module_accessor(boma)
    }

    unsafe fn get_boma(boma: &mut Self) -> &'static mut BattleObjectModuleAccessor {
        return boma;
    }
}


pub trait InputCheck {
    unsafe fn is_cat_flag(&mut self, category: i32, fighter_pad_cmd_flag: i32) -> bool;
}

impl InputCheck for L2CAgentBase {
    unsafe fn  is_cat_flag(&mut self, category: i32, fighter_pad_cmd_flag: i32) -> bool {
        return self.boma().is_cat_flag(category, fighter_pad_cmd_flag);
    }
}

impl InputCheck for BattleObjectModuleAccessor {
    unsafe fn  is_cat_flag(&mut self, category: i32, fighter_pad_cmd_flag: i32) -> bool {
        let flag_mask = ControlModule::get_command_flag_cat(self, category);
        return compare_mask(flag_mask, fighter_pad_cmd_flag);
    }
}



pub fn install() {
    djc::install();
}