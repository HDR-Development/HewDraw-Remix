use super::*;
use globals::*;
// status script import

mod attack_hi4;
mod special_hi;
mod special_lw;
mod special_s;

pub fn install(agent: &mut Agent) {
    attack_hi4::install(agent);
    special_hi::install(agent);
    special_lw::install(agent);
    special_s::install(agent);
}