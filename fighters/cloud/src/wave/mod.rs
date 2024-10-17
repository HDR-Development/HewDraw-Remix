use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("cloud_wave");
    acmd::install(agent);
    agent.install();
}