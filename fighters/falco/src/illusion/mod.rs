use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("fa");
    acmd::install(agent);
    agent.install();
}