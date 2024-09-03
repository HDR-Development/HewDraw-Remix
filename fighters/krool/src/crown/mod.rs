use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("krool_crown");
    acmd::install(agent);
    agent.install();
}