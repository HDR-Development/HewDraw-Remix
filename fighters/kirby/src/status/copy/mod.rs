use super::*;
use globals::*;
// status script import

mod bayonetta_special_n_cancel;
// mod buddy_bayonet_end;
mod captain_special_n;
mod daisy_special_n;
mod diddy_special_n_cancel;
mod donkey_special_n;
mod edge_special_n;
mod ganon_special_n;
mod ganon_special_n_float;
mod gaogaen_special_n;
mod koopa_special_n;
mod littlemac_special_n;
mod lucas_special_n;
mod luigi_special_n;
mod mario_special_n;
mod mariod_special_n;
mod miigunner_special_n;
mod pacman_special_n;
mod palutena_special_n;
mod purin_special_n;
mod reflet_special_n;
mod ridley_special_n;
mod sonic_special_n;

pub fn install(agent: &mut Agent) {
    bayonetta_special_n_cancel::install(agent);
    // buddy_bayonet_end::install(agent);
    captain_special_n::install(agent);
    daisy_special_n::install(agent);
    diddy_special_n_cancel::install(agent);
    donkey_special_n::install(agent);
    edge_special_n::install(agent);
    ganon_special_n::install(agent);
    ganon_special_n_float::install(agent);
    gaogaen_special_n::install(agent);
    koopa_special_n::install(agent);
    littlemac_special_n::install(agent);
    lucas_special_n::install(agent);
    luigi_special_n::install(agent);
    mario_special_n::install(agent);
    mariod_special_n::install(agent);
    miigunner_special_n::install(agent);
    pacman_special_n::install(agent);
    palutena_special_n::install(agent);
    purin_special_n::install(agent);
    reflet_special_n::install(agent);
    ridley_special_n::install(agent);
    sonic_special_n::install(agent);
}