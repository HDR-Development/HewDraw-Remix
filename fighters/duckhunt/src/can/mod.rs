use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("duckhunt_can");
    acmd::install(agent);
    agent.install();
}