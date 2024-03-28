use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("master_arrow1");
    acmd::install(agent);
    agent.install();
}