use super::*;
use smash_script::macros::*;
use smashline::*;
use app::{sv_system, sv_animcmd::{frame, wait}};

mod aerials;

pub fn install() {
    aerials::install();
}