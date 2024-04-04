use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("edge_flash");
    acmd::install(agent);
    agent.install();
}