use super::*;

mod brave;
mod jack;
mod link;
mod pickel;
mod ptrainer;

pub fn install() {
    brave::install();
    jack::install();
    pickel::install();
    ptrainer::install();
}