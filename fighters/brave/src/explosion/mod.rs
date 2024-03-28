use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("brave_explosion");
    acmd::install(agent);
    agent.install();
}