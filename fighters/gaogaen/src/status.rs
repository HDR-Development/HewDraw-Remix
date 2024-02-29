use super::*;
use globals::*;

mod attack_lw4;
mod special_n;

pub fn install() {
    attack_lw4::install();
    special_n::install();
}
