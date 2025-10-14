use super::*;

mod fly;

pub fn install(agent: &mut Agent) {
    fly::install(agent);
}