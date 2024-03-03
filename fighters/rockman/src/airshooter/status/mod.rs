use super::*;

mod regular;

pub fn install(agent: &mut Agent) {
    regular::install(agent);
}