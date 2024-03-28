use super::*;
use globals::*;
utils::import!(common::djc::attack_air_main_status);
// status script import

mod attack_air;
mod special_hi;

pub fn install(agent: &mut Agent) {
    attack_air::install(agent);
    special_hi::install(agent);
}