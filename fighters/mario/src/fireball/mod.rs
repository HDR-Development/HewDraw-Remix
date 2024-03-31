use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("mario_fireball");
    acmd::install(agent);
    agent.install();
}