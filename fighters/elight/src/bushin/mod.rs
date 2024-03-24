use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("elight_bushin");
    acmd::install(agent);
    agent.install();
}