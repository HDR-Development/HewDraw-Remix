use super::*;

mod specials;

pub fn install(agent: &mut Agent) {
    specials::install(agent);
}