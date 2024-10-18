use super::*;
use globals::*;

mod attack_air;
mod attacklw3;
mod attacks3;
mod special_lw;
mod special_n;
mod special_s;

pub fn install(agent: &mut Agent) {
    attack_air::install(agent);
    attacklw3::install(agent);
    attacks3::install(agent);
    special_lw::install(agent);
    special_n::install(agent);
    special_s::install(agent);
}