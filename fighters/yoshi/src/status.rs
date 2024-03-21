use super::*;
use globals::*;
utils::import!(common::djc::attack_air_main_status);

mod guard_on;
mod guard_off;
mod guard_damage;
mod jump_aerial;
mod attack_air;

pub fn install(agent: &mut Agent) {
    guard_on::install(agent);
    guard_off::install(agent);
    guard_damage::install(agent);
    jump_aerial::install(agent);
    attack_air::install(agent);
}
