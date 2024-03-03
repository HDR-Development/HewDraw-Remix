use super::*;

mod donkey;
mod ganon;
mod lucario;
mod ptrainer;
mod littlemac;
mod rockman;
mod krool;
mod brave;

pub fn install() {
    donkey::install();
    ganon::install();
    lucario::install();
    ptrainer::install();
    littlemac::install();
    rockman::install();
    krool::install();
    brave::install();
}