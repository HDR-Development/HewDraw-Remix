use smash::{
    lua2cpp::{
        L2CFighterCommon,
        L2CAgentBase
    },
    app::{
        BattleObject,
        BattleObjectModuleAccessor,
        lua_bind::*
    },
    lib::{
        L2CValue,
        lua_const::*
    }
};
use utils::{
    *,
    ext::*,
    consts::*,
    util::*
};

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
    pub use smash::app::{self, {BattleObject, BattleObjectModuleAccessor}, lua_bind::*};
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
    pub use smash::app::{self, lua_bind::*, utility::*};
    pub use smash::lua2cpp::*;
    pub use smash::lib::{*, lua_const::*};
    pub use smash::phx::*;
    pub use smash_script::macros::*;
    pub use smashline::*;
    pub use smash::hash40;
    pub use app::{sv_system, sv_animcmd::{frame, wait}};
    pub use smash::app::sv_battle_object::notify_event_msc_cmd;
    pub use utils::{VarModule, ParamModule, BufferModule, MeterModule};
    pub use utils::consts::globals::*;
}

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
        std::mem::transmute(utils::util::get_battle_object_from_accessor(boma))
    }

    unsafe fn get_boma(_: &mut Self) -> &'static mut BattleObjectModuleAccessor {
        panic!("Calling GetObjects::get_boma() on a BattleObjectModuleAccessor is invalid")
    }
}

pub trait AgentUtil {
    unsafe fn is_cat_flag<T: Into<CommandCat>>(&mut self, fighter_pad_cmd_flag: T) -> bool;
    unsafe fn is_cat_flag_all<T: Into<CommandCat>>(&mut self, fighter_pad_cmd_flag: T) -> bool;
    unsafe fn is_button_on(&mut self, buttons: Buttons) -> bool;
    unsafe fn is_button_off(&mut self, buttons: Buttons) -> bool;
    unsafe fn is_button_trigger(&mut self, buttons: Buttons) -> bool;
    unsafe fn is_button_release(&mut self, buttons: Buttons) -> bool;
    unsafe fn was_prev_button_on(&mut self, buttons: Buttons) -> bool;
    unsafe fn was_prev_button_off(&mut self, buttons: Buttons) -> bool;
    unsafe fn is_status(&mut self, kind: i32) -> bool;
    unsafe fn is_status_one_of(&mut self, kinds: &[i32]) -> bool;
    unsafe fn is_prev_status(&mut self, kind: i32) -> bool;
    unsafe fn is_prev_status_one_of(&mut self, kinds: &[i32]) -> bool;
    unsafe fn is_situation(&mut self, kind: i32) -> bool;
    unsafe fn is_prev_situation(&mut self, kind: i32) -> bool;
    unsafe fn is_motion(&mut self, motion: Hash40) -> bool;
    unsafe fn is_motion_one_of(&mut self, motions: &[Hash40]) -> bool;
    unsafe fn is_fighter(&mut self) -> bool;
    unsafe fn is_weapon(&mut self) -> bool;
    unsafe fn kind(&mut self) -> i32;
}

impl AgentUtil for L2CAgentBase {
    unsafe fn is_cat_flag<T: Into<CommandCat>>(&mut self, fighter_pad_cmd_flag: T) -> bool {
        return self.boma().is_cat_flag(fighter_pad_cmd_flag);
    }

    unsafe fn is_cat_flag_all<T: Into<CommandCat>>(&mut self, fighter_pad_cmd_flag: T) -> bool {
        return self.boma().is_cat_flag_all(fighter_pad_cmd_flag);
    }

    unsafe fn is_button_on(&mut self, buttons: Buttons) -> bool {
        return self.boma().is_button_on(buttons);
    }

    unsafe fn is_button_off(&mut self, buttons: Buttons) -> bool {
        return self.boma().is_button_off(buttons);
    }

    unsafe fn is_button_trigger(&mut self, buttons: Buttons) -> bool {
        return self.boma().is_button_trigger(buttons);
    }

