use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("szerosuit_whip");
    acmd::install(agent);
    agent.install();
}