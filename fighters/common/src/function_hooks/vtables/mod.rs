use super::*;

mod brave;
mod donkey;
mod duckhunt;
mod gamewatch;
mod ganon;
mod gekkouga;
mod krool;
mod littlemac;
mod lucario;
mod pickel;
mod ptrainer;
mod reflet;
mod rockman;

mod miigunner_grenadelauncher;
mod ryu_shinkuhadoken;

mod weapon;

pub fn install() {
    brave::install();
    donkey::install();
    duckhunt::install();
    gamewatch::install();
    ganon::install();
    gekkouga::install();
    krool::install();
    littlemac::install();
    lucario::install();
    pickel::install();
    ptrainer::install();
    reflet::install();
    rockman::install();

    miigunner_grenadelauncher::install();
    ryu_shinkuhadoken::install();

    weapon::install();
}