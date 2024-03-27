use super::*;
use globals::*;
// status script import

mod jump_aerial;
mod attack_lw4;
mod attack_air;
mod special_s;
mod special_n;

pub fn install(agent: &mut Agent) {
    jump_aerial::install(agent);
    attack_lw4::install(agent);
    attack_air::install(agent);
    special_s::install(agent);
    special_n::install(agent);
}
