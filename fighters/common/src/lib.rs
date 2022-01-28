#![allow(unused)]
#![allow(non_snake_case)]
use smash::app::lua_bind::*;
use smash::lua2cpp::*;
use smash::lib::{*, lua_const::*};
use smash::phx::*;
use smash::app::*;
use smash::app;
use smash::hash40;

use utils::{
    *,
    util::*,
    ext::*,
    consts::*,
};
use smashline::*;

#[macro_use] extern crate smash_script;

pub mod djc;
pub mod opff;
pub mod misc;
pub mod general_statuses;
pub mod function_hooks;

pub fn install() {
    djc::install();
    misc::install();
    general_statuses::install();
    function_hooks::install();
}