use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("samus_gbeam");
    acmd::install(agent);
    agent.install();
}