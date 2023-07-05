use super::*;
use smashline::*;

mod attack_air;
mod jump_aerial;

pub fn install() {
    attack_air::install();
    jump_aerial::install();
}