use super::*;

mod bayonetta;
mod brave;
mod demon;
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
mod shotos;
mod shulk;

mod miigunner_grenadelauncher;
mod ryu_shinkuhadoken;
mod dolly_burst;

mod weapon;

pub fn install() {
    bayonetta::install();
    brave::install();
    demon::install();
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
    shotos::install();
    shulk::install();

    miigunner_grenadelauncher::install();
    ryu_shinkuhadoken::install();
    dolly_burst::install();

    weapon::install();
}
