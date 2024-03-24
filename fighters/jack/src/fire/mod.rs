use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("jack");
    acmd::install(agent);
    agent.install();
}