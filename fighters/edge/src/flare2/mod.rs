use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("edge_flare2");
    acmd::install(agent);
    agent.install();
}