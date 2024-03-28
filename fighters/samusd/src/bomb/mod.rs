use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("samusd_bomb");
    acmd::install(agent);
    agent.install();
}