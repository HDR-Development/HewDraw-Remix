use super::*;

mod jack;
mod pickel;

pub fn install() {
    jack::install();
    pickel::install();
}