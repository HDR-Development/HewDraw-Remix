use common::prelude::*;

pub mod status;

use smash::app::{self, lua_bind::*};
use smash::lua2cpp::*;
use smash::lib::{*, lua_const::*};
use smash::phx::*;

use smash_script::*;

pub fn install() {
    status::install();
}