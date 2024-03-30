use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("mario_pump");
    acmd::install(agent);
    agent.install();
}