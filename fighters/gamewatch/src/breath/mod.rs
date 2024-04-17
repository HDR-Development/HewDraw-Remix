use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("gamewatch_breath");
    acmd::install(agent);
    agent.install();
}