use super::*;

mod smashes;
mod specials;

pub fn install(agent: &mut Agent) {
    smashes::install(agent);
    specials::install(agent);
}