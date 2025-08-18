use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("samusd_gbeam");
    acmd::install(agent);
    agent.install();
}