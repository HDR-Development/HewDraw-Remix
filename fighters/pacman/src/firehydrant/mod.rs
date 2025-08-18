use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("pacman_firehydrant");
    acmd::install(agent);
    agent.install();
}