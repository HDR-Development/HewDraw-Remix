use super::*;
use globals::*;
// status script import

mod attack_lw4;
mod special_n;
mod special_hi;
mod special_lw;

pub fn install(agent: &mut Agent) {
    attack_lw4::install(agent);
    special_n::install(agent);
    special_hi::install(agent);
    special_lw::install(agent);
}
