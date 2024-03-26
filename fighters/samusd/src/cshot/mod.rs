use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("samusd_cshot");
    acmd::install(agent);
    agent.install();
}