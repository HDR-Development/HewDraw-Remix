use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("wolf_blaster_bullet");
    acmd::install(agent);
    agent.install();
}