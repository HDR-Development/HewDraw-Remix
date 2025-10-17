use super::*;

mod brave;
mod donkey;
mod duckhunt;
mod edge;
mod gamewatch;
mod ganon;
mod gekkouga;
mod ike;
mod krool;
mod littlemac;
mod lucario;
mod master;
mod pickel;
mod ptrainer;
mod reflet;
mod rockman;

mod miigunner_grenadelauncher;
mod ryu_shinkuhadoken;
mod dolly_burst;

mod weapon;

pub fn install() {
    brave::install();
    donkey::install();
    duckhunt::install();
    edge::install();
    gamewatch::install();
    ganon::install();
    gekkouga::install();
    ike::install();
    krool::install();
    littlemac::install();
    lucario::install();
    master::install();
    pickel::install();
    ptrainer::install();
    reflet::install();
    rockman::install();

    miigunner_grenadelauncher::install();
    ryu_shinkuhadoken::install();
    dolly_burst::install();

    weapon::install();
}