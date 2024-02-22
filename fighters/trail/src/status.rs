use super::*;
use globals::*;
// status script import

mod jump_aerial;

mod attack_lw4;

mod attack_air;

mod special_s;

mod special_n;

pub fn install() {
    jump_aerial::install();

    attack_lw4::install();

    attack_air::install();

    special_s::install();

    special_n::install();
}
