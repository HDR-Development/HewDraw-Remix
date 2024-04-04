use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("krool_ironball");
    acmd::install(agent);
    agent.install();
}