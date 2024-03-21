use super::*;

mod acmd;
mod opff;

pub fn install(agent: &mut Agent) {
    let agent = &mut Agent::new("kirby");
    acmd::install(agent);
    opff::install();
    agent.install();
}