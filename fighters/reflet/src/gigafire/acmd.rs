use super::*;

mod special;

pub fn install(agent: &mut Agent) {
    special::install(agent);
}