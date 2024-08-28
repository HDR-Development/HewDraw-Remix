use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("brave_tornado");
    acmd::install(agent);
    agent.install();
}