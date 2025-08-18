use super::*;
use globals::*;
// status script import

mod attack_air;
mod attacklw3;
mod attacks3;
mod special_lw;

pub fn install(agent: &mut Agent) {
    attack_air::install(agent);
    attacklw3::install(agent);
    attacks3::install(agent);
    special_lw::install(agent);
}