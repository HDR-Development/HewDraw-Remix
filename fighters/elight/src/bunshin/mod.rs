use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("elight_bunshin");
    acmd::install(agent);
    agent.install();
}