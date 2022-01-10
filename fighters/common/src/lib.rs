use smash::app::lua_bind::*;
use smash::lua2cpp::*;
use smash::lib::{*, lua_const::*};
use smash::phx::*;
use utils::{*, consts::*, util::*};
use smash::app::*;

pub use common_dyn::ext::*;

pub mod djc;
pub mod opff;


pub fn install() {
    djc::install();
}