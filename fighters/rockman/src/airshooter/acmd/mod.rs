use super::*;

mod aerials;

pub fn install(agent: &mut Agent) {
    aerials::install(agent);
}