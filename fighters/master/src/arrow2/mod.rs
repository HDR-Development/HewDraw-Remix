use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("master_arrow2");
    acmd::install(agent);
    agent.install();
}