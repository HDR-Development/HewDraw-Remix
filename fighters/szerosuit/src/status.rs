use super::*;

mod rebirth;

pub fn install(agent: &mut Agent) {
    rebirth::install(agent);
}
