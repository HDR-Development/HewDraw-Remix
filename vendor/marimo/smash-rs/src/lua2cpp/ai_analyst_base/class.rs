use std::ops::{Deref, DerefMut};

use crate::*;

use super::cpp::*;

#[repr(C)]
pub struct L2CFighterAIAnalystBase {
    ai_base: lua2cpp::L2CFighterAIBase,
    pub global_table: lib::L2CValue,
    pub private_table: lib::L2CValue,
}

impl Deref for L2CFighterAIAnalystBase {
    type Target = lua2cpp::L2CFighterAIBase;

    fn deref(&self) -> &Self::Target {
        &self.ai_base
    }
}

impl DerefMut for L2CFighterAIAnalystBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ai_base
    }
}

impl L2CFighterAIAnalystBase {
    pub fn main_func__analyst_global_variables(&mut self) {
        unsafe {
            main_func__analyst_global_variables(self)
        }
    }

    pub fn main_func__analyst_private_variables(&mut self) {
        unsafe {
            main_func__analyst_private_variables(self)
        }
    }

    pub fn change_status(&mut self, arg1: lib::L2CValue) {
        unsafe {
            change_status(self, arg1.into())
        }
    }

    pub fn update_status(&mut self) {
        unsafe {
            update_status(self)
        }
    }

    pub fn update_chanced_frame(&mut self) {
        unsafe {
            update_chanced_frame(self)
        }
    }

    pub fn ENTRY(&mut self) -> lib::L2CValue {
        unsafe {
            ENTRY(self).into()
        }
    }

    pub fn init_lines(&mut self) {
        unsafe {
            init_lines(self)
        }
    }

    pub fn init_events(&mut self) {
        unsafe {
            init_events(self)
        }
    }

    pub fn init_global_variables(&mut self) {
        unsafe {
            init_global_variables(self)
        }
    }

    pub fn init_private_variables(&mut self) {
        unsafe {
            init_private_variables(self)
        }
    }

    pub fn call_function_update(&mut self) -> lib::L2CValue {
        unsafe {
            call_function_update(self).into()
        }
    }

    pub fn call_event_on_dead(&mut self) -> lib::L2CValue {
        unsafe {
            call_event_on_dead(self).into()
        }
    }
}

#[cfg(feature = "type_assert")]
impl L2CFighterAIAnalystBase {
    pub fn assert() {
        assert_eq!(size_of!(L2CFighterAIAnalystBase), 0x178);
        assert_eq!(offset_of!(L2CFighterAIAnalystBase, ai_base), 0x0);
        assert_eq!(offset_of!(L2CFighterAIAnalystBase, global_table), 0x158);
        assert_eq!(offset_of!(L2CFighterAIAnalystBase, private_table), 0x168);
    }
}