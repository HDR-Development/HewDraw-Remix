use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("falco_blaster");
    acmd::install(agent);
    agent.install();
}