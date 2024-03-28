use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("mario_pumpwater");
    acmd::install(agent);
    agent.install();
}