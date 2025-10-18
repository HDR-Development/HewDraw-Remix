use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("toonlink_boomerang");
    acmd::install(agent);
    agent.install();
}