use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("krool_backpack");
    acmd::install(agent);
    agent.install();
}