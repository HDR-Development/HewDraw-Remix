use super::*;

mod tilts;

pub fn install(agent: &mut Agent) {
    tilts::install(agent);
}