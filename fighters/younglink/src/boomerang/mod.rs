use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("younglink_boomerang");
    acmd::install(agent);
    agent.install();
}