use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("bayonetta");
    acmd::install(agent);
    agent.install();
}