use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("pit_arrow");
    acmd::install(agent);
    agent.install();
}