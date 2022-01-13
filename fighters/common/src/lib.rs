#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_parens)]
use smash::app::lua_bind::*;
use smash::lua2cpp::*;
use smash::lib::{*, lua_const::*};
use smash::phx::*;
use utils::{*, consts::*, util::*};
use smash::app::*;

use utils::ext::*;

#[macro_use] extern crate smash_script;

pub mod djc;
pub mod opff;
pub mod misc;

pub fn install() {
    djc::install();
}