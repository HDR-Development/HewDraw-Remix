use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("link_boomerang");
    acmd::install(agent);
    agent.install();
}