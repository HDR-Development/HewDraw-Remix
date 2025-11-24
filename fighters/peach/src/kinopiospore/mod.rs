use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("peach_kinopiospore");
    acmd::install(agent);
    agent.install();
}