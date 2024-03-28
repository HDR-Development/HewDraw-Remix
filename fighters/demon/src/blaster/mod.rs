use super::*;

mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("demon_blaster");
    acmd::install(agent);
    agent.install();
}