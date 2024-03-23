use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("miiswordfighter_tornado");
    acmd::install(agent);
    agent.install();
}