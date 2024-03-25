use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("jack_fire");
    acmd::install(agent);
    agent.install();
}