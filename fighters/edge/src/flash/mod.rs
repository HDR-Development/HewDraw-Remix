use super::*;

mod acmd;
mod status;

pub fn install() {
    let agent = &mut Agent::new("edge_flash");
    acmd::install(agent);
    status::install(agent);
    agent.install();
}