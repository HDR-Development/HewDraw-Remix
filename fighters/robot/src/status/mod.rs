use super::*;
use globals::*;
// status script import

mod attack_air;
mod special_s;
mod special_hi;

pub fn install(agent: &mut Agent) {
    attack_air::install(agent);
    special_s::install(agent);
    special_hi::install(agent);
}
