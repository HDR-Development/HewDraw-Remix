use super::*;
use smash_script::macros::*;
use smashline::*;
use app::{sv_system, sv_animcmd::{frame, wait}};
use ::common::prelude::smash::app::sv_battle_object::notify_event_msc_cmd;

mod aerials;
mod ground;
mod other;
mod smashes;
mod specials;
mod tilts;

pub fn install() {
    aerials::install();
    ground::install();
    other::install();
    smashes::install();
    specials::install();
    tilts::install();
}