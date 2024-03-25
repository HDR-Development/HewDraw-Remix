use super::*;

mod smashes;

pub fn install(agent: &mut Agent) {
    smashes::install(agent);
}