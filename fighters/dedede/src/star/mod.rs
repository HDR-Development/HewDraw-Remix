use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("dedede_star");
    acmd::install(agent);
    agent.install();
}