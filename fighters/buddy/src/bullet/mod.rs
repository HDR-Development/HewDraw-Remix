use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("buddy_bullet");
    acmd::install(agent);
    agent.install();
}