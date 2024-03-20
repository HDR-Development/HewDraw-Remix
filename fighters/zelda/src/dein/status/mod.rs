use super::*;

mod move;

pub fn install(agent: &mut Agent) {
    move::install(agent);
}