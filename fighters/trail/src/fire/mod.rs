use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("trail_fire");
    acmd::install(agent);
    agent.install();
}