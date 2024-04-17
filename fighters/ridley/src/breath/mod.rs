use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("ridley_breath");
    acmd::install(agent);
    agent.install();
}