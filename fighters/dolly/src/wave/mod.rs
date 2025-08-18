use super::*;

mod acmd;

pub fn install() {
    let agent = &mut smashline::Agent::new("dolly_wave");
    acmd::install(agent);
    agent.install();
}