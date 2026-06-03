use super::*;

mod acmd;
mod opff;

pub fn install() {
    let agent = &mut Agent::new("link_boomerang");
    acmd::install(agent);
    opff::install(agent);
    agent.install();
}