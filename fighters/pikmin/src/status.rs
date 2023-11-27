use super::*;
mod escape_air;
mod attack_air;

pub fn install() {
    escape_air::install();
    attack_air::install();
}