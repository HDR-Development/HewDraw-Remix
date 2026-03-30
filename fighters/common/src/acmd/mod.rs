use super::*;

mod damage;
mod swordswing;

pub fn install() {
    damage::install();
    swordswing::install();
}