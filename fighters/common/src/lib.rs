pub mod prelude {
    pub use smash;
    pub use skyline;
    pub use smash_script;
    pub use smash_script::*;
    pub use smashline;
    pub use utils::{self, *, ext::*, consts::*, util::*};
    pub use super::StatusShift;
    pub use super::AgentUtil;
    pub use super::GetObjects;
}

pub mod acmd_import {
    pub use super::prelude::*;
    pub use smash::app::{self, *, lua_bind::*};
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


pub trait AgentUtil {
    unsafe fn is_cat_flag(&mut self, category: i32, fighter_pad_cmd_flag: i32) -> bool;
    unsafe fn is_status(&mut self, kind: i32) -> bool;
    unsafe fn is_status_one_of(&mut self, kinds: &[i32]) -> bool;
    unsafe fn is_prev_status(&mut self, kind: i32) -> bool;
    unsafe fn is_fighter(&mut self) -> bool;
    unsafe fn is_weapon(&mut self) -> bool;
    unsafe fn kind(&mut self) -> i32;
}

impl AgentUtil for L2CAgentBase {
    unsafe fn is_cat_flag(&mut self, category: i32, fighter_pad_cmd_flag: i32) -> bool {
        return self.boma().is_cat_flag(category, fighter_pad_cmd_flag);
    }

    unsafe fn is_status(&mut self, kind: i32) -> bool {
        return self.boma().is_status(kind);
    }

    unsafe fn is_status_one_of(&mut self, kinds: &[i32]) -> bool {
        return self.boma().is_status_one_of(kinds);
    }

    unsafe fn is_prev_status(&mut self, kind: i32) -> bool {
        return self.boma().is_prev_status(kind);
    }

    unsafe fn is_fighter(&mut self) -> bool {
        return self.boma().is_fighter();
    }

    unsafe fn is_weapon(&mut self) -> bool {
        return self.boma().is_weapon();
    }

    unsafe fn kind(&mut self) -> i32 {
        return self.boma().kind();
    }
}

impl AgentUtil for BattleObjectModuleAccessor {
    unsafe fn is_cat_flag(&mut self, category: i32, fighter_pad_cmd_flag: i32) -> bool {
        let flag_mask = ControlModule::get_command_flag_cat(self, category);
        return compare_mask(flag_mask, fighter_pad_cmd_flag);
    }

    unsafe fn is_status(&mut self, kind: i32) -> bool {
        return StatusModule::status_kind(self) == kind;
    }

    unsafe fn is_status_one_of(&mut self, kinds: &[i32]) -> bool {
        let kind = StatusModule::status_kind(self);
        return kinds.contains(&kind);
    }

    unsafe fn is_prev_status(&mut self, kind: i32) -> bool {
        return StatusModule::prev_status_kind(self, 0) == kind;
    }

    unsafe fn is_fighter(&mut self) -> bool {
        return smash::app::utility::get_category(self) == *BATTLE_OBJECT_CATEGORY_FIGHTER;
    }

    unsafe fn is_weapon(&mut self) -> bool {
        return smash::app::utility::get_category(self) == *BATTLE_OBJECT_CATEGORY_WEAPON;
    }

    unsafe fn kind(&mut self) -> i32 {
        return smash::app::utility::get_kind(self);
    }
}


pub fn install() {
    djc::install();
}