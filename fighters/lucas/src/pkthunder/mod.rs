use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("lucas_pkthunder");
    acmd::install(agent);
    agent.install();
}