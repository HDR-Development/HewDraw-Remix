use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("diddy_barreljet");
    acmd::install(agent);
    agent.install();
}