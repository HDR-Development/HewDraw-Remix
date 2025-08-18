use super::*;

mod jack;
mod pickel;
mod ptrainer;

pub fn install() {
    jack::install();
    pickel::install();
    ptrainer::install();
}