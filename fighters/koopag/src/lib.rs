#![feature(asm)]#![allow(unused_imports)]#![allow(unused_variables)]
use ::common::prelude::*;

// pub mod acmd;

//pub mod status;
pub mod opff;

use smash::app::{self, lua_bind::*};
use smash::lua2cpp::*;
use smash::lib::{*, lua_const::*};
use smash::phx::*;

use smash_script::*;

pub fn install(is_runtime: bool) {
    // acmd::install();
    //status::install();
    opff::install(is_runtime);
}