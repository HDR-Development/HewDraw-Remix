use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("bayonetta_wickedweaveleg");
    acmd::install(agent);
    agent.install();
}