use super::*;

mod diddy;
mod edge;
mod falco;
mod ganon;
mod koopa;
mod koopajr;
mod krool;
mod littlemac;
mod lucas;
mod luigi;
mod mario;
mod mariod;
mod miigunner;
mod palutena;
mod richter;
mod ridley;
mod roy;
mod shizue;
mod sonic;
mod wolf;

pub fn install(agent: &mut Agent) {
    diddy::install(agent);
    edge::install(agent);
    falco::install(agent);
    ganon::install(agent);
    koopa::install(agent);
    koopajr::install(agent);
    krool::install(agent);
    littlemac::install(agent);
    lucas::install(agent);
    luigi::install(agent);
    mario::install(agent);
    mariod::install(agent);
    miigunner::install(agent);
    palutena::install(agent);
    richter::install(agent);
    ridley::install(agent);
    roy::install(agent);
    shizue::install(agent);
    sonic::install(agent);
    wolf::install(agent);
}
