use smash::app::lua_bind::*;
use smash::lua2cpp::*;
use smash::lib::{*, lua_const::*};
use smash::phx::*;
use utils::{*, consts::*, util::*};
use smash::app::*;

mod ext;
pub use ext::*;

pub mod djc;


pub fn install() {
    djc::install();
}