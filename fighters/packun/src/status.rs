use super::*;
use globals::*;

mod special_hi;
mod special_s;
mod attack_s4;

pub fn install() {
    special_s::install();
    special_hi::install();
    attack_s4::install();
}