    unsafe fn is_button_release(&mut self, buttons: Buttons) -> bool {
        return self.boma().is_button_release(buttons);
    }

    unsafe fn was_prev_button_on(&mut self, buttons: Buttons) -> bool {
        return self.boma().was_prev_button_on(buttons);
    }

    unsafe fn was_prev_button_off(&mut self, buttons: Buttons) -> bool {
        return self.boma().was_prev_button_off(buttons);
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

    unsafe fn is_prev_status_one_of(&mut self, kinds: &[i32]) -> bool {
        return self.boma().is_prev_status_one_of(kinds);
    }

    unsafe fn is_situation(&mut self, kind: i32) -> bool {
        return self.boma().is_situation(kind);
    }

    unsafe fn is_prev_situation(&mut self, kind: i32) -> bool {
        return self.boma().is_prev_situation(kind);
    }

    unsafe fn is_motion(&mut self, kind: Hash40) -> bool {
        return self.boma().is_motion(kind);
    }

    unsafe fn is_motion_one_of(&mut self, kind: Hash40) -> bool {
        return self.boma().is_motion_one_of(kind);
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
    unsafe fn is_cat_flag<T: Into<CommandCat>>(&mut self, fighter_pad_cmd_flag: T) -> bool {
        let cat = fighter_pad_cmd_flag.into();
        match cat {
            CommandCat::Cat1(cat) => Cat1::new(self).intersects(cat),
            CommandCat::Cat2(cat) => Cat2::new(self).intersects(cat),
            CommandCat::Cat3(cat) => Cat3::new(self).intersects(cat),
            CommandCat::Cat4(cat) => Cat4::new(self).intersects(cat)
        }
    }

    unsafe fn is_cat_flag_all<T: Into<CommandCat>>(&mut self, fighter_pad_cmd_flag: T) -> bool {
        let cat = fighter_pad_cmd_flag.into();
        match cat {
            CommandCat::Cat1(cat) => Cat1::new(self).contains(cat),
            CommandCat::Cat2(cat) => Cat2::new(self).contains(cat),
            CommandCat::Cat3(cat) => Cat3::new(self).contains(cat),
            CommandCat::Cat4(cat) => Cat4::new(self).contains(cat)
        }
    }

    unsafe fn is_button_on(&mut self, buttons: Buttons) -> bool {
        Buttons::from_bits_unchecked(ControlModule::get_button(self)).intersects(buttons)
    }

    unsafe fn is_button_off(&mut self, buttons: Buttons) -> bool {
        !self.is_button_on(buttons)
    }

    unsafe fn is_button_trigger(&mut self, buttons: Buttons) -> bool {
        Buttons::from_bits_unchecked(ControlModule::get_trigger(self)).intersects(buttons)
    }

    unsafe fn is_button_release(&mut self, buttons: Buttons) -> bool {
        Buttons::from_bits_unchecked(ControlModule::get_release(self)).intersects(buttons)
    }

    unsafe fn was_prev_button_on(&mut self, buttons: Buttons) -> bool {
        Buttons::from_bits_unchecked(ControlModule::get_button_prev(self)).intersects(buttons)
    }

    unsafe fn was_prev_button_off(&mut self, buttons: Buttons) -> bool {
        !self.was_prev_button_on(buttons)
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

    unsafe fn is_prev_status_one_of(&mut self, kinds: &[i32]) -> bool {
        let kind = StatusModule::prev_status_kind(self, 0);
        return kinds.contains(&kind);
    }

    unsafe fn is_situation(&mut self, kind: i32) -> bool {
        return StatusModule::situation_kind(self) == kind;
    }

    unsafe fn is_prev_situation(&mut self, kind: i32) -> bool {
        return StatusModule::prev_situation_kind(self) == kind;
    }

    unsafe fn is_motion(&mut self, kind: Hash40) -> bool {
        return MotionModule::motion_kind(self) == kind;
    }

    unsafe fn is_motion_one_of(&mut self, kinds: &[Hash40]) -> bool {
        let kind = MotionModule::motion_kind(self);
        return kinds.contains(&kind);
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