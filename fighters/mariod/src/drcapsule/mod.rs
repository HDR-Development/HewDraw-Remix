use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("mariod_drcapsule");
    acmd::install(agent);
    agent.install();
}