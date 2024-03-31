use super::*;
use globals::*;
// status script import

mod escape_air;
mod attack_air;
mod rebirth;

pub fn install(agent: &mut Agent) {
    escape_air::install(agent);
    attack_air::install(agent);
    rebirth::install(agent);
}
