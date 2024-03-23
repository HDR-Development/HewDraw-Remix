use super::*;

mod special_s;
mod attack_100;

pub fn install(agent: &mut Agent) {
    special_s::install(agent);
    attack_100::install(agent);
}