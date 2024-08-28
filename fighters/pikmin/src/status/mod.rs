use super::*;
use globals::*;
// status script import

mod escape_air;
mod attack_air;
mod rebirth;
mod special_n;

pub fn install(agent: &mut Agent) {
    escape_air::install(agent);
    attack_air::install(agent);
    rebirth::install(agent);
    special_n::install(agent);
}
