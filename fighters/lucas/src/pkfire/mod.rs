use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("lucas_pkfire");
    acmd::install(agent);
    agent.install();
}