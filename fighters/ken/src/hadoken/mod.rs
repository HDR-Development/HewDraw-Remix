use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("ken_hadoken");
    acmd::install(agent);
    agent.install();
}