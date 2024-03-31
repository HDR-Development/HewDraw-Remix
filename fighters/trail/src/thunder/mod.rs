use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("trail_thunder");
    acmd::install(agent);
    agent.install();
}