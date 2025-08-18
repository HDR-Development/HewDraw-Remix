use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("inkling_squid");
    acmd::install(agent);
    agent.install();
}