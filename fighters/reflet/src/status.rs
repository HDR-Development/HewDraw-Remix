use super::*;

mod attack_air;

mod special_n;

pub fn install() {
    attack_air::install();

    special_n::install();
}
