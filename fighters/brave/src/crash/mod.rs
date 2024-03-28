use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("brave_crash");
    acmd::install(agent);
    agent.install();
}