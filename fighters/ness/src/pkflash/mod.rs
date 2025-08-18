use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("ness_pkflash");
    acmd::install(agent);
    agent.install();
}