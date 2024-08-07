use super::*;

mod acmd;
mod status;

pub fn install() {
    let agent = &mut Agent::new("master_axe");
    acmd::install(agent);
    status::install(agent);
    agent.install();
}