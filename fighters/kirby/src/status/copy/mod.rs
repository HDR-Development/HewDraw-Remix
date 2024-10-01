use super::*;
use globals::*;
// status script import

mod captain_special_n;
mod donkey_special_n;
mod gaogaen_special_n;
mod luigi_special_n;
mod mario_special_n;
mod mariod_special_n;
mod ridley_special_n;
mod ganon_special_n;
mod ganon_special_n_float;
mod koopa_special_n;
mod littlemac_special_n;
mod diddy_special_n_cancel;
mod lucas_special_n;
mod sonic_special_n;
mod edge_special_n;
mod bayonetta_special_n_cancel;
mod reflet_special_n;
mod palutena_special_n;
mod daisy_special_n;
// mod buddy_bayonet_end;
mod purin_special_n;

pub fn install(agent: &mut Agent) {
    captain_special_n::install(agent);
    donkey_special_n::install(agent);
    gaogaen_special_n::install(agent);
    luigi_special_n::install(agent);
    mario_special_n::install(agent);
    mariod_special_n::install(agent);
    ridley_special_n::install(agent);
    ganon_special_n::install(agent);
    ganon_special_n_float::install(agent);
    koopa_special_n::install(agent);
    littlemac_special_n::install(agent);
    diddy_special_n_cancel::install(agent);
    lucas_special_n::install(agent);
    sonic_special_n::install(agent);
    edge_special_n::install(agent);
    bayonetta_special_n_cancel::install(agent);
    reflet_special_n::install(agent);
    palutena_special_n::install(agent);
    daisy_special_n::install(agent);
    // buddy_bayonet_end::install(agent);
    purin_special_n::install(agent);
}