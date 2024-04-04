use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("simon_axe");
    acmd::install(agent);
    agent.install();
}