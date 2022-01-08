use super::*;
use smash_script::macros::*;
use smashline::*;
use app::{sv_system, sv_animcmd::{frame, wait}};

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