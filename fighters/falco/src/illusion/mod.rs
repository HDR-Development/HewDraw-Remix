use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("falco_illusion");
    acmd::install(agent);
    agent.install();
}