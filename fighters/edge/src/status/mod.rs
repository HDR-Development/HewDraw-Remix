use super::*;

mod special_n;
mod special_hi;
mod edge_fire_fly;

pub fn install(agent: &mut Agent) {
    special_n::install(agent);
    special_hi::install(agent);
    edge_fire_fly::install(agent);
}