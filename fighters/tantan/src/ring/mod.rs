use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("tantan_ring");
    acmd::install(agent);
    agent.install();
}