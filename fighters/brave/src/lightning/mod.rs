use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("brave_lightning");
    acmd::install(agent);
    agent.install();
}