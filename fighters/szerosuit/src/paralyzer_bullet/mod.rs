use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("szerosuit_paralyzer_bullet");
    acmd::install(agent);
    agent.install();
}