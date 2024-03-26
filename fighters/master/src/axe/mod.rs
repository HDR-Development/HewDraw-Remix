use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("master_axe");
    acmd::install(agent);
    agent.install();
}