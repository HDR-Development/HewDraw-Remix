use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("luigi_obakyumu");
    acmd::install(agent);
    agent.install();
}