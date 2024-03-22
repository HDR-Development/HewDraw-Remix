use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("gamewatch_rescue");
    acmd::install(agent);
    agent.install();
}