pub mod prelude {
    pub use smash;
    pub use skyline;
    pub use smash_script;
    pub use smashline;
    pub use utils::{self, *, consts::*};
    pub use super::StatusShift;
}

use smash::app::lua_bind::*;
use smash::lua2cpp::*;
use smash::lib::{*, lua_const::*};
use smash::phx::*;
use utils::{*, consts::*};

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

pub fn install() {
    djc::install();
}