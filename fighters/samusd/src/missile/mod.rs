use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("samusd_missile");
    acmd::install(agent);
    agent.install();
}