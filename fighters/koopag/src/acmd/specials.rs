use smash::app::{self, lua_bind::*};
use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::phx::*;
use acmd::*;
use smash::app::utility::*;
use crate::utils::hdr;
use crate::vars::*;


pub fn install() {
    acmd::add_hooks!(
    );
}


