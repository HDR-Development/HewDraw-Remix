use super::*;
use globals::*;
// status script import

mod attack_air;
mod special_hi;
mod special_n;
mod special_s;

pub fn install(agent: &mut Agent) {
    attack_air::install(agent);
    special_n::install(agent);
    special_hi::install(agent);
    special_s::install(agent);
}