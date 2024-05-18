use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("pacman_trampoline");
    acmd::install(agent);
    agent.install();
}