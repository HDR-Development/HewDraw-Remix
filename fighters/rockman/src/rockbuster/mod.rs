use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("rockman_rockbuster");
    acmd::install(agent);
    agent.install();
}