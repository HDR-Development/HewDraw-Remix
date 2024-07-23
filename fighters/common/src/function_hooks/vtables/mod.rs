use super::*;

mod donkey;
mod ganon;
mod lucario;
mod ptrainer;
mod littlemac;
mod reflet;
mod rockman;
mod krool;
mod brave;

mod ryu_shinkuhadoken;

pub fn install() {
    donkey::install();
    ganon::install();
    lucario::install();
    ptrainer::install();
    littlemac::install();
    reflet::install();
    rockman::install();
    krool::install();
    brave::install();

    ryu_shinkuhadoken::install();
}