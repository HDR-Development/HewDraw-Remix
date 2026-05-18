use super::*;

mod opff;

pub fn install() {
    let agent = &mut Agent::new("simon_cross");
    opff::install(agent);
    agent.install();
